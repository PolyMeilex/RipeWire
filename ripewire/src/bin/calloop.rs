#![allow(clippy::single_match)]

use std::io::{self, Read};
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd};

use calloop::{generic::Generic, EventLoop, Interest, Mode, PostAction};
use libspa_consts::{SpaDirection, SpaEnum, SpaParamType};
use pod::{dictionary::Dictionary, Value};

use ripewire::connection::MessageBuffer;
use ripewire::context::Context;
use ripewire::global_list::GlobalList;
use ripewire::memory_registry::MemoryRegistry;
use ripewire::object_map::ObjectType;
use ripewire::protocol::pw_client_node::methods::{
    NodeInfoChangeMask, PortInfoChangeMask, PortUpdateChangeMask,
};
use ripewire::protocol::{
    self, pw_client, pw_client_node, pw_core, pw_device, pw_node, pw_registry, ParamFlags,
    ParamInfo,
};
use ripewire::proxy::{PwClient, PwClientNode, PwCore, PwDevice, PwNode, PwRegistry};

fn properties() -> Dictionary {
    let host = nix::unistd::gethostname().unwrap();
    let host: &str = &host.to_string_lossy();

    let uid = nix::unistd::getuid();
    let user = nix::unistd::User::from_uid(uid).unwrap().unwrap();

    let pid = nix::unistd::getpid().to_string();

    Dictionary::from([
        ("log.level", "0"),
        ("cpu.max-align", "32"),
        ("default.clock.rate", "48000"),
        ("default.clock.quantum", "1024"),
        ("default.clock.min-quantum", "32"),
        ("default.clock.max-quantum", "2048"),
        ("default.clock.quantum-limit", "8192"),
        ("default.video.width", "640"),
        ("default.video.height", "480"),
        ("default.video.rate.num", "25"),
        ("default.video.rate.denom", "1"),
        ("clock.power-of-two-quantum", "true"),
        ("link.max-buffers", "64"),
        ("mem.warn-mlock", "false"),
        ("mem.allow-mlock", "true"),
        ("settings.check-quantum", "false"),
        ("settings.check-rate", "false"),
        ("application.name", "ripewire"),
        ("application.process.binary", "ripewire"),
        ("application.language", "en_US.UTF-8"),
        ("application.process.id", &pid),
        ("application.process.user", &user.name),
        ("application.process.host", host),
        ("window.x11.display", ":0"),
        ("core.version", "0.3.58"),
        ("core.name", "pipewire-poly-185501"),
    ])
}

pub fn run_rust() {
    let mut ctx = Context::<PipewireState>::connect("/run/user/1000/pipewire-0").unwrap();
    let core = ctx.core();
    let client = ctx.client();

    core.hello(&mut ctx);

    client.update_properties(&mut ctx, properties());

    let registry = core.get_registry(&mut ctx);

    core.sync(&mut ctx, 0, 0);

    ctx.set_object_callback(&core, PipewireState::core_event);
    ctx.set_object_callback(&client, PipewireState::client_event);
    ctx.set_object_callback(&registry, PipewireState::registry_event);

    let mut ev = EventLoop::<CalloopState>::try_new().unwrap();

    let fd = ctx.as_raw_fd();
    let mut state = CalloopState {
        ctx,
        state: PipewireState {
            registry,
            globals: GlobalList::default(),
            mems: MemoryRegistry::default(),
        },
    };

    let mut buffer = MessageBuffer::new();
    ev.handle()
        .insert_source(
            Generic::new(fd, Interest::READ, Mode::Level),
            move |_, _, state| {
                loop {
                    let msg = state.ctx.rcv_msg(&mut buffer);

                    if let Err(err) = &msg {
                        if err.kind() == io::ErrorKind::WouldBlock {
                            break;
                        }
                    }

                    let msg = msg.unwrap();

                    state.ctx.dispatch_event(&mut state.state, msg);
                }

                Ok(PostAction::Continue)
            },
        )
        .unwrap();

    ev.run(None, &mut state, |_state| {
        //
    })
    .unwrap();
}

struct CalloopState {
    ctx: Context<PipewireState>,
    state: PipewireState,
}

struct PipewireState {
    registry: PwRegistry,
    globals: GlobalList,
    mems: MemoryRegistry,
}

impl PipewireState {
    pub fn core_event(
        &mut self,
        ctx: &mut Context<Self>,
        core: PwCore,
        core_event: pw_core::Event,
    ) {
        dbg!(&core_event);
        match core_event {
            pw_core::Event::Done(done) => {
                if done.id == Some(0) && done.seq == 0 {
                    self.done(ctx);
                }
            }
            pw_core::Event::AddMem(add_mem) => {
                self.mems.add_mem(&add_mem);
            }
            pw_core::Event::RemoveMem(remove_mem) => {
                self.mems.remove_mem(&remove_mem);
            }
            pw_core::Event::Ping(ping) => {
                core.pong(ctx, ping.id, ping.seq);
            }
            _ => {}
        }
    }

