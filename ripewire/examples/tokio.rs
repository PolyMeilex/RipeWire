use libspa_consts::{SpaParamRoute, SpaParamType, SpaProp, SpaType};
use ripewire::connection::MessageBuffer;
use ripewire::memory_registry::MemoryRegistry;
use ripewire::object_map::ObjectType;
use ripewire::HashMapExt;
use std::collections::HashMap;
use std::io;
use std::os::fd::AsRawFd;
use tokio::io::unix::AsyncFd;

use ripewire::context::Context;
use ripewire::global_list::GlobalList;
use ripewire::protocol::{pw_client, pw_core, pw_device, pw_registry, PwDictionary};
use ripewire::proxy::{PwClient, PwCore, PwDevice, PwRegistry};

struct PipewireState {
    registry: PwRegistry,
    globals: GlobalList,
    mems: MemoryRegistry,
}

impl PipewireState {
    pub fn core_event(
        &mut self,
        context: &mut Context<Self>,
        core: PwCore,
        core_event: pw_core::Event,
    ) {
        dbg!(&core_event);

        match core_event {
            pw_core::Event::Done(done) => {
                if done.id == Some(0) && done.seq == 0 {
                    self.done(context);
                }
            }
            pw_core::Event::AddMem(add_mem) => {
                self.mems.add_mem(&add_mem);
            }
            pw_core::Event::RemoveMem(remove_mem) => {
                self.mems.remove_mem(&remove_mem);
            }
            pw_core::Event::Ping(ping) => {
                core.pong(context, ping.id, ping.seq);
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
        let device = self
            .globals
            .iter()
            .filter(|g| g.interface == ObjectType::Device)
            .find(|g| {
                g.properties.get("device.name").map(String::as_str)
                    == Some("alsa_card.pci-0000_0b_00.6")
            });

        let Some(global) = device else {
            return;
        };
        let device: PwDevice = self.registry.bind(context, global);

        context.set_object_callback(&device, Self::device_event);

        device.set_param(
            context,
            // TODO: Not very fun to write
            pod::Builder::with(|b| {
                b.write_object_with(SpaType::ObjectParamRoute, SpaParamType::Route as u32, |b| {
                    b.write_property(SpaParamRoute::Index as u32, 0, |b| {
                        b.write_u32(4);
                    });
                    b.write_property(SpaParamRoute::Device as u32, 0, |b| {
                        b.write_u32(4);
                    });
                    b.write_property(SpaParamRoute::Props as u32, 0, |b| {
                        b.write_object_with(
                            SpaType::ObjectProps,
                            SpaParamType::Route as u32,
                            |b| {
                                b.write_property(SpaProp::Mute as u32, 0, |b| {
                                    b.write_bool(false);
                                });
                                b.write_property(SpaProp::Volume as u32, 0, |b| {
                                    b.write_float(0.1);
                                });
                            },
                        );
                    });
                });
            }),
        );
    }
}

struct TokioState {
    ctx: Context<PipewireState>,
    state: PipewireState,
}

#[tokio::main]
async fn main() {
    let mut ctx = Context::<PipewireState>::connect("/run/user/1000/pipewire-0").unwrap();
    let core = ctx.core();
    let client = ctx.client();

    ctx.core().hello(&mut ctx);

    ctx.client().update_properties(
        &mut ctx,
        PwDictionary::from_dict([
            ("application.name", "ripewire"),
            ("application.process.binary", "ripewire"),
        ]),
    );

    let registry = ctx.core().get_registry(&mut ctx);

    core.sync(&mut ctx, 0, 0);

    ctx.set_object_callback(&core, PipewireState::core_event);
    ctx.set_object_callback(&client, PipewireState::client_event);
    ctx.set_object_callback(&registry, PipewireState::registry_event);

    let fd = AsyncFd::new(ctx.as_raw_fd()).unwrap();

    let mut state = TokioState {
        ctx,
        state: PipewireState {
            registry,
            globals: GlobalList::default(),
            mems: MemoryRegistry::default(),
        },
    };

    let mut buffer = MessageBuffer::new();
    loop {
        let fd = fd.readable().await.unwrap();

        if fd.ready().is_readable() {
            loop {
                let msg = match state.ctx.rcv_msg(&mut buffer) {
                    Ok(res) => res,
                    Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                        break;
                    }
                    Err(err) => {
                        panic!("{err}");
                    }
                };

                state.ctx.dispatch_event(&mut state.state, msg);
            }
        }
    }
}
