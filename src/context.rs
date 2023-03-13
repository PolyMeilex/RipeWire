use std::{
    io,
    os::fd::{AsRawFd, RawFd},
    path::Path,
    sync::{Arc, Mutex, Weak},
};

use crate::{
    connection::{Connection, Message},
    memory_registry::MemoryRegistry,
    object_map::{Object, ObjectMap},
    protocol::pw_core,
    proxy::{ObjectId, PwClient, PwCore},
};

#[derive(Clone)]
struct ObjectData {
    //
}

pub struct ContextInner {
    conn: Connection,
    map: ObjectMap<ObjectData>,
    mem: MemoryRegistry,
}

pub struct Context {
    inner: Arc<Mutex<ContextInner>>,
}

impl Context {
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let conn = Connection::connect(path)?;

        let inner = ContextInner {
            conn,
            map: ObjectMap::new(),
            mem: MemoryRegistry::new(),
        };

        let this = Self {
            inner: Arc::new(Mutex::new(inner)),
        };

        let _client_obj_id = this.new_object();

        Ok(this)
    }

    pub fn core(&self) -> PwCore {
        PwCore::new(0, self)
    }

    pub fn client(&self) -> PwClient {
        PwClient::new(1, self)
    }

    pub fn new_object(&self) -> ObjectId {
        let mut guard = self.inner.lock().unwrap();

        let new_id = guard.map.client_insert_new(Object {
            interface: "",
            version: 3,
            data: ObjectData {},
        });

        ObjectId::new(new_id)
    }

    pub fn send_msg(&self, bytes: &[u8], fds: &[RawFd]) -> io::Result<usize> {
        self.inner.lock().unwrap().conn.send_msg(bytes, fds)
    }

    pub fn rcv_msg(&self) -> (Vec<Message>, Vec<RawFd>) {
        self.inner.lock().unwrap().conn.rcv_msg()
    }

    pub fn downgrade(&self) -> WeakContext {
        WeakContext {
            inner: Arc::downgrade(&self.inner),
        }
    }

    pub fn add_mem(&mut self, add_mem: &pw_core::event::AddMem, fds: &[RawFd]) {
        self.inner.lock().unwrap().mem.add_mem(add_mem, fds)
    }

    pub fn remove_mem(&mut self, remove_mem: &pw_core::event::RemoveMem) {
        self.inner.lock().unwrap().mem.remove_mem(remove_mem)
    }
}

impl AsRawFd for Context {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.lock().unwrap().conn.as_raw_fd()
    }
}

#[derive(Debug, Clone)]
pub struct WeakContext {
    inner: Weak<Mutex<ContextInner>>,
}

impl WeakContext {
    pub fn upgrade(&self) -> Option<Context> {
        Some(Context {
            inner: self.inner.upgrade()?,
        })
    }
}
