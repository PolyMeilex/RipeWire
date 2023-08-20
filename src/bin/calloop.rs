use std::os::fd::AsRawFd;

use calloop::{generic::Generic, EventLoop, Interest, Mode, PostAction};
use pod::{dictionary::Dictionary, Value};

use ripewire::context::Context;
use ripewire::global_list::GlobalList;
use ripewire::protocol::{self, pw_client, pw_client_node, pw_core, pw_device, pw_registry};
use ripewire::proxy::{ObjectId, PwClient, PwClientNode, PwDevice, PwRegistry};

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
    let mut context = Context::connect("/run/user/1000/pipewire-0").unwrap();

    context
        .core()
        .hello(&mut context, pw_core::methods::Hello { version: 3 });

    context.client().update_properties(
        &mut context,
        pw_client::methods::UpdateProperties {
            properties: properties(),
        },
    );

    let registry = context.core().get_registry(
        &mut context,
        pw_core::methods::GetRegistry {
            version: 3,
            new_id: 2,
        },
    );

    context
        .core()
        .sync(&mut context, pw_core::methods::Sync { id: 0, seq: 0 });

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
                let (messages, fds) = state.context.rcv_msg().unwrap();
                for msg in messages {
                    let id = ObjectId::new(msg.header.object_id);

                    match state.context.object_type(&id).unwrap() {
                        ripewire::object_map::ObjectType::Core => {
                            let event =
                                pw_core::Event::from(msg.header.opcode, &msg.body, &fds).unwrap();
                            state.core_event(msg.header.object_id, event);
                        }
                        ripewire::object_map::ObjectType::Client => {
                            let event =
                                pw_client::Event::from(msg.header.opcode, &msg.body, &fds).unwrap();
                            state.client_event(msg.header.object_id, event);
                        }
                        ripewire::object_map::ObjectType::Registry => {
                            let event =
                                pw_registry::Event::from(msg.header.opcode, &msg.body, &fds)
                                    .unwrap();
                            state.registry_event(msg.header.object_id, event);
                        }
                        ripewire::object_map::ObjectType::Device => {
                            let event =
                                pw_device::Event::from(msg.header.opcode, &msg.body, &fds).unwrap();
                            state.device_event(msg.header.object_id, event);
                        }
                        ripewire::object_map::ObjectType::ClientNode => {
                            let client_node =
                                pw_client_node::Event::from(msg.header.opcode, &msg.body, &fds)
                                    .unwrap();
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
    pub fn core_event(&mut self, _object_id: u32, core_event: pw_core::Event) {
        dbg!(&core_event);
        match core_event {
            pw_core::Event::Done(done) => {
                if done.id == 0 && done.seq == 0 {
                    self.done();
                }
            }
            pw_core::Event::AddMem(add_mem) => {
                self.context.add_mem(&add_mem);
            }
            pw_core::Event::RemoveMem(remove_mem) => {
                self.context.remove_mem(&remove_mem);
            }
            pw_core::Event::Ping(ping) => {
                self.context.core().pong(
                    &mut self.context,
                    pw_core::methods::Pong {
                        id: ping.id as u32,
                        seq: ping.seq,
                    },
                );
            }
            _ => {}
        }
    }

    pub fn client_event(&mut self, _object_id: u32, client_event: pw_client::Event) {
        dbg!(&client_event);
    }

    pub fn registry_event(&mut self, _object_id: u32, registry_event: pw_registry::Event) {
        dbg!(&registry_event);
        self.globals.handle_event(&registry_event);
    }

    pub fn device_event(&mut self, _object_id: u32, device_event: pw_device::Event) {
        dbg!(&device_event);
    }

    pub fn done(&mut self) {
        let client = self
            .globals
            .globals
            .iter()
            .filter(|global| global.obj_type == "PipeWire:Interface:Client")
            .skip(1)
            .next();

        let device = self.globals.globals.iter().find(|global| {
            global.obj_type == "PipeWire:Interface:Device"
                && matches!(
                    global.properties.0.get("device.name").map(|s| s.as_str()),
                    Some("alsa_card.pci-0000_0b_00.6")
                )
        });

        if let Some(global) = client {
            let client: PwClient = self.registry.bind(&mut self.context, global);

            client.get_permissions(
                &mut self.context,
                pw_client::methods::GetPermissions { index: 0, num: 50 },
            );
        }

        if let Some(global) = device {
            let device: PwDevice = self.registry.bind(&mut self.context, global);

            device.set_mute(&mut self.context, 4, 4, false);

            self.device = Some(device);
        }

        {
            let client_node: PwClientNode = self.context.core().create_object(
                &mut self.context,
                pw_core::methods::CreateObject {
                    factory_name: "client-node".into(),
                    obj_type: "PipeWire:Interface:ClientNode".into(),
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

                let body = protocol::pw_client_node::methods::PortUpdate {
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
                    info: Some(protocol::pw_client_node::methods::PortInfo {
                        change_mask: 15,
                        flags: 0,
                        rate_num: 0,
                        rate_denom: 1,
                        items: pod::dictionary::Dictionary::from([
                            ("format.dsp", "8 bit raw midi"),
                            ("port.name", "input"),
                            ("port.id", "0"),
                            ("port.direction", "in"),
                            ("port.alias", "rustypipe:input"),
                        ]),
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
