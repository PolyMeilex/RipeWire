use std::os::fd::AsRawFd;

use calloop::{generic::Generic, EventLoop, Interest, Mode, PostAction};
use pod::{dictionary::Dictionary, Value};

use ripewire::context::Context;
use ripewire::global_list::GlobalList;
use ripewire::protocol::{self, pw_client, pw_client_node, pw_core, pw_device, pw_registry};
use ripewire::proxy::{PwClient, PwClientNode, PwCore, PwDevice, PwRegistry};

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

    ctx.core().hello(&mut ctx);

    ctx.client().update_properties(&mut ctx, properties());

    let registry = ctx.core().get_registry(&mut ctx);

    ctx.core().sync(&mut ctx, 0, 0);

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
        },
    };

    ev.handle()
        .insert_source(
            Generic::new(fd, Interest::READ, Mode::Level),
            |_, _, state| {
                let (messages, fds) = state.ctx.rcv_msg().unwrap();
                for msg in messages {
                    state.ctx.dispatch_event(&mut state.state, msg, &fds);
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
                if done.id == 0 && done.seq == 0 {
                    self.done(ctx);
                }
            }
            pw_core::Event::AddMem(add_mem) => {
                ctx.add_mem(&add_mem);
            }
            pw_core::Event::RemoveMem(remove_mem) => {
                ctx.remove_mem(&remove_mem);
            }
            pw_core::Event::Ping(ping) => {
                core.pong(ctx, ping.id as u32, ping.seq);
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

    pub fn client_node_event(
        &mut self,
        _ctx: &mut Context<Self>,
        _client: PwClientNode,
        client_node_event: pw_client_node::Event,
    ) {
        dbg!(&client_node_event);
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
        _ctx: &mut Context<Self>,
        _device: PwDevice,
        device_event: pw_device::Event,
    ) {
        dbg!(&device_event);
    }

    pub fn done(&mut self, ctx: &mut Context<Self>) {
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
            let client: PwClient = self.registry.bind(ctx, global);

            client.get_permissions(ctx, 0, 50);
        }

        if let Some(global) = device {
            let device: PwDevice = self.registry.bind(ctx, global);
            device.set_mute(ctx, 4, 4, false);

            ctx.set_object_callback(&device, Self::device_event);
        }

        {
            let client_node: PwClientNode = ctx.core().create_object(
                ctx,
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

            ctx.set_object_callback(&client_node, Self::client_node_event);

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

                ctx.send_msg(&msg, &[]).unwrap();
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

                ctx.send_msg(&msg, &[]).unwrap();
            }

            {
                let body = pw_client_node::methods::SetActive { active: true };

                let msg = protocol::manual_create_msg(id, 4, &body);
                ctx.send_msg(&msg, &[]).unwrap();
            }
        }
    }
}

fn main() {
    run_rust();
}
