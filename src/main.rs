use std::os::fd::{AsRawFd, RawFd};

use calloop::{generic::Generic, EventLoop, Interest, Mode, PostAction};
use context::Context;
use pod::{dictionary::Dictionary, Value};

mod connection;

mod global_list;
use global_list::GlobalList;
use protocol::{pw_client, pw_client_node, pw_core, pw_device, pw_registry};
use proxy::{PwClientNode, PwDevice, PwRegistry};

pub mod context;
pub mod memory_registry;
pub mod object_map;
pub mod protocol;
pub mod proxy;

pub const MAX_FDS_OUT: usize = 28;

fn properties() -> Dictionary {
    let host = nix::unistd::gethostname().unwrap();
    let host: &str = &host.to_string_lossy();

    let uid = nix::unistd::getuid();
    let user = nix::unistd::User::from_uid(uid).unwrap().unwrap();

    let pid = nix::unistd::getpid().to_string();

    Dictionary::from(
        [
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
        ]
        .into_iter(),
    )
}

pub fn run_rust() {
    let context = Context::connect("/run/user/1000/pipewire-0").unwrap();

    context.core().hello(pw_core::methods::Hello { version: 3 });

    context
        .client()
        .update_properties(pw_client::methods::UpdateProperties {
            properties: properties(),
        });

    let registry = context.core().get_registry(pw_core::methods::GetRegistry {
        version: 3,
        new_id: 2,
    });

    context
        .core()
        .sync(pw_core::methods::Sync { id: 0, seq: 0 });

    let mut ev = EventLoop::<State>::try_new().unwrap();

    let fd = context.as_raw_fd();
    let mut state = State {
        context,
        registry,
        globals: GlobalList::default(),

        device: None,
        client_node: None,
    };

    ev.handle()
        .insert_source(
            Generic::new(fd, Interest::READ, Mode::Level),
            |_, _, state| {
                let (messages, fds) = state.context.rcv_msg();
                for msg in messages {
                    let device = state.device.as_ref().map(|obj| obj.id().protocol_id());
                    let client_node = state.client_node.as_ref().map(|obj| obj.id().protocol_id());

                    match msg.header.object_id {
                        protocol::pw_core::OBJECT_ID => {
                            let event = pw_core::Event::from(msg.header.opcode, &msg.body).unwrap();

                            // dbg!(&event);
                            state.core_event(msg.header.object_id, event, &fds);
                        }
                        protocol::pw_client::OBJECT_ID => {
                            let client =
                                pw_client::Event::from(msg.header.opcode, &msg.body).unwrap();

                            dbg!(&client);
                            state.client_event(msg.header.object_id, client);
                        }
                        id if id == state.registry.id().protocol_id() => {
                            let event =
                                pw_registry::Event::from(msg.header.opcode, &msg.body).unwrap();

                            // dbg!(&event);
                            state.registry_event(msg.header.object_id, event);
                        }
                        id if device == Some(id) => {
                            let device =
                                pw_device::Event::from(msg.header.opcode, &msg.body).unwrap();

                            // dbg!(&device);
                            state.device_event(msg.header.object_id, device);
                        }
                        id if client_node == Some(id) => {
                            let client_node =
                                pw_client_node::Event::from(msg.header.opcode, &msg.body).unwrap();
                            dbg!(client_node);
                        }
                        _ => {
                            unimplemented!("{:?}", msg.header);
                            // let value = PodDeserializer::deserialize_from(&msg.body).unwrap().1;
                            // dbg!(msg.header);
                        }
                    }
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

struct State {
    context: Context,

    registry: PwRegistry,
    globals: GlobalList,

    device: Option<PwDevice>,
    client_node: Option<PwClientNode>,
}

impl State {
    pub fn core_event(&mut self, _object_id: u32, event: pw_core::Event, fds: &[RawFd]) {
        dbg!(&event);
        match event {
            pw_core::Event::Done(done) => {
                if done.id == 0 && done.seq == 0 {
                    self.done();
                }
            }
            pw_core::Event::AddMem(add_mem) => {
                self.context.add_mem(&add_mem, fds);
            }
            pw_core::Event::RemoveMem(remove_mem) => {
                self.context.remove_mem(&remove_mem);
            }
            pw_core::Event::Ping(ping) => {
                self.context.core().pong(pw_core::methods::Pong {
                    id: ping.id as u32,
                    seq: ping.seq,
                });
            }
            _ => {}
        }
    }

    pub fn client_event(&mut self, _object_id: u32, _event: pw_client::Event) {
        //
    }

    pub fn registry_event(&mut self, _object_id: u32, event: pw_registry::Event) {
        self.globals.handle_event(&event);
    }

    pub fn device_event(&mut self, _object_id: u32, _event: pw_device::Event) {}

    pub fn done(&mut self) {
        let device = self.globals.globals.iter().find(|global| {
            global.obj_type == "PipeWire:Interface:Device"
                && matches!(
                    global.properties.0.get("device.name").map(|s| s.as_str()),
                    Some("alsa_card.pci-0000_03_00.6")
                )
        });

        if let Some(_global) = device {
            // let device = self.registry.bind::<PwDevice>(pw_registry::methods::Bind {
            //     id: global.id,
            //     obj_type: global.obj_type.clone(),
            //     version: global.version,
            //     new_id: 3,
            // });

            // device.set_mute(false);

            // self.device = Some(device);
        }

        {
            let client_node =
                self.context
                    .core()
                    .create_object::<PwClientNode>(pw_core::methods::CreateObject {
                        factory_name: "client-node".into(),
                        obj_type: "PipeWire:Interface:ClientNode".into(),
                        version: 3,
                        properties: Dictionary::from(
                            [
                                ("application.name", "rustypipe"),
                                ("media.type", "Midi"),
                                ("format.dsp", "8 bit raw midi"),
                                ("stream.is-live", "true"),
                            ]
                            .into_iter(),
                        ),
                        new_id: 4,
                    });

            let id = client_node.id().protocol_id();

            self.client_node = Some(client_node);
            // return;

            // Client node update
            {
                use Value::*;
                let body = Struct(vec![
                    Int(3),
                    Int(0),
                    Struct(vec![
                        Int(-1),
                        Int(-1),
                        Long(7),
                        Long(1),
                        Int(7),
                        String("object.register".into()),
                        String("false".into()),
                        String("media.type".into()),
                        String("Midi".into()),
                        String("media.category".into()),
                        String("Filter".into()),
                        String("media.role".into()),
                        String("DSP".into()),
                        String("media.name".into()),
                        String("ripewire".into()),
                        String("node.name".into()),
                        String("ripewire".into()),
                        String("node.want-driver".into()),
                        String("true".into()),
                        Int(3),
                        Id(pod::utils::Id(1)),
                        Int(0),
                        Id(pod::utils::Id(2)),
                        Int(4),
                        Id(pod::utils::Id(16)),
                        Int(0),
                    ]),
                ]);

                let msg = protocol::manual_create_msg(id, 2, &body);

                self.context.send_msg(&msg, &[]).unwrap();
            }

            {
                use Value::*;

                let body = protocol::pw_client_node::event::PortUpdate {
                    direction: 0,
                    port_id: 0,
                    change_mask: 3,
                    params: vec![
                        Object(pod::Object {
                            type_: libspa_consts::SpaType::ObjectFormat as u32,
                            id: libspa_consts::SpaParamType::EnumFormat as u32,
                            properties: vec![
                                pod::Property {
                                    key: libspa_consts::spa_format::SPA_FORMAT_mediaType as u32,
                                    flags: pod::PropertyFlags::empty(),
                                    // application
                                    value: Id(pod::utils::Id(6)),
                                },
                                pod::Property {
                                    key: libspa_consts::spa_format::SPA_FORMAT_mediaSubtype as u32,
                                    flags: pod::PropertyFlags::empty(),
                                    // control
                                    value: Id(pod::utils::Id(393217)),
                                },
                            ],
                        }),
                        Object(pod::Object {
                            type_: libspa_consts::SpaType::ObjectParamIo as u32,
                            id: libspa_consts::SpaParamType::IO as u32,
                            properties: vec![
                                pod::Property {
                                    key: 1,
                                    flags: pod::PropertyFlags::empty(),
                                    value: Id(pod::utils::Id(1)),
                                },
                                pod::Property {
                                    key: 2,
                                    flags: pod::PropertyFlags::empty(),
                                    value: Int(8),
                                },
                            ],
                        }),
                    ],
                    info: Some(protocol::pw_client_node::event::PortUpdateInfo {
                        change_mask: 15,
                        flags: 0,
                        rate_num: 0,
                        rate_denom: 1,
                        items: vec![
                            ("format.dsp".into(), "8 bit raw midi".into()),
                            ("port.name".into(), "input".into()),
                            ("port.id".into(), "0".into()),
                            ("port.direction".into(), "in".into()),
                            ("port.alias".into(), "rustypipe:input".into()),
                        ],
                        params: vec![
                            (pod::utils::Id(3), 3),
                            (pod::utils::Id(6), 0),
                            (pod::utils::Id(7), 3),
                            (pod::utils::Id(4), 4),
                            (pod::utils::Id(5), 0),
                            (pod::utils::Id(15), 4),
                        ],
                    }),
                };

                let msg = protocol::manual_create_msg(id, 3, &body);

                self.context.send_msg(&msg, &[]).unwrap();
            }

            {
                let body = pw_client_node::methods::SetActive { active: true };

                let msg = protocol::manual_create_msg(id, 4, &body);
                self.context.send_msg(&msg, &[]).unwrap();
            }
        }
    }
}

fn main() {
    run_rust();
}
