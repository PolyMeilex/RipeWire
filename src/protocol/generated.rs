pub trait HasOpCode {
    const OPCODE: u8;
}
pub mod pw_core {
    use super::*;
    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct ChangeMask: u64 {
            const PROPS = 1;
        }
    }
    impl<'de> pod::deserialize::PodDeserialize<'de> for ChangeMask {
        fn deserialize(
            deserializer: pod::deserialize::PodDeserializer<'de>,
        ) -> Result<
            (Self, pod::deserialize::DeserializeSuccess<'de>),
            pod::deserialize::DeserializeError<&'de [u8]>,
        >
        where
            Self: Sized,
        {
            struct TestVisitor;
            impl<'de> pod::deserialize::Visitor<'de> for TestVisitor {
                type Value = ChangeMask;
                type ArrayElem = std::convert::Infallible;
                fn visit_long(
                    &self,
                    v: i64,
                ) -> Result<Self::Value, pod::deserialize::DeserializeError<&'de [u8]>>
                {
                    Ok(Self::Value::from_bits_retain(v as u64))
                }
            }
            deserializer.deserialize_long(TestVisitor)
        }
    }

    pub mod methods {
        use super::*;
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct AddListener {}
        impl HasOpCode for AddListener {
            const OPCODE: u8 = 0;
        }
        #[doc = "Start a conversation with the server. This will send\nthe core info and will destroy all resources for the client\n(except the core and client resource)."]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct Hello {
            pub version: u32,
        }
        impl HasOpCode for Hello {
            const OPCODE: u8 = 1;
        }
        #[doc = "Do server roundtrip\n\nAsk the server to emit the 'done' event with \\a seq.\n\nSince methods are handled in-order and events are delivered\nin-order, this can be used as a barrier to ensure all previous\nmethods and the resulting events have been handled.\n\nseq - the seq number passed to the done event"]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct Sync {
            pub id: u32,
            pub seq: i32,
        }
        impl HasOpCode for Sync {
            const OPCODE: u8 = 2;
        }
        #[doc = "Reply to a server ping event.\n\nReply to the server ping event with the same seq.\n\nseq - the seq number received in the ping event"]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct Pong {
            pub id: u32,
            pub seq: i32,
        }
        impl HasOpCode for Pong {
            const OPCODE: u8 = 3;
        }
        #[doc = "Fatal error event\n\nThe error method is sent out when a fatal (non-recoverable)\nerror has occurred. The id argument is the proxy object where\nthe error occurred, most often in response to an event on that\nobject. The message is a brief description of the error,\nfor (debugging) convenience.\n\nThis method is usually also emitted on the resource object with\nid.\n\nid - object where the error occurred\nres - error code\nmessage - error description"]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct Error {
            pub id: u32,
            pub seq: i32,
            pub res: i32,
            pub error: String,
        }
        impl HasOpCode for Error {
            const OPCODE: u8 = 4;
        }
        #[doc = "Get the registry object\n\nCreate a registry object that allows the client to list and bind\nthe global objects available from the PipeWire server\n\nversion - the client version\nuser_data_size - extra size"]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct GetRegistry {
            pub version: u32,
            pub new_id: u32,
        }
        impl HasOpCode for GetRegistry {
            const OPCODE: u8 = 5;
        }
        #[doc = "Create a new object on the PipeWire server from a factory.\n\nfactory_name - the factory name to use\nobj_type - the interface to bind to\nversion - the version of the interface\nproperties - extra properties"]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct CreateObject {
            pub factory_name: String,
            pub obj_type: String,
            pub version: u32,
            pub properties: pod::dictionary::Dictionary,
            pub new_id: u32,
        }
        impl HasOpCode for CreateObject {
            const OPCODE: u8 = 6;
        }
        #[doc = "Destroy an resource\n\nDestroy the server resource\n\nid - id of object to destroy"]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct Destroy {
            pub id: u32,
        }
        impl HasOpCode for Destroy {
            const OPCODE: u8 = 7;
        }
    }
    pub mod events {
        use super::*;
        #[doc = "This event is emitted when first bound to the core or when the\nhello method is called."]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
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
        impl HasOpCode for Info {
            const OPCODE: u8 = 0;
        }
        #[doc = "The done event is emitted as a result of a sync method with the\nsame seq number."]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Done {
            pub id: u32,
            pub seq: i32,
        }
        impl HasOpCode for Done {
            const OPCODE: u8 = 1;
        }
        #[doc = "The client should reply with a pong reply with the same seq\nnumber."]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Ping {
            pub id: u32,
            pub seq: i32,
        }
        impl HasOpCode for Ping {
            const OPCODE: u8 = 2;
        }
        #[doc = "Fatal error event\n\nThe error event is sent out when a fatal (non-recoverable)\nerror has occurred. The `id` is the object where\nthe error occurred, most often in response to a request to that\nobject. The message is a brief description of the error,\nfor (debugging) convenience."]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Error {
            pub id: u32,
            pub seq: i32,
            pub res: u32,
            pub error: String,
        }
        impl HasOpCode for Error {
            const OPCODE: u8 = 3;
        }
        #[doc = "This event is used by the object ID management\nlogic. When a client deletes an object, the server will send\nthis event to acknowledge that it has seen the delete request.\nWhen the client receives this event, it will know that it can\nsafely reuse the object ID."]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct RemoveId {
            pub id: u32,
        }
        impl HasOpCode for RemoveId {
            const OPCODE: u8 = 4;
        }
        #[doc = "This event is emitted when a local object ID is bound to a\nglobal ID. It is emitted before the global becomes visible in the\nregistry."]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct BoundId {
            pub id: u32,
            pub global_id: u32,
        }
        impl HasOpCode for BoundId {
            const OPCODE: u8 = 5;
        }
        #[doc = "Add memory for a client\n\nMemory is given to a client as `fd` of a certain\nmemory `type`.\n\nFurther references to this fd will be made with the per memory\nunique identifier `id`."]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct AddMem {
            pub id: u32,
            pub ty: pod::utils::Id,
            pub fd: pod::utils::Fd,
            pub flags: u32,
        }
        impl HasOpCode for AddMem {
            const OPCODE: u8 = 6;
        }
        #[doc = "Remove memory for a client"]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct RemoveMem {
            pub id: u32,
        }
        impl HasOpCode for RemoveMem {
            const OPCODE: u8 = 7;
        }
    }
    #[derive(Debug, Clone, pod_derive :: EventDeserialize)]
    pub enum Event {
        #[doc = "This event is emitted when first bound to the core or when the\nhello method is called."]
        Info(events::Info),
        #[doc = "The done event is emitted as a result of a sync method with the\nsame seq number."]
        Done(events::Done),
        #[doc = "The client should reply with a pong reply with the same seq\nnumber."]
        Ping(events::Ping),
        #[doc = "Fatal error event\n\nThe error event is sent out when a fatal (non-recoverable)\nerror has occurred. The `id` is the object where\nthe error occurred, most often in response to a request to that\nobject. The message is a brief description of the error,\nfor (debugging) convenience."]
        Error(events::Error),
        #[doc = "This event is used by the object ID management\nlogic. When a client deletes an object, the server will send\nthis event to acknowledge that it has seen the delete request.\nWhen the client receives this event, it will know that it can\nsafely reuse the object ID."]
        RemoveId(events::RemoveId),
        #[doc = "This event is emitted when a local object ID is bound to a\nglobal ID. It is emitted before the global becomes visible in the\nregistry."]
        BoundId(events::BoundId),
        #[doc = "Add memory for a client\n\nMemory is given to a client as `fd` of a certain\nmemory `type`.\n\nFurther references to this fd will be made with the per memory\nunique identifier `id`."]
        AddMem(events::AddMem),
        #[doc = "Remove memory for a client"]
        RemoveMem(events::RemoveMem),
    }
}

