use std::os::fd::RawFd;

use super::*;

pub const OBJECT_ID: u32 = 0;

pub mod methods {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct AddListener {}

    impl MethodSerializeSimple for AddListener {
        const OPCODE: u8 = 0;
        fn serialize_simple(&self, buff: impl Write + Seek) {
            unreachable!()
        }
    }

    /// Start a conversation with the server. This will send
    /// the core info and will destroy all resources for the client
    /// (except the core and client resource).
    #[derive(Debug, Clone)]
    pub struct Hello {
        pub version: u32,
    }

    impl MethodSerializeSimple for Hello {
        const OPCODE: u8 = 1;
        fn serialize_simple(&self, mut buff: impl Write + Seek) {
            pod::Builder::new(&mut buff).push_struct_with(|b| {
                b.write_u32(self.version);
            });
        }
    }

    /// Do server roundtrip
    ///
    /// Ask the server to emit the 'done' event with \a seq.
    ///
    /// Since methods are handled in-order and events are delivered
    /// in-order, this can be used as a barrier to ensure all previous
    /// methods and the resulting events have been handled.
    ///
    /// seq - the seq number passed to the done event
    #[derive(Debug, Clone)]
    pub struct Sync {
        pub id: u32,
        pub seq: u32,
    }

    impl MethodSerializeSimple for Sync {
        const OPCODE: u8 = 2;
        fn serialize_simple(&self, mut buff: impl Write + Seek) {
            pod::Builder::new(&mut buff).push_struct_with(|b| {
                b.write_u32(self.id);
                b.write_u32(self.seq);
            });
        }
    }

    /// Reply to a server ping event.
    ///
    /// Is sent from the client to the server when the server emits the Ping event.
    /// The id and seq should be copied from the Ping event.
    #[derive(Debug, Clone)]
    pub struct Pong {
        pub id: u32,
        /// The seq number received in the ping event
        pub seq: u32,
    }

    impl MethodSerializeSimple for Pong {
        const OPCODE: u8 = 3;
        fn serialize_simple(&self, mut buff: impl Write + Seek) {
            pod::Builder::new(&mut buff).push_struct_with(|b| {
                b.write_u32(self.id);
                b.write_u32(self.seq);
            });
        }
    }

    /// An error occurred in an object on the client.
    #[derive(Debug, Clone)]
    pub struct Error {
        /// The id of the proxy that is in error.
        pub id: u32,
        /// A seq number from the failing request (if any)
        pub seq: u32,
        /// A negative errno style error code
        pub res: u32,
        /// An error message
        pub message: String,
    }

    impl MethodSerializeSimple for Error {
        const OPCODE: u8 = 4;
        fn serialize_simple(&self, mut buff: impl Write + Seek) {
            pod::Builder::new(&mut buff).push_struct_with(|b| {
                b.write_u32(self.id);
                b.write_u32(self.seq);
                b.write_u32(self.res);
                b.write_str(&self.message);
            });
        }
    }
    /// A client requests to bind to the registry object and list the available objects on the server.
    ///
    /// Like with all bindings, first the client allocates a new proxy id and puts this as the new_id field.
    /// Methods and Events can then be sent and received on the new_id (in the message Id field).
    #[derive(Debug, Clone)]
    pub struct GetRegistry {
        /// The version of the registry interface used on the client
        pub version: u32,
        /// The id of the new proxy with the registry interface
        pub new_id: u32,
    }

    impl MethodSerializeSimple for GetRegistry {
        const OPCODE: u8 = 5;
        fn serialize_simple(&self, mut buff: impl Write + Seek) {
            pod::Builder::new(&mut buff).push_struct_with(|b| {
                b.write_u32(self.version);
                b.write_u32(self.new_id);
            });
        }
    }

    /// Create a new object on the PipeWire server from a factory.
    ///
    /// factory_name - the factory name to use
    /// interface - the interface to bind to
    /// version - the version of the interface
    /// properties - extra properties
    #[derive(Debug, Clone)]
    pub struct CreateObject {
        pub factory_name: String,
        pub interface: String,
        pub version: u32,
        pub properties: PwDictionary,
        pub new_id: u32,
    }

    impl MethodSerializeSimple for CreateObject {
        const OPCODE: u8 = 6;
        fn serialize_simple(&self, mut buff: impl Write + Seek) {
            pod::Builder::new(&mut buff).push_struct_with(|b| {
                b.write_str(&self.factory_name);
                b.write_str(&self.interface);
                b.write_u32(self.version);
                b.push_struct_with(|b| {
                    b.write_u32(self.properties.len() as u32);
                    for (key, value) in self.properties.iter() {
                        b.write_str(key);
                        b.write_str(value);
                    }
                });
                b.write_u32(self.new_id);
            });
        }
    }

