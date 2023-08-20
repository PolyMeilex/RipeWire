use pod::dictionary::Dictionary;
use std::io;
use std::os::fd::{AsRawFd, RawFd};
use tokio::io::unix::AsyncFd;

use ripewire::context::Context;
use ripewire::global_list::GlobalList;
use ripewire::protocol::{pw_client, pw_client_node, pw_core, pw_device, pw_registry};
use ripewire::proxy::{PwClientNode, PwDevice, PwRegistry};

fn properties() -> Dictionary {
    Dictionary::from([
        ("application.name", "ripewire"),
        ("application.process.binary", "ripewire"),
    ])
}

struct State {
    context: Context,

    registry: PwRegistry,
    globals: GlobalList,

    device: Option<PwDevice>,
    client_node: Option<PwClientNode>,
}

impl State {
    pub fn core_event(&mut self, _object_id: u32, core_event: pw_core::Event, fds: &[RawFd]) {
        dbg!(&core_event);
        match core_event {
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

    pub fn done(&mut self) {}
}

#[tokio::main]
async fn main() {
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

    let fd = AsyncFd::new(context.as_raw_fd()).unwrap();

    let mut state = State {
        context,
        registry,
        globals: GlobalList::default(),

        device: None,
        client_node: None,
    };

    loop {
        let fd = fd.readable().await.unwrap();

        if fd.ready().is_readable() {
            let (messages, fds) = match state.context.rcv_msg() {
                Ok(res) => res,
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(err) => {
                    panic!("{err}");
                }
            };

            for msg in messages {
                let device = state.device.as_ref().map(|obj| obj.id().protocol_id());
                let client_node = state.client_node.as_ref().map(|obj| obj.id().protocol_id());
                match msg.header.object_id {
                    id if id == state.context.core().id().protocol_id() => {
                        let event = pw_core::Event::from(msg.header.opcode, &msg.body).unwrap();
                        state.core_event(msg.header.object_id, event, &fds);
                    }
                    id if id == state.context.client().id().protocol_id() || id == 3 => {
                        let event = pw_client::Event::from(msg.header.opcode, &msg.body).unwrap();
                        state.client_event(msg.header.object_id, event);
                    }
                    id if id == state.registry.id().protocol_id() => {
                        let event = pw_registry::Event::from(msg.header.opcode, &msg.body).unwrap();
                        state.registry_event(msg.header.object_id, event);
                    }
                    id if device == Some(id) => {
                        let event = pw_device::Event::from(msg.header.opcode, &msg.body).unwrap();
                        state.device_event(msg.header.object_id, event);
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
        }
    }
}