pub mod pw_client {
    use super::*;
    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct ChangeMask: u64 {
            const PROPS = 1;
        }
    }
    impl<'de> pod::deserialize::PodDeserialize<'de> for ChangeMask {
        fn deserialize(
            deserializer: pod::deserialize::PodDeserializer<'de>,
        ) -> Result<
            (Self, pod::deserialize::DeserializeSuccess<'de>),
            pod::deserialize::DeserializeError<&'de [u8]>,
        >
        where
            Self: Sized,
        {
            struct TestVisitor;
            impl<'de> pod::deserialize::Visitor<'de> for TestVisitor {
                type Value = ChangeMask;
                type ArrayElem = std::convert::Infallible;
                fn visit_long(
                    &self,
                    v: i64,
                ) -> Result<Self::Value, pod::deserialize::DeserializeError<&'de [u8]>>
                {
                    Ok(Self::Value::from_bits_retain(v as u64))
                }
            }
            deserializer.deserialize_long(TestVisitor)
        }
    }

    pub mod methods {
        use super::*;
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct AddListener {}
        impl HasOpCode for AddListener {
            const OPCODE: u8 = 0;
        }
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct Error {
            pub id: u32,
            pub res: u32,
            pub error: String,
        }
        impl HasOpCode for Error {
            const OPCODE: u8 = 1;
        }
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct UpdateProperties {
            pub properties: pod::dictionary::Dictionary,
        }
        impl HasOpCode for UpdateProperties {
            const OPCODE: u8 = 2;
        }
    }
    pub mod events {
        use super::*;
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Info {
            pub id: u32,
            pub change_mask: ChangeMask,
            pub properties: pod::dictionary::Dictionary,
        }
        impl HasOpCode for Info {
            const OPCODE: u8 = 0;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Permissions {}
        impl HasOpCode for Permissions {
            const OPCODE: u8 = 1;
        }
    }
    #[derive(Debug, Clone, pod_derive :: EventDeserialize)]
    pub enum Event {
        Info(events::Info),
        Permissions(events::Permissions),
    }
}

pub mod pw_device {
    use super::*;
    pub mod methods {
        use super::*;
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct AddListener {}
        impl HasOpCode for AddListener {
            const OPCODE: u8 = 0;
        }
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct SubscribeParams {}
        impl HasOpCode for SubscribeParams {
            const OPCODE: u8 = 1;
        }
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct EnumParams {}
        impl HasOpCode for EnumParams {
            const OPCODE: u8 = 2;
        }
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct SetParam {
            pub id: pod::utils::Id,
            pub flags: i32,
            pub param: pod::Value,
        }
        impl HasOpCode for SetParam {
            const OPCODE: u8 = 3;
        }
    }
    pub mod events {
        use super::*;
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Info {}
        impl HasOpCode for Info {
            const OPCODE: u8 = 0;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Param {}
        impl HasOpCode for Param {
            const OPCODE: u8 = 1;
        }
    }
    #[derive(Debug, Clone, pod_derive :: EventDeserialize)]
    pub enum Event {
        Info(events::Info),
        Param(events::Param),
    }
}

pub mod pw_node {
    use super::*;
    pub mod methods {
        use super::*;
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct AddListener {}
        impl HasOpCode for AddListener {
            const OPCODE: u8 = 0;
        }
    }
    pub mod events {
        use super::*;
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Info {}
        impl HasOpCode for Info {
            const OPCODE: u8 = 0;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Param {}
        impl HasOpCode for Param {
            const OPCODE: u8 = 1;
        }
    }
    #[derive(Debug, Clone, pod_derive :: EventDeserialize)]
    pub enum Event {
        Info(events::Info),
        Param(events::Param),
    }
}

pub mod pw_client_node {
    use super::*;
    pub mod methods {
        use super::*;
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct AddListener {}
        impl HasOpCode for AddListener {
            const OPCODE: u8 = 0;
        }
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct GetNode {}
        impl HasOpCode for GetNode {
            const OPCODE: u8 = 1;
        }
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct Update {
            pub change_mask: i32,
            pub n_params: i32,
            pub info: pod::Value,
        }
        impl HasOpCode for Update {
            const OPCODE: u8 = 2;
        }
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct PortUpdate {
            pub direction: i32,
            pub port_id: i32,
            pub change_mask: i32,
            pub n_params: i32,
            pub info: pod::Value,
        }
        impl HasOpCode for PortUpdate {
            const OPCODE: u8 = 3;
        }
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct SetActive {
            pub active: bool,
        }
        impl HasOpCode for SetActive {
            const OPCODE: u8 = 4;
        }
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct Event {}
        impl HasOpCode for Event {
            const OPCODE: u8 = 5;
        }
    }
    pub mod events {
        use super::*;
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Transport {}
        impl HasOpCode for Transport {
            const OPCODE: u8 = 0;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct SetParam {}
        impl HasOpCode for SetParam {
            const OPCODE: u8 = 1;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct SetIo {}
        impl HasOpCode for SetIo {
            const OPCODE: u8 = 2;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Event {}
        impl HasOpCode for Event {
            const OPCODE: u8 = 3;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Command {}
        impl HasOpCode for Command {
            const OPCODE: u8 = 4;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct AddPort {}
        impl HasOpCode for AddPort {
            const OPCODE: u8 = 5;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct RemovePort {}
        impl HasOpCode for RemovePort {
            const OPCODE: u8 = 6;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct PortSetParam {}
        impl HasOpCode for PortSetParam {
            const OPCODE: u8 = 7;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct PortUseBuffers {}
        impl HasOpCode for PortUseBuffers {
            const OPCODE: u8 = 8;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct PortSetIo {}
        impl HasOpCode for PortSetIo {
            const OPCODE: u8 = 9;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct SetActivation {}
        impl HasOpCode for SetActivation {
            const OPCODE: u8 = 10;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct PortSetMixInfo {}
        impl HasOpCode for PortSetMixInfo {
            const OPCODE: u8 = 11;
        }
    }
    #[derive(Debug, Clone, pod_derive :: EventDeserialize)]
    pub enum Event {
        Transport(events::Transport),
        SetParam(events::SetParam),
        SetIo(events::SetIo),
        Event(events::Event),
        Command(events::Command),
        AddPort(events::AddPort),
        RemovePort(events::RemovePort),
        PortSetParam(events::PortSetParam),
        PortUseBuffers(events::PortUseBuffers),
        PortSetIo(events::PortSetIo),
        SetActivation(events::SetActivation),
        PortSetMixInfo(events::PortSetMixInfo),
    }
}

pub mod pw_registry {
    use super::*;
    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct Permission: u32 {
            const R = 0o400;
            const W = 0o200;
            const X = 0o100;
            const M = 0o010;
        }
    }
    impl<'de> pod::deserialize::PodDeserialize<'de> for Permission {
        fn deserialize(
            deserializer: pod::deserialize::PodDeserializer<'de>,
        ) -> Result<
            (Self, pod::deserialize::DeserializeSuccess<'de>),
            pod::deserialize::DeserializeError<&'de [u8]>,
        >
        where
            Self: Sized,
        {
            struct TestVisitor;
            impl<'de> pod::deserialize::Visitor<'de> for TestVisitor {
                type Value = Permission;
                type ArrayElem = std::convert::Infallible;
                fn visit_int(
                    &self,
                    v: i32,
                ) -> Result<Self::Value, pod::deserialize::DeserializeError<&'de [u8]>>
                {
                    Ok(Self::Value::from_bits_retain(v as u32))
                }
            }
            deserializer.deserialize_int(TestVisitor)
        }
    }

    pub mod methods {
        use super::*;
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct Bind {
            pub id: u32,
            pub obj_type: String,
            pub version: u32,
            pub new_id: u32,
        }
        impl HasOpCode for Bind {
            const OPCODE: u8 = 0;
        }
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct Destroy {
            pub id: u32,
        }
        impl HasOpCode for Destroy {
            const OPCODE: u8 = 1;
        }
    }
    pub mod events {
        use super::*;
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Global {
            pub id: u32,
            pub permissions: Permission,
            pub obj_type: String,
            pub version: u32,
            pub properties: pod::dictionary::Dictionary,
        }
        impl HasOpCode for Global {
            const OPCODE: u8 = 0;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct GlobalRemove {
            pub id: u32,
        }
        impl HasOpCode for GlobalRemove {
            const OPCODE: u8 = 1;
        }
    }
    #[derive(Debug, Clone, pod_derive :: EventDeserialize)]
    pub enum Event {
        Global(events::Global),
        GlobalRemove(events::GlobalRemove),
    }
}