    /// Destroy an resource
    ///
    /// Destroy the server resource
    ///
    /// id - id of object to destroy
    #[derive(Debug, Clone)]
    pub struct Destroy {
        pub id: u32,
    }

    impl MethodSerializeSimple for Destroy {
        const OPCODE: u8 = 7;
        fn serialize_simple(&self, mut buff: impl Write + Seek) {
            pod::Builder::new(&mut buff).push_struct_with(|b| {
                b.write_u32(self.id);
            });
        }
    }
}

pub use events::{ChangeMask, MemblockFlags};
pub mod events {
    use super::*;

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct ChangeMask: u64 {
            const PROPS = 1;
        }
    }

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct MemblockFlags: u32 {
            /// memory is readable
            const READABLE = 1 << 0;
            /// memory is writable
            const WRITABLE = 1 << 1;
            /// seal the fd
            const SEAL = 1 << 2;
            /// mmap the fd
            const MAP = 1 << 3;
            /// don't close fd
            const DONT_CLOSE = 1 << 4;
            /// don't notify events
            const DONT_NOTIFY = 1 << 5;
            /// the fd can not be mmapped
            const UNMAPPABLE = 1 << 6;

            const READWRITE = Self::READABLE.bits() | Self::WRITABLE.bits();
        }
    }

    /// This event is emitted when first bound to the core or when the
    /// hello method is called.
    #[derive(Debug, Clone)]
    pub struct Info {
        pub id: u32,
        pub cookie: u32,
        pub user_name: String,
        pub host_name: String,
        pub version: String,
        pub name: String,
        pub change_mask: ChangeMask,
        pub properties: PwDictionary,
    }

    impl EventDeserialize for Info {
        const OPCODE: u8 = 0;

        fn deserialize(
            pod: &mut pod::PodDeserializer,
            fds: &[RawFd],
        ) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_u32()?,
                cookie: pod.pop_field()?.as_u32()?,
                user_name: pod.pop_field()?.as_str()?.to_string(),
                host_name: pod.pop_field()?.as_str()?.to_string(),
                version: pod.pop_field()?.as_str()?.to_string(),
                name: pod.pop_field()?.as_str()?.to_string(),
                change_mask: ChangeMask::from_bits_retain(pod.pop_field()?.as_u64()?),
                properties: parse_dict(&mut pod.pop_field()?.as_struct()?)?,
            })
        }
    }

    /// The done event is emitted as a result of a sync method with the
    /// same seq number.
    #[derive(Debug, Clone)]
    pub struct Done {
        pub id: Option<u32>,
        pub seq: i32,
    }

    impl EventDeserialize for Done {
        const OPCODE: u8 = 1;

        fn deserialize(
            pod: &mut pod::PodDeserializer,
            fds: &[RawFd],
        ) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: {
                    let id = pod.pop_field()?.as_u32()?;
                    // is SPA_ID_INVALID
                    if id == u32::MAX {
                        None
                    } else {
                        Some(id)
                    }
                },
                seq: pod.pop_field()?.as_i32()?,
            })
        }
    }

    /// The client should reply with a pong reply with the same seq
    /// number.
    #[derive(Debug, Clone)]
    pub struct Ping {
        pub id: u32,
        pub seq: u32,
    }

    impl EventDeserialize for Ping {
        const OPCODE: u8 = 2;

        fn deserialize(
            pod: &mut pod::PodDeserializer,
            fds: &[RawFd],
        ) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_u32()?,
                seq: pod.pop_field()?.as_u32()?,
            })
        }
    }

    /// Fatal error event
    ///
    /// The error event is sent out when a fatal (non-recoverable)
    /// error has occurred. The `id` is the object where
    /// the error occurred, most often in response to a request to that
    /// object. The message is a brief description of the error,
    /// for (debugging) convenience.
    #[derive(Debug, Clone)]
    pub struct Error {
        pub id: u32,
        pub seq: i32,
        pub res: i32,
        pub message: String,
    }

    impl EventDeserialize for Error {
        const OPCODE: u8 = 3;

        fn deserialize(
            pod: &mut pod::PodDeserializer,
            fds: &[RawFd],
        ) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_u32()?,
                seq: pod.pop_field()?.as_i32()?,
                res: pod.pop_field()?.as_i32()?,
                message: pod.pop_field()?.as_str()?.to_string(),
            })
        }
    }

    /// This event is used by the object ID management
    /// logic. When a client deletes an object, the server will send
    /// this event to acknowledge that it has seen the delete request.
    /// When the client receives this event, it will know that it can
    /// safely reuse the object ID.
    #[derive(Debug, Clone)]
    pub struct RemoveId {
        pub id: u32,
    }

    impl EventDeserialize for RemoveId {
        const OPCODE: u8 = 4;

        fn deserialize(
            pod: &mut pod::PodDeserializer,
            fds: &[RawFd],
        ) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_u32()?,
            })
        }
    }

    /// This event is emitted when a local object ID is bound to a
    /// global ID. It is emitted before the global becomes visible in the
    /// registry.
    #[derive(Debug, Clone)]
    pub struct BoundId {
        pub id: u32,
        pub global_id: u32,
    }

    impl EventDeserialize for BoundId {
        const OPCODE: u8 = 5;

        fn deserialize(
            pod: &mut pod::PodDeserializer,
            fds: &[RawFd],
        ) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_u32()?,
                global_id: pod.pop_field()?.as_u32()?,
            })
        }
    }

    /// Add memory for a client
    ///
    /// Memory is given to a client as `fd` of a certain
    /// memory `type`.
    ///
    /// Further references to this fd will be made with the per memory\nunique identifier `id`.
    #[derive(Debug, Clone)]
    pub struct AddMem {
        pub id: u32,
        pub ty: SpaEnum<SpaDataType>,
        pub fd: Fd,
        pub flags: MemblockFlags,
    }

    impl EventDeserialize for AddMem {
        const OPCODE: u8 = 6;

        fn deserialize(
            pod: &mut pod::PodDeserializer,
            fds: &[RawFd],
        ) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_u32()?,
                ty: SpaEnum::from_raw(pod.pop_field()?.as_id()?),
                fd: {
                    let id = pod.pop_field()?.as_fd()?;
                    Fd {
                        id,
                        fd: fds.get(id as usize).copied(),
                    }
                },
                flags: MemblockFlags::from_bits_retain(pod.pop_field()?.as_u32()?),
            })
        }
    }

    /// Remove memory for a client
    #[derive(Debug, Clone)]
    pub struct RemoveMem {
        pub id: u32,
    }

    impl EventDeserialize for RemoveMem {
        const OPCODE: u8 = 7;

        fn deserialize(
            pod: &mut pod::PodDeserializer,
            fds: &[RawFd],
        ) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_u32()?,
            })
        }
    }

    /// This event is emitted when a local object ID is bound to a global ID. It is emitted before the global becomes visible in the registry.
    #[derive(Debug, Clone)]
    pub struct BoundProps {
        /// Proxy id
        pub id: u32,
        /// The global_id as it will appear in the registry.
        pub global_id: u32,
        /// The properties of the global
        pub properties: PwDictionary,
    }

    impl EventDeserialize for BoundProps {
        const OPCODE: u8 = 8;

        fn deserialize(
            pod: &mut pod::PodDeserializer,
            fds: &[RawFd],
        ) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_u32()?,
                global_id: pod.pop_field()?.as_u32()?,
                properties: parse_dict(&mut pod.pop_field()?.as_struct()?)?,
            })
        }
    }
}

