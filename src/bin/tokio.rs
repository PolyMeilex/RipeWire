use pod::dictionary::Dictionary;
use std::io;
use std::os::fd::AsRawFd;
use tokio::io::unix::AsyncFd;

use ripewire::context::Context;
use ripewire::global_list::GlobalList;
use ripewire::protocol::{pw_client, pw_core, pw_device, pw_registry};
use ripewire::proxy::{ObjectId, Proxy, PwClient, PwCore, PwDevice, PwRegistry};

fn properties() -> Dictionary {
    Dictionary::from([
        ("application.name", "ripewire"),
        ("application.process.binary", "ripewire"),
    ])
}

struct PipewireState {
    registry: PwRegistry,
    globals: GlobalList,
}

impl PipewireState {
    pub fn core_event(
        &mut self,
        context: &mut Context<Self>,
        core: PwCore,
        core_event: pw_core::Event,
    ) {
        dbg!(&core_event);

        // TODO:
        let fds = [];

        match core_event {
            pw_core::Event::Done(done) => {
                if done.id == 0 && done.seq == 0 {
                    self.done(context);
                }
            }
            pw_core::Event::AddMem(add_mem) => {
                context.add_mem(&add_mem, &fds);
            }
            pw_core::Event::RemoveMem(remove_mem) => {
                context.remove_mem(&remove_mem);
            }
            pw_core::Event::Ping(ping) => {
                core.pong(
                    context,
                    pw_core::methods::Pong {
                        id: ping.id as u32,
                        seq: ping.seq,
                    },
                );
            }
            _ => {}
        }
    }

    pub fn client_event(
        &mut self,
        _context: &mut Context<Self>,
        _client: PwClient,
        client_event: pw_client::Event,
    ) {
        dbg!(&client_event);
    }

    pub fn registry_event(
        &mut self,
        _context: &mut Context<Self>,
        _registry: PwRegistry,
        registry_event: pw_registry::Event,
    ) {
        dbg!(&registry_event);
        self.globals.handle_event(&registry_event);
    }

    pub fn device_event(
        &mut self,
        _context: &mut Context<Self>,
        _device: PwDevice,
        device_event: pw_device::Event,
    ) {
        dbg!(&device_event);
    }

    pub fn done(&mut self, context: &mut Context<Self>) {
        let device = self.globals.globals.iter().find(|global| {
            global.obj_type == "PipeWire:Interface:Device"
                && matches!(
                    global.properties.0.get("device.name").map(|s| s.as_str()),
                    Some("alsa_card.pci-0000_0b_00.6")
                )
        });

        let Some(global) = device else { return; };
        let device: PwDevice = self.registry.bind(context, global);

        context.set_object_callback(&device, Self::device_event);

        device.set_mute(context, 4, 4, true);
    }
}

struct State {
    ctx: Context<PipewireState>,
    state: PipewireState,
}

#[tokio::main]
async fn main() {
    let mut ctx = Context::<PipewireState>::connect("/run/user/1000/pipewire-0").unwrap();
    let core = ctx.core();
    let client = ctx.client();

    ctx.core()
        .hello(&mut ctx, pw_core::methods::Hello { version: 3 });

    ctx.client().update_properties(
        &mut ctx,
        pw_client::methods::UpdateProperties {
            properties: properties(),
        },
    );

    let registry = ctx.core().get_registry(
        &mut ctx,
        pw_core::methods::GetRegistry {
            version: 3,
            new_id: 0,
        },
    );

    core.sync(&mut ctx, pw_core::methods::Sync { id: 0, seq: 0 });

    ctx.set_object_callback(&core, PipewireState::core_event);
    ctx.set_object_callback(&client, PipewireState::client_event);
    ctx.set_object_callback(&registry, PipewireState::registry_event);

    let fd = AsyncFd::new(ctx.as_raw_fd()).unwrap();

    let mut state = State {
        ctx,
        state: PipewireState {
            registry,
            globals: GlobalList::default(),
        },
    };

    loop {
        let fd = fd.readable().await.unwrap();

        if fd.ready().is_readable() {
            let (messages, fds) = match state.ctx.rcv_msg() {
                Ok(res) => res,
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(err) => {
                    panic!("{err}");
                }
            };

            if !fds.is_empty() {
                todo!();
            }

            for msg in messages {
                let id = ObjectId::new(msg.header.object_id);

                match state.ctx.object_type(&id).unwrap() {
                    ripewire::object_map::ObjectType::Core => {
                        let event = pw_core::Event::from(msg.header.opcode, &msg.body).unwrap();
                        let core = PwCore::from_id(id);
                        state.ctx.call_cb(&mut state.state, core, event);
                    }
                    ripewire::object_map::ObjectType::Client => {
                        let event = pw_client::Event::from(msg.header.opcode, &msg.body).unwrap();
                        let client = PwClient::from_id(id);
                        state.ctx.call_cb(&mut state.state, client, event);
                    }
                    ripewire::object_map::ObjectType::Registry => {
                        let event = pw_registry::Event::from(msg.header.opcode, &msg.body).unwrap();
                        let registry = PwRegistry::from_id(id);
                        state.ctx.call_cb(&mut state.state, registry, event);
                    }
                    ripewire::object_map::ObjectType::Device => {
                        let event = pw_device::Event::from(msg.header.opcode, &msg.body).unwrap();
                        let device = PwDevice::from_id(id);
                        state.ctx.call_cb(&mut state.state, device, event);
                    }
                    ty => unimplemented!("{ty:?}"),
                }
            }
        }
    }
}
