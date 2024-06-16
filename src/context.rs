use std::{
    any::Any,
    io,
    os::fd::{AsRawFd, RawFd},
    path::Path,
};

use crate::{
    connection::{Connection, Message},
    object_map::{Object, ObjectMap, ObjectType},
    protocol::{pw_client, pw_client_node, pw_core, pw_device, pw_registry},
    proxy::{ObjectId, Proxy, PwClient, PwClientNode, PwCore, PwDevice, PwRegistry},
};

struct ObjectData<D> {
    cb: Option<Box<dyn FnMut(&mut Context<D>, &mut D, ObjectId, Box<dyn Any>)>>,
}

pub struct Context<D = ()> {
    conn: Connection,
    map: ObjectMap<ObjectData<D>>,
}

impl<D> Context<D> {
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let conn = Connection::connect(path)?;

        let mut this = Self {
            conn,
            map: ObjectMap::new(),
        };

        let core_id = this.map.client_insert_new(Object {
            interface: ObjectType::Core,
            version: 3,
            data: ObjectData { cb: None },
        });

        let client_id = this.map.client_insert_new(Object {
            interface: ObjectType::Client,
            version: 3,
            data: ObjectData { cb: None },
        });

        assert_eq!(core_id, 0);
        assert_eq!(client_id, 1);

        Ok(this)
    }

    pub fn core(&self) -> PwCore {
        PwCore::new(pw_core::OBJECT_ID)
    }

    pub fn client(&self) -> PwClient {
        PwClient::new(pw_client::OBJECT_ID)
    }

    pub fn new_object(&mut self, kind: ObjectType) -> ObjectId {
        let new_id = self.map.client_insert_new(Object {
            interface: kind,
            version: 3,
            data: ObjectData { cb: None },
        });

        ObjectId::new(new_id)
    }

    pub fn send_msg(&mut self, bytes: &[u8], fds: &[RawFd]) -> io::Result<usize> {
        self.conn.send_msg(bytes, fds)
    }

    pub fn rcv_msg(&mut self) -> io::Result<(Vec<Message>, Vec<RawFd>)> {
        self.conn.rcv_msg()
    }

    pub fn object_type(&mut self, object_id: &ObjectId) -> Option<ObjectType> {
        let obj = self.map.find(object_id.protocol_id())?;
        Some(obj.interface.clone())
    }

    pub fn dispatch_event(&mut self, state: &mut D, msg: Message, fds: &[RawFd]) {
        let id = ObjectId::new(msg.header.object_id);

        match self.object_type(&id).unwrap() {
            ObjectType::Core => {
                let mut pod = msg.body.as_deserializer();
                let event = pw_core::Event::deserialize(msg.header.opcode, &mut pod, fds).unwrap();

                if let pw_core::Event::RemoveId(ref event) = event {
                    self.map.remove(event.id);
                }

                let core = PwCore::from_id(id);
                self.dispatch_event_inner(state, core, event);
            }
            ObjectType::Client => {
                let mut pod = msg.body.as_deserializer();
                let event =
                    pw_client::Event::deserialize(msg.header.opcode, &mut pod, fds).unwrap();

                let client = PwClient::from_id(id);
                self.dispatch_event_inner(state, client, event);
            }
            ObjectType::ClientNode => {
                let mut pod = msg.body.as_deserializer();
                let event =
                    pw_client_node::Event::deserialize(msg.header.opcode, &mut pod, fds).unwrap();
                let client_node = PwClientNode::from_id(id);
                self.dispatch_event_inner(state, client_node, event);
            }
            ObjectType::Registry => {
                let mut pod = msg.body.as_deserializer();
                let event =
                    pw_registry::Event::deserialize(msg.header.opcode, &mut pod, fds).unwrap();

                let registry = PwRegistry::from_id(id);
                self.dispatch_event_inner(state, registry, event);
            }
            ObjectType::Device => {
                let mut pod = msg.body.as_deserializer();
                let event =
                    pw_device::Event::deserialize(msg.header.opcode, &mut pod, fds).unwrap();

                let device = PwDevice::from_id(id);
                self.dispatch_event_inner(state, device, event);
            }
            ty => unimplemented!("{ty:?}"),
        }
    }

    fn dispatch_event_inner<P>(&mut self, state: &mut D, object: P, event: P::Event)
    where
        P: Proxy,
        P::Event: 'static,
    {
        let mut cb = {
            let Some(obj) = self.map.find_mut(object.id().protocol_id()) else {
                return;
            };
            obj.data.cb.take()
        };

        if let Some(cb) = cb.as_mut() {
            (cb)(self, state, object.id(), Box::new(event));
        }

        let Some(obj) = self.map.find_mut(object.id().protocol_id()) else {
            return;
        };
        obj.data.cb = cb;
    }

    pub fn set_object_callback<P, F>(&mut self, proxy: &P, mut cb: F)
    where
        P: Proxy,
        P::Event: 'static,
        F: FnMut(&mut D, &mut Self, P, P::Event) + 'static,
    {
        let Some(obj) = self.map.find_mut(proxy.id().protocol_id()) else {
            return;
        };

        obj.data.cb = Some(Box::new(move |context, state, object_id, event| {
            let event: Box<P::Event> = event.downcast().unwrap();
            cb(state, context, P::from_id(object_id), *event);
        }));
    }
}

impl<D> AsRawFd for Context<D> {
    fn as_raw_fd(&self) -> RawFd {
        self.conn.as_raw_fd()
    }
}