#[derive(Debug, Clone, pod_derive::EventDeserialize)]
pub enum Event {
    /// This event is emitted when first bound to the core or when the hello method is called.
    Info(events::Info),
    /// The done event is emitted as a result of a sync method with the same seq number.
    Done(events::Done),
    /// The client should reply with a pong reply with the same seq number.
    Ping(events::Ping),
    /// Fatal error event
    /// The error event is sent out when a fatal (non-recoverable)
    /// error has occurred.
    Error(events::Error),
    /// This event is used by the object ID management
    /// logic. When a client deletes an object, the server will send this event to acknowledge that it has seen the delete request.
    /// When the client receives this event, it will know that it can\nsafely reuse the object ID.
    RemoveId(events::RemoveId),
    /// This event is emitted when a local object ID is bound to a\nglobal ID. It is emitted before the global becomes visible in the
    /// registry.
    BoundId(events::BoundId),
    /// Add memory for a client
    ///
    /// Memory is given to a client as `fd` of a certain memory `type`.
    ///
    /// Further references to this fd will be made with the per memory\nunique identifier `id`.
    AddMem(events::AddMem),
    /// Remove memory for a client
    RemoveMem(events::RemoveMem),
    /// This event is emitted when a local object ID is bound to a global ID. It is emitted before the global becomes visible in the registry.
    BoundProps(events::BoundProps),
}

impl HasInterface for Event {
    const INTERFACE: &'static str = "Core";
}
