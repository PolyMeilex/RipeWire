use std::os::fd::RawFd;

use super::HasOpCode;

pub const OBJECT_ID: u32 = 0;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, pod_derive::PodBitflagDeserialize)]
    pub struct ChangeMask: u64 {
        const PROPS = 1;
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, pod_derive::PodBitflagDeserialize)]
    pub struct MemblockFlags: u32 {
        const READABLE = 1;
        const WRITABLE = 2;
        const SEAL = 4;
        const MAP = 8;
        const DONT_CLOSE = 16;
        const DONT_NOTIFY = 32;

        const READWRITE = Self::READABLE.bits() | Self::WRITABLE.bits();
    }
}

pub mod methods {
    use super::*;

    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    #[op_code(0)]
    pub struct AddListener {}

    /// Start a conversation with the server. This will send
    /// the core info and will destroy all resources for the client
    /// (except the core and client resource).
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    #[op_code(1)]
    pub struct Hello {
        pub version: u32,
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
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    #[op_code(2)]
    pub struct Sync {
        pub id: u32,
        pub seq: i32,
    }

    /// Reply to a server ping event.
    ///
    /// Reply to the server ping event with the same seq.
    ///
    /// seq - the seq number received in the ping event
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    #[op_code(3)]
    pub struct Pong {
        pub id: u32,
        pub seq: i32,
    }

    /// Fatal error event
    ///
    /// The error method is sent out when a fatal (non-recoverable)
    /// error has occurred. The id argument is the proxy object where
    /// the error occurred, most often in response to an event on that
    /// object. The message is a brief description of the error,
    /// for (debugging) convenience.
    ///
    /// This method is usually also emitted on the resource object with
    /// id.
    ///
    /// id - object where the error occurred
    /// res - error code
    /// message - error description
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    #[op_code(4)]
    pub struct Error {
        pub id: u32,
        pub seq: i32,
        pub res: i32,
        pub error: String,
    }

    /// Get the registry object
    ///
    /// Create a registry object that allows the client to list and bind
    /// the global objects available from the PipeWire server
    ///
    /// version - the client version
    /// user_data_size - extra size
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    #[op_code(5)]
    pub struct GetRegistry {
        pub version: u32,
        pub new_id: u32,
    }

    /// Create a new object on the PipeWire server from a factory.
    ///
    /// factory_name - the factory name to use
    /// obj_type - the interface to bind to
    /// version - the version of the interface
    /// properties - extra properties
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    #[op_code(6)]
    pub struct CreateObject {
        pub factory_name: String,
        pub obj_type: String,
        pub version: u32,
        pub properties: pod::dictionary::Dictionary,
        pub new_id: u32,
    }

    /// Destroy an resource
    ///
    /// Destroy the server resource
    ///
    /// id - id of object to destroy
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    #[op_code(7)]
    pub struct Destroy {
        pub id: u32,
    }
}

pub mod events {
    use super::*;

    /// This event is emitted when first bound to the core or when the
    /// hello method is called.
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    #[op_code(0)]
    pub struct Info {
        pub id: u32,
        pub cookie: u32,
        pub user_name: String,
        pub host_name: String,
        pub version: String,
        pub name: String,
        pub change_mask: ChangeMask,
        pub properties: pod::dictionary::Dictionary,
    }

    /// The done event is emitted as a result of a sync method with the
    /// same seq number.
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    #[op_code(1)]
    pub struct Done {
        pub id: u32,
        pub seq: i32,
    }

    /// The client should reply with a pong reply with the same seq
    /// number.
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    #[op_code(2)]
    pub struct Ping {
        pub id: u32,
        pub seq: i32,
    }

    /// Fatal error event
    ///
    /// The error event is sent out when a fatal (non-recoverable)
    /// error has occurred. The `id` is the object where
    /// the error occurred, most often in response to a request to that
    /// object. The message is a brief description of the error,
    /// for (debugging) convenience.
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    #[op_code(3)]
    pub struct Error {
        pub id: u32,
        pub seq: i32,
        pub res: u32,
        pub error: String,
    }

    /// This event is used by the object ID management
    /// logic. When a client deletes an object, the server will send
    /// this event to acknowledge that it has seen the delete request.
    /// When the client receives this event, it will know that it can
    /// safely reuse the object ID.
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    #[op_code(4)]
    pub struct RemoveId {
        pub id: u32,
    }

    /// This event is emitted when a local object ID is bound to a
    /// global ID. It is emitted before the global becomes visible in the
    /// registry.
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    #[op_code(5)]
    pub struct BoundId {
        pub id: u32,
        pub global_id: u32,
    }

    /// Add memory for a client
    ///
    /// Memory is given to a client as `fd` of a certain
    /// memory `type`.
    ///
    /// Further references to this fd will be made with the per memory\nunique identifier `id`.
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    #[op_code(6)]
    pub struct AddMem {
        pub id: u32,
        pub ty: pod::utils::Id,
        #[fd]
        pub fd: pod::utils::Fd,
        pub flags: MemblockFlags,
    }

    /// Remove memory for a client
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    #[op_code(7)]
    pub struct RemoveMem {
        pub id: u32,
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
}
