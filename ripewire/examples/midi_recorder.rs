//! Print MIDI events received on the input port, similar to `pw-mididump`
//!
//! NOTE that this is still WIP code dump, of dubious quality

use std::{
    collections::HashMap,
    io,
    os::fd::{AsFd, AsRawFd, FromRawFd, OwnedFd, RawFd},
    path::PathBuf,
    ptr::NonNull,
    time::Duration,
};

use libspa_consts::{
    abi_unstable::{PwNodeActivationStatus, PW_VERSION_NODE_ACTIVATION},
    SpaChoiceType, SpaChunk, SpaControlType, SpaDataType, SpaDirection, SpaEnum, SpaFormat,
    SpaIoAsyncBuffers, SpaIoBuffers, SpaIoPosition, SpaIoType, SpaMediaSubtype, SpaMediaType,
    SpaNodeBuffersFlags, SpaNodeCommand, SpaParamIo, SpaParamType, SpaStatus, SpaType,
};
use ripewire::{
    connection::MessageBuffer,
    context::Context,
    memory_registry::{MemMap, MemoryRegistry},
    node_activation::{eventfd_read, monotonic_ns, Activation, Target},
    protocol::{
        pw_client_node::{
            self,
            methods::{
                NodeFlags, NodeInfo, NodeInfoChangeMask, PortFlags, PortInfo, PortInfoChangeMask,
                PortUpdate, PortUpdateChangeMask, Update, UpdateChangeMask,
            },
        },
        pw_core, ParamFlags, ParamInfo, PwDictionary,
    },
    proxy::{PwClientNode, PwCore},
    HashMapExt,
};

const NODE_NAME: &str = "midi-recorder";
const PORT_ID: u32 = 0;

struct BufferData {
    chunk: NonNull<SpaChunk>,
    data: NonNull<u8>,
    maxsize: u32,
    _map: Option<MemMap>,
}

struct Buffer {
    datas: Vec<BufferData>,
    _map: MemMap,
}

#[derive(Default)]
struct Mix {
    io: Option<MemMap>,
    buffers: Vec<Buffer>,
}

struct State {
    mems: MemoryRegistry,

    activation: Option<Activation>,
    position: Option<MemMap>,
    readfd: Option<OwnedFd>,
    _writefd: Option<OwnedFd>,
    targets: Vec<Target>,

    mixes: HashMap<u32, Mix>,
    format: Option<pod::serialize::OwnedPod>,
    running: bool,

    clock_start_position: Option<u64>,
}

fn owned_fd(fd: Option<RawFd>) -> Option<OwnedFd> {
    fd.map(|fd| unsafe { OwnedFd::from_raw_fd(fd) })
}

fn port_update(format: Option<&pod::serialize::OwnedPod>) -> PortUpdate {
    let enum_format = pod::Builder::with(|b| {
        b.write_object_with(
            SpaType::ObjectFormat,
            SpaParamType::EnumFormat as u32,
            |b| {
                b.write_property(SpaFormat::MediaType as u32, 0, |b| {
                    b.write_id(SpaMediaType::Application as u32);
                });
                b.write_property(SpaFormat::MediaSubtype as u32, 0, |b| {
                    b.write_id(SpaMediaSubtype::Control as u32);
                });
                b.write_property(SpaFormat::ControlTypes as u32, 0, |b| {
                    b.write_choice_with(SpaChoiceType::Flags, 0, |b| {
                        b.write_u32(1 << SpaControlType::Midi as u32);
                    });
                });
            },
        );
    });

    let io = |id: SpaIoType| {
        pod::Builder::with(|b| {
            b.write_object_with(SpaType::ObjectParamIo, SpaParamType::Io as u32, |b| {
                b.write_property(SpaParamIo::Id as u32, 0, |b| {
                    b.write_id(id as u32);
                });
                b.write_property(SpaParamIo::Size as u32, 0, |b| {
                    b.write_u32(std::mem::size_of::<SpaIoBuffers>() as u32);
                });
            });
        })
    };

    let mut params = vec![
        enum_format,
        io(SpaIoType::Buffers),
        io(SpaIoType::AsyncBuffers),
    ];
    params.extend(format.cloned());

    let param_info = |id: SpaParamType, flags: ParamFlags| ParamInfo {
        id: id.into(),
        flags,
    };

    PortUpdate {
        direction: SpaEnum::Value(SpaDirection::Input),
        port_id: PORT_ID,
        change_mask: PortUpdateChangeMask::PARAMS | PortUpdateChangeMask::INFO,
        params,
        info: Some(PortInfo {
            change_mask: PortInfoChangeMask::FLAGS
                | PortInfoChangeMask::RATE
                | PortInfoChangeMask::PROPS
                | PortInfoChangeMask::PARAMS,
            flags: PortFlags::empty(),
            rate_num: 0,
            rate_denom: 1,
            items: PwDictionary::from_dict([
                ("format.dsp", "8 bit raw midi"),
                ("port.name", "input"),
                ("port.id", "0"),
                ("port.direction", "in"),
            ]),
            params: vec![
                param_info(SpaParamType::EnumFormat, ParamFlags::READ),
                param_info(SpaParamType::Meta, ParamFlags::empty()),
                param_info(SpaParamType::Io, ParamFlags::READ),
                param_info(
                    SpaParamType::Format,
                    if format.is_some() {
                        ParamFlags::READWRITE
                    } else {
                        ParamFlags::WRITE
                    },
                ),
                param_info(SpaParamType::Buffers, ParamFlags::empty()),
                param_info(SpaParamType::Latency, ParamFlags::WRITE),
                param_info(SpaParamType::Tag, ParamFlags::WRITE),
            ],
        }),
    }
}