    pub fn client_event(
        &mut self,
        _ctx: &mut Context<Self>,
        _client: PwClient,
        client_event: pw_client::Event,
    ) {
        dbg!(&client_event);
    }

    pub fn node_event(
        &mut self,
        ctx: &mut Context<Self>,
        node: PwNode,
        node_event: pw_node::Event,
    ) {
        dbg!(&node_event);

        match node_event {
            pw_node::Event::Info(msg) => {
                for param in msg.params.iter() {
                    if let SpaEnum::Value(id) = param.id {
                        node.enum_param(ctx, id);
                    }
                }

                println!("&node_event = {:?}", &msg);
            }
            pw_node::Event::Param(msg) => {
                let SpaEnum::Value(id) = msg.id else {
                    return;
                };

                match id {
                    ty @ (SpaParamType::Format | SpaParamType::EnumFormat) => {
                        let obj = pod_v2::obj_gen::typed::Format(
                            msg.params.as_deserializer().as_object().unwrap(),
                        );
                        println!("{ty:?}: {obj:?}");
                    }
                    ty @ (SpaParamType::Route | SpaParamType::EnumRoute) => {
                        let obj = pod_v2::obj_gen::typed::Route(
                            msg.params.as_deserializer().as_object().unwrap(),
                        );
                        println!("{ty:?}: {obj:?}");
                    }
                    ty @ (SpaParamType::Profile | SpaParamType::EnumProfile) => {
                        let obj = pod_v2::obj_gen::typed::Profile(
                            msg.params.as_deserializer().as_object().unwrap(),
                        );
                        println!("{ty:?}: {obj:?}");
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn client_node_event(
        &mut self,
        _ctx: &mut Context<Self>,
        _client: PwClientNode,
        client_node_event: pw_client_node::Event,
    ) {
        dbg!(&client_node_event);

        match client_node_event {
            pw_client_node::Event::Transport(msg) => {
                let mem = self.mems.get(&msg.memid).unwrap();

                if false {
                    let mut buf = vec![0; msg.size as usize];

                    unsafe {
                        let mut file = std::fs::File::from_raw_fd(mem.fd().as_raw_fd());
                        file.read_exact(&mut buf).unwrap();
                        let _ = file.into_raw_fd();
                    }

                    use libspa_consts::abi_unstable::PwNodeActivation;

                    assert_eq!(msg.size as usize, std::mem::size_of::<PwNodeActivation>());
                    let v = unsafe { &*(buf.as_ptr() as *const PwNodeActivation) };
                    dbg!(v);
                }
            }
            _ => {}
        }
    }

    pub fn registry_event(
        &mut self,
        _ctx: &mut Context<Self>,
        _registry: PwRegistry,
        registry_event: pw_registry::Event,
    ) {
        dbg!(&registry_event);
        self.globals.handle_event(&registry_event);
    }

    pub fn device_event(
        &mut self,
        ctx: &mut Context<Self>,
        device: PwDevice,
        device_event: pw_device::Event,
    ) {
        match device_event {
            pw_device::Event::Info(msg) => {
                for param in msg.params.iter() {
                    if let SpaEnum::Value(id) = param.id {
                        device.enum_param(ctx, id);
                    }
                }

                println!("&device_event = {:?}", &msg);
            }
            pw_device::Event::Param(msg) => {
                let SpaEnum::Value(id) = msg.id else {
                    return;
                };

                match id {
                    ty @ (SpaParamType::Route | SpaParamType::EnumRoute) => {
                        let obj = pod_v2::obj_gen::untyped::Route(
                            msg.params.as_deserializer().as_object().unwrap(),
                        );
                        println!("{ty:?}: {obj:?}");
                    }
                    ty @ (SpaParamType::Profile | SpaParamType::EnumProfile) => {
                        let obj = pod_v2::obj_gen::untyped::Profile(
                            msg.params.as_deserializer().as_object().unwrap(),
                        );
                        println!("{ty:?}: {obj:?}");
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn done(&mut self, ctx: &mut Context<Self>) {
        let client = self
            .globals
            .iter()
            .filter(|global| global.interface == ObjectType::Client)
            .nth(1);

        let device = self
            .globals
            .iter()
            .filter(|g| g.interface == ObjectType::Device)
            .find(|g| {
                g.properties.get("device.name").map(String::as_str)
                    == Some("alsa_card.pci-0000_0b_00.6")
            });

        for node in self
            .globals
            .iter()
            .filter(|global| global.interface == ObjectType::Node)
        {
            let node: PwNode = self.registry.bind(ctx, node);
            ctx.set_object_callback(&node, Self::node_event);
        }

        if let Some(global) = client {
            let client: PwClient = self.registry.bind(ctx, global);

            client.get_permissions(ctx, 0, 50);
        }

        if let Some(global) = device {
            let device: PwDevice = self.registry.bind(ctx, global);

            device.set_param(
                ctx,
                pod::params::RouteParamBuilder::route()
                    .index(4)
                    .device(4)
                    .props(
                        pod::props::PropsBuilder::new()
                            .mute(false)
                            .volume(0.1)
                            .build(),
                    )
                    .build(),
            );

            ctx.set_object_callback(&device, Self::device_event);
        }

        {
            let client_node: PwClientNode = ctx.core().create_object(
                ctx,
                pw_core::methods::CreateObject {
                    factory_name: "client-node".into(),
                    interface: "PipeWire:Interface:ClientNode".into(),
                    version: 3,
                    properties: Dictionary::from([
                        ("application.name", "rustypipe"),
                        ("media.type", "Midi"),
                        ("format.dsp", "8 bit raw midi"),
                        ("stream.is-live", "true"),
                    ]),
                    new_id: 0,
                },
            );

            let id = client_node.id().protocol_id();

            ctx.set_object_callback(&client_node, Self::client_node_event);

            let msg = pw_client_node::methods::Update {
                change_mask: pw_client_node::methods::UpdateChangeMask::from_bits_retain(0b11),
                params: vec![],
                info: Some(pw_client_node::methods::NodeInfo {
                    max_input_ports: u32::MAX,
                    max_output_ports: u32::MAX,
                    change_mask: NodeInfoChangeMask::FLAGS
                        | NodeInfoChangeMask::PROPS
                        | NodeInfoChangeMask::PARAMS,
                    flags: pw_client_node::methods::NodeFlags::RT,
                    props: Dictionary::from([
                        ("object.register", "false"),
                        ("media.type", "Midi"),
                        ("media.category", "Filter"),
                        ("media.role", "DSP"),
                        ("media.name", "ripewire"),
                        ("node.name", "ripewire"),
                        ("node.want-driver", "true"),
                    ]),
                    params: vec![
                        ParamInfo {
                            id: SpaParamType::PropInfo.into(),
                            flags: ParamFlags::empty(),
                        },
                        ParamInfo {
                            id: SpaParamType::Props.into(),
                            flags: ParamFlags::from_bits_retain(0b100),
                        },
                        ParamInfo {
                            id: SpaParamType::EnumFormat.into(),
                            flags: ParamFlags::from_bits_retain(0b010),
                        },
                        ParamInfo {
                            id: SpaParamType::Format.into(),
                            flags: ParamFlags::from_bits_retain(0b100),
                        },
                    ],
                }),
            };

            let msg = protocol::create_msg(id, &msg);

            ctx.send_msg(&msg, &[]).unwrap();

            let msg = protocol::create_msg(
                id,
                &pw_client_node::methods::PortUpdate {
                    direction: SpaEnum::Value(SpaDirection::Output),
                    port_id: 0,
                    change_mask: PortUpdateChangeMask::PARAMS | PortUpdateChangeMask::INFO,
                    params: vec![
                        Value::Object(
                            pod::params::FormatParamBuilder::enum_format()
                                .media_type(libspa_consts::SpaMediaType::Application)
                                .media_subtype(libspa_consts::SpaMediaSubtype::Control)
                                .build(),
                        ),
                        Value::Object(
                            pod::params::IoParamBuilder::io()
                                .id(libspa_consts::SpaIoType::Buffers)
                                .size(8)
                                .build(),
                        ),
                    ],
                    info: Some(pw_client_node::methods::PortInfo {
                        change_mask: PortInfoChangeMask::FLAGS
                            | PortInfoChangeMask::RATE
                            | PortInfoChangeMask::PROPS
                            | PortInfoChangeMask::PARAMS,
                        flags: pw_client_node::methods::PortFlags::empty(),
                        rate_num: 0,
                        rate_denom: 1,
                        items: pod::dictionary::Dictionary::from([
                            ("format.dsp", "8 bit raw midi"),
                            ("port.name", "output"),
                            ("port.id", "0"),
                            ("port.direction", "out"),
                            ("port.alias", "rustypipe:input"),
                        ]),
                        params: vec![
                            ParamInfo {
                                id: SpaParamType::EnumFormat.into(),
                                flags: ParamFlags::from_bits_retain(0b011),
                            },
                            ParamInfo {
                                id: SpaParamType::Meta.into(),
                                flags: ParamFlags::empty(),
                            },
                            ParamInfo {
                                id: SpaParamType::Io.into(),
                                flags: ParamFlags::from_bits_retain(0b011),
                            },
                            ParamInfo {
                                id: SpaParamType::Format.into(),
                                flags: ParamFlags::from_bits_retain(0b100),
                            },
                            ParamInfo {
                                id: SpaParamType::Buffers.into(),
                                flags: ParamFlags::from_bits_retain(0b000),
                            },
                            ParamInfo {
                                id: SpaParamType::Latency.into(),
                                flags: ParamFlags::from_bits_retain(0b100),
                            },
                        ],
                    }),
                },
            );

            ctx.send_msg(&msg, &[]).unwrap();

            let msg =
                protocol::create_msg(id, &pw_client_node::methods::SetActive { active: true });
            ctx.send_msg(&msg, &[]).unwrap();
        }
    }
}

fn main() {
    run_rust();
}