impl State {
    fn core_event(&mut self, ctx: &mut Context<Self>, core: PwCore, event: pw_core::Event) {
        match event {
            pw_core::Event::AddMem(add_mem) => self.mems.add_mem(&add_mem),
            pw_core::Event::RemoveMem(remove_mem) => self.mems.remove_mem(&remove_mem),
            pw_core::Event::Ping(ping) => core.pong(ctx, ping.id, ping.seq),
            pw_core::Event::Error(error) => eprintln!("{error:?}"),
            _ => {}
        }
    }

    fn client_node_event(
        &mut self,
        ctx: &mut Context<Self>,
        node: PwClientNode,
        event: pw_client_node::Event,
    ) {
        use pw_client_node::Event;

        match event {
            Event::Transport(msg) => {
                let activation = self
                    .mems
                    .map(msg.memid, msg.offset, msg.size)
                    .map(Activation::new)
                    .unwrap()
                    .expect("Incompatible version of activation record");

                activation.set_client_version(PW_VERSION_NODE_ACTIVATION);

                self.activation = Some(activation);
                self.update_active_driver();
                self.readfd = owned_fd(msg.readfd.fd);
                self._writefd = owned_fd(msg.writefd.fd);

                node.set_active(ctx, true);
            }
            Event::SetActivation(msg) => {
                self.targets.retain(|t| t.node_id() != msg.node_id);

                if let Ok(map) = self.mems.map(msg.memid, msg.offset, msg.size) {
                    let activation = Activation::new(map).unwrap();
                    let signalfd = owned_fd(msg.signalfd.fd).unwrap();
                    self.targets
                        .push(Target::new(msg.node_id, activation, signalfd));
                } else {
                    // Invalid memid is used to remove the target
                }

                eprintln!("targets: {:?}", self.target_ids());
            }
            Event::SetIo(msg) => {
                if msg.id == SpaEnum::Value(SpaIoType::Position) {
                    // Invalid memid is used to unset the io
                    self.position = self.mems.map(msg.memid, msg.offset, msg.size).ok();
                    self.update_active_driver();
                }
            }
            Event::PortSetMixInfo(msg) => {
                if msg.peer_id == u32::MAX {
                    // Invalid peer id is used to remove the mix
                    self.mixes.remove(&msg.mix_id);
                } else {
                    self.mixes.entry(msg.mix_id).or_default();
                }
                eprintln!("mix {}: peer {}", msg.mix_id, msg.peer_id as i32);
            }
            Event::PortSetIo(msg) => {
                let is_buffers = matches!(
                    msg.id,
                    SpaEnum::Value(SpaIoType::Buffers | SpaIoType::AsyncBuffers)
                );

                if is_buffers {
                    // Invalid memid is used to unset the io
                    let io = self.mems.map(msg.memid, msg.offset, msg.size).ok();
                    self.mixes.entry(msg.mix_id).or_default().io = io;
                }
            }
            Event::PortSetParam(msg) => {
                if msg.id != SpaEnum::Value(SpaParamType::Format) {
                    return;
                }

                let format = msg.param.as_deserializer();
                self.format = (!format.is_none()).then(|| msg.param.to_serialize());
                eprintln!(
                    "format: {}",
                    if self.format.is_some() {
                        "set"
                    } else {
                        "unset"
                    }
                );

                for mix in self.mixes.values_mut() {
                    mix.buffers.clear();
                }

                node.port_update(ctx, port_update(self.format.as_ref()));
            }
            Event::PortUseBuffers(msg) => {
                let flags = SpaNodeBuffersFlags::from_bits_retain(msg.flags);
                assert!(
                    !flags.contains(SpaNodeBuffersFlags::ALLOC),
                    "SpaNodeBuffersFlags::ALLOC not supported"
                );

                let buffers = msg.buffers.iter().map(|b| self.map_buffer(b)).collect();
                self.mixes.entry(msg.mix_id).or_default().buffers = buffers;

                eprintln!("mix {}: {} buffers", msg.mix_id as i32, msg.buffers.len());
            }
            Event::Command(msg) => {
                let command = msg.command.as_deserializer().as_object().unwrap();
                let command = SpaEnum::<SpaNodeCommand>::from_raw(command.object_id());
                eprintln!("command: {command:?}");

                match command {
                    SpaEnum::Value(SpaNodeCommand::Start) => self.start(),
                    SpaEnum::Value(SpaNodeCommand::Pause | SpaNodeCommand::Suspend) => self.stop(),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn position(&self) -> Option<SpaIoPosition> {
        let position = self.position.as_ref()?.ptr_at::<SpaIoPosition>(0)?;
        Some(unsafe { position.read_volatile() })
    }

    /// Driver skips us until we acknowledge that we are using its position
    fn update_active_driver(&self) {
        if let (Some(activation), Some(position)) = (&self.activation, self.position()) {
            activation.set_active_driver_id(position.clock.id);
            eprintln!("driver: {}", position.clock.id);
        }
    }

    fn target_ids(&self) -> Vec<u32> {
        self.targets.iter().map(Target::node_id).collect()
    }

    fn map_buffer(&self, buffer: &pw_client_node::events::PortBuffer) -> Buffer {
        let map = self
            .mems
            .map(buffer.mem_id, buffer.offset, buffer.size)
            .unwrap();

        // Buffer memory starts with metadata, followed by chunk for each data
        let chunks_offset: usize = buffer
            .metas
            .iter()
            .map(|(_ty, size)| size.next_multiple_of(8) as usize)
            .sum();

        let datas = buffer
            .data_blocks
            .iter()
            .enumerate()
            .map(|(id, data)| {
                let chunk = map
                    .ptr_at(chunks_offset + id * std::mem::size_of::<SpaChunk>())
                    .unwrap();

                let (data_ptr, data_map) = match data.type_ {
                    SpaEnum::Value(SpaDataType::MemPtr) => {
                        assert!(data.data as usize + data.maxsize as usize <= map.len());
                        (map.ptr_at::<u8>(data.data as usize).unwrap(), None)
                    }
                    _ => {
                        let map = self
                            .mems
                            .map(data.data, data.mapoffset, data.maxsize)
                            .unwrap();
                        (map.ptr_at::<u8>(0).unwrap(), Some(map))
                    }
                };

                BufferData {
                    chunk,
                    data: data_ptr,
                    maxsize: data.maxsize,
                    _map: data_map,
                }
            })
            .collect();

        Buffer { datas, _map: map }
    }

    fn start(&mut self) {
        if let (Some(activation), false) = (&self.activation, self.running) {
            activation.set_status(PwNodeActivationStatus::Finished);
            self.running = true;
        }
    }

    fn stop(&mut self) {
        if let (Some(activation), true) = (&self.activation, self.running) {
            let old_status = activation.swap_status(PwNodeActivationStatus::Inactive);

            if old_status.ok().is_some_and(|s| s.is_pending_trigger()) {
                self.trigger_targets(monotonic_ns());
            }
            self.running = false;
        }
    }

    fn trigger_targets(&self, nsec: Duration) {
        for target in self.targets.iter() {
            if let Err(err) = target.trigger(nsec) {
                eprintln!("Failed to trigger node {}: {err}", target.node_id());
            }
        }
    }

    /// Called when graph wakes us up
    fn process(&mut self) {
        let nsec = monotonic_ns();

        let Some(activation) = self.activation.as_ref() else {
            return;
        };

        if !activation.compare_exchange_status(
            PwNodeActivationStatus::Triggered,
            PwNodeActivationStatus::Awake,
        ) {
            return;
        }
        activation.set_awake_time(nsec);

        if let Some(clock) = self.position().map(|p| p.clock) {
            let start = *self.clock_start_position.get_or_insert(clock.position);
            for mix in self.mixes.values() {
                self.process_mix(mix, clock.cycle, |offset, ty, data| {
                    let frame = clock.position.wrapping_sub(start) + offset as u64;
                    let sec = frame as f64 / clock.rate.denom as f64;
                    print_event(sec, ty, data);
                });
            }
        }

        activation.set_process_status(SpaStatus::NEED_DATA);

        let nsec = monotonic_ns();
        let was_awake = activation.compare_exchange_status(
            PwNodeActivationStatus::Awake,
            PwNodeActivationStatus::Finished,
        );
        activation.set_finish_time(nsec);

        if was_awake {
            self.trigger_targets(nsec);
        }
    }

    fn process_mix(
        &self,
        mix: &Mix,
        cycle: u32,
        mut on_event: impl FnMut(u32, SpaEnum<SpaControlType>, &[u8]),
    ) {
        let Some(io) = mix.io.as_ref() else {
            return;
        };

        // We have two areas, the one that is safe to read changes every cycle
        let io = match io.ptr_at::<SpaIoAsyncBuffers>(0) {
            Some(io) => unsafe { &raw mut (*io.as_ptr()).buffers[(cycle & 1) as usize] },
            None => io.ptr_at::<SpaIoBuffers>(0).unwrap().as_ptr(),
        };

        let SpaIoBuffers { status, buffer_id } = unsafe { io.read_volatile() };
        if SpaStatus::from_bits_retain(status) != SpaStatus::HAVE_DATA {
            return;
        }

        // Buffers can be set either on the mix, or on the port itself (invalid mix id)
        let buffer = [Some(mix), self.mixes.get(&u32::MAX)]
            .into_iter()
            .flatten()
            .find_map(|mix| mix.buffers.get(buffer_id as usize));

        if let Some(data) = buffer.and_then(|b| b.datas.first()) {
            let chunk = unsafe { data.chunk.read_volatile() };
            let offset = chunk.offset.checked_rem(data.maxsize).unwrap_or(0);
            let size = chunk.size.min(data.maxsize - offset);

            let bytes = unsafe {
                std::slice::from_raw_parts(data.data.as_ptr().add(offset as usize), size as usize)
            };

            for_each_control(bytes, &mut on_event);
        }

        unsafe { (&raw mut (*io).status).write_volatile(SpaStatus::NEED_DATA.bits()) };
    }
}

fn for_each_control(bytes: &[u8], mut cb: impl FnMut(u32, SpaEnum<SpaControlType>, &[u8])) {
    let Some(size) = bytes.first_chunk::<4>().map(|v| u32::from_ne_bytes(*v)) else {
        return;
    };
    if 8 + (size as usize).next_multiple_of(8) > bytes.len() {
        return;
    }

    let (pod, _) = pod::PodDeserializer::new(bytes);
    let Ok(sequence) = pod.as_sequence() else {
        return;
    };

    for control in sequence {
        if let Ok(data) = control.value().as_bytes() {
            cb(control.offset(), control.type_(), data);
        }
    }
}

fn print_event(sec: f64, ty: SpaEnum<SpaControlType>, data: &[u8]) {
    let hex = |data: &[u8]| {
        data.iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join(" ")
    };

    let SpaEnum::Value(SpaControlType::Midi) = ty else {
        println!("{sec:12.6} {ty:?}: {}", hex(data));
        return;
    };

    let status = data.first().copied().unwrap_or(0);
    let channel = (status & 0x0f) + 1;
    let a = data.get(1).copied().unwrap_or(0);
    let b = data.get(2).copied().unwrap_or(0);

    let description = match status & 0xf0 {
        0x80 => format!("Note Off   (channel {channel:2}): note {a:3}, velocity {b:3}"),
        0x90 => format!("Note On    (channel {channel:2}): note {a:3}, velocity {b:3}"),
        0xa0 => format!("Aftertouch (channel {channel:2}): note {a:3}, pressure {b:3}"),
        0xb0 => format!("Controller (channel {channel:2}): controller {a:3}, value {b:3}"),
        0xc0 => format!("Program    (channel {channel:2}): program {a:3}"),
        0xd0 => format!("Channel Aftertouch (channel {channel:2}): pressure {a:3}"),
        0xe0 => format!(
            "Pitch Bend (channel {channel:2}): value {}",
            ((b as i32) << 7 | a as i32) - 0x2000
        ),
        _ => format!("System {status:#04x}"),
    };

    println!("{sec:12.6} {description}  [{}]", hex(data));
}

fn socket_path() -> PathBuf {
    let runtime_dir = std::env::var_os("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR is not set");
    let remote = std::env::var_os("PIPEWIRE_REMOTE").unwrap_or("pipewire-0".into());
    PathBuf::from(runtime_dir).join(remote)
}

fn main() {
    let mut ctx = Context::<State>::connect(socket_path()).unwrap();
    ripewire::set_blocking(ctx.as_raw_fd(), false);

    let core = ctx.core();
    let client = ctx.client();

    core.hello(&mut ctx);

    client.update_properties(
        &mut ctx,
        PwDictionary::from_dict([
            ("application.name", NODE_NAME),
            ("application.process.binary", NODE_NAME),
            ("application.process.id", &std::process::id().to_string()),
        ]),
    );

    let node_props = [
        ("media.type", "Midi"),
        ("media.category", "Filter"),
        ("media.role", "DSP"),
        ("media.name", NODE_NAME),
        ("node.name", NODE_NAME),
        ("node.want-driver", "true"),
    ];

    let node: PwClientNode = core.create_object(
        &mut ctx,
        pw_core::methods::CreateObject {
            factory_name: "client-node".into(),
            interface: "PipeWire:Interface:ClientNode".into(),
            version: 6,
            properties: PwDictionary::from_dict(node_props),
            new_id: 0,
        },
    );

    node.update(
        &mut ctx,
        Update {
            change_mask: UpdateChangeMask::PARAMS | UpdateChangeMask::INFO,
            params: vec![],
            info: Some(NodeInfo {
                max_input_ports: 1,
                max_output_ports: 0,
                change_mask: NodeInfoChangeMask::FLAGS
                    | NodeInfoChangeMask::PROPS
                    | NodeInfoChangeMask::PARAMS,
                flags: NodeFlags::RT,
                props: PwDictionary::from_dict(node_props),
                params: vec![],
            }),
        },
    );

    node.port_update(&mut ctx, port_update(None));
    node.set_active(&mut ctx, true);

    ctx.set_object_callback(&core, State::core_event);
    ctx.set_object_callback(&node, State::client_node_event);

    let mut state = State {
        mems: MemoryRegistry::new(),
        activation: None,
        readfd: None,
        _writefd: None,
        targets: Vec::new(),
        position: None,
        mixes: HashMap::new(),
        format: None,
        running: false,
        clock_start_position: None,
    };

    eprintln!("Waiting for MIDI on `{NODE_NAME}:input`, link something to it");

    let mut buffer = MessageBuffer::new();
    loop {
        let mut fds = [
            libc::pollfd {
                fd: ctx.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            },
            libc::pollfd {
                // Negative fds are ignored by poll
                fd: state.readfd.as_ref().map_or(-1, |fd| fd.as_raw_fd()),
                events: libc::POLLIN,
                revents: 0,
            },
        ];

        if unsafe { libc::poll(fds.as_mut_ptr(), fds.len() as libc::nfds_t, -1) } < 0 {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            panic!("{err}");
        }

        if fds[1].revents & libc::POLLIN != 0 {
            if let Some(readfd) = state.readfd.as_ref() {
                if eventfd_read(readfd.as_fd()).is_ok() {
                    state.process();
                }
            }
        }

        if fds[0].revents & (libc::POLLHUP | libc::POLLERR) != 0 {
            eprintln!("Connection closed");
            return;
        }

        if fds[0].revents & libc::POLLIN != 0 {
            loop {
                match ctx.rcv_msg(&mut buffer) {
                    Ok(msg) => ctx.dispatch_event(&mut state, msg),
                    Err(err) if err.kind() == io::ErrorKind::WouldBlock => break,
                    Err(err) => panic!("{err}"),
                }
            }
        }
    }
}
