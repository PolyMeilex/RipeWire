pub trait HasOpCode {
    const OPCODE: u8;
}
pub mod pw_core {
    use super::*;
    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy, pod_derive::PodBitflagDeserialize)]
        pub struct ChangeMask: u64 {
            const PROPS = 1;
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
        #[derive(Debug, Clone, Copy, pod_derive::PodBitflagDeserialize)]
        pub struct ChangeMask: u64 {
            const PROPS = 1;
        }
    }

    pub mod methods {
        use super::*;
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct AddListener {}
        impl HasOpCode for AddListener {
            const OPCODE: u8 = 0;
        }
        #[doc = "Send an error to a client\n\nid - the global id to report the error on\nram - res an errno style error code\nmessage - an error string\n\nThis requires W and X permissions on the client."]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct Error {
            pub id: u32,
            pub res: u32,
            pub error: String,
        }
        impl HasOpCode for Error {
            const OPCODE: u8 = 1;
        }
        #[doc = "Update client properties\n\nprops - new properties\n\nThis requires W and X permissions on the client."]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct UpdateProperties {
            pub properties: pod::dictionary::Dictionary,
        }
        impl HasOpCode for UpdateProperties {
            const OPCODE: u8 = 2;
        }
        #[doc = "Get client permissions\n\nA permissions event will be emitted with the permissions.\n\nindex - the first index to query, 0 for first\nnum - the maximum number of items to get\n\nThis requires W and X permissions on the client."]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct GetPermissions {
            pub index: u32,
            pub num: u32,
        }
        impl HasOpCode for GetPermissions {
            const OPCODE: u8 = 3;
        }
        #[doc = "Manage the permissions of the global objects for this client\n\nUpdate the permissions of the global objects using the provided array with permissions\n\nGlobals can use the default permissions or can have specific permissions assigned to them.\n\nn_permissions - number of permissions\npermissions - array of permissions\n\nThis requires W and X permissions on the client."]
        #[derive(Debug, Clone)]
        pub struct UpdatePermissions(pub pod::permissions::Permissions);
        impl pod::serialize::PodSerialize for UpdatePermissions {
            fn serialize<O: std::io::Write + std::io::Seek>(
                &self,
                serializer: pod::serialize::PodSerializer<O>,
            ) -> Result<pod::serialize::SerializeSuccess<O>, pod::serialize::GenError> {
                self.0.serialize(serializer)
            }
        }
        impl HasOpCode for UpdatePermissions {
            const OPCODE: u8 = 4;
        }
    }
    pub mod events {
        use super::*;
        #[doc = "Notify client info\n\ninfo - info about the client"]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Info {
            pub id: u32,
            pub change_mask: ChangeMask,
            pub properties: pod::dictionary::Dictionary,
        }
        impl HasOpCode for Info {
            const OPCODE: u8 = 0;
        }
        #[doc = "Notify a client permission\n\nEvent emitted as a result of the get_permissions method.\n\ndefault_permissions - the default permissions\nindex - the index of the first permission entry\nn_permissions - the number of permissions\npermissions - the permissions"]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Permissions {
            pub index: u32,
            pub permissions: pod::permissions::Permissions,
        }
        impl HasOpCode for Permissions {
            const OPCODE: u8 = 1;
        }
    }
    #[derive(Debug, Clone, pod_derive :: EventDeserialize)]
    pub enum Event {
        #[doc = "Notify client info\n\ninfo - info about the client"]
        Info(events::Info),
        #[doc = "Notify a client permission\n\nEvent emitted as a result of the get_permissions method.\n\ndefault_permissions - the default permissions\nindex - the index of the first permission entry\nn_permissions - the number of permissions\npermissions - the permissions"]
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
        #[doc = "Subscribe to parameter changes\n\nAutomatically emit param events for the given ids when they are changed.\n\nids - an array of param ids\nn_ids - the number of ids in `ids`\n\nThis requires X permissions on the device."]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct SubscribeParams {
            pub ids: pod::array::Array<pod::utils::Id>,
        }
        impl HasOpCode for SubscribeParams {
            const OPCODE: u8 = 1;
        }
        #[doc = "Enumerate device parameters\n\nStart enumeration of device parameters. For each param, a param event will be emitted.\n\nseq - a sequence number to place in the reply\nid - the parameter id to enum or PW_ID_ANY for all\nstart - the start index or 0 for the first param\nnum - the maximum number of params to retrieve\nfilter - a param filter or NULL\n\nThis requires X permissions on the device."]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct EnumParams {
            pub seq: i32,
            pub id: pod::utils::Id,
            pub index: u32,
            pub num: u32,
            pub filter: pod::Value,
        }
        impl HasOpCode for EnumParams {
            const OPCODE: u8 = 2;
        }
        #[doc = "Set a parameter on the device\n\nid - the parameter id to set\nflags - extra parameter flags\nparam - the parameter to set\n\nThis requires W and X permissions on the device."]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct SetParam {
            pub id: pod::utils::Id,
            pub flags: u32,
            pub param: pod::Value,
        }
        impl HasOpCode for SetParam {
            const OPCODE: u8 = 3;
        }
    }
    pub mod events {
        use super::*;
        #[doc = "Notify device info\n\ninfo - info about the device"]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Info {
            pub id: u32,
            pub change_mask: u64,
            pub props: pod::dictionary::Dictionary,
            pub params: pod::pod_struct::Struct,
        }
        impl HasOpCode for Info {
            const OPCODE: u8 = 0;
        }
        #[doc = "Notify a device param\n\nEvent emitted as a result of the enum_params method.\n\nseq - the sequence number of the request\nid - the param id\nindex - the param index\nnext - the param index of the next param\nparam - the parameter"]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Param {
            pub seq: i32,
            pub id: pod::utils::Id,
            pub index: u32,
            pub next: u32,
            pub params: pod::Value,
        }
        impl HasOpCode for Param {
            const OPCODE: u8 = 1;
        }
    }
    #[derive(Debug, Clone, pod_derive :: EventDeserialize)]
    pub enum Event {
        #[doc = "Notify device info\n\ninfo - info about the device"]
        Info(events::Info),
        #[doc = "Notify a device param\n\nEvent emitted as a result of the enum_params method.\n\nseq - the sequence number of the request\nid - the param id\nindex - the param index\nnext - the param index of the next param\nparam - the parameter"]
        Param(events::Param),
    }
}

pub mod pw_factory {
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
        #[doc = "Notify factory info\n\ninfo - info about the factory"]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Info {
            pub id: u32,
            pub name: String,
            pub obj_type: String,
            pub version: u32,
            pub change_mask: u64,
            pub props: pod::dictionary::Dictionary,
        }
        impl HasOpCode for Info {
            const OPCODE: u8 = 0;
        }
    }
    #[derive(Debug, Clone, pod_derive :: EventDeserialize)]
    pub enum Event {
        #[doc = "Notify factory info\n\ninfo - info about the factory"]
        Info(events::Info),
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
        #[doc = "Subscribe to parameter changes\n\nAutomatically emit param events for the given ids when they are changed.\n\nids - an array of param ids\nn_ids - the number of ids in `ids`\n\nThis requires X permissions on the device."]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct SubscribeParams {
            pub ids: pod::array::Array<pod::utils::Id>,
        }
        impl HasOpCode for SubscribeParams {
            const OPCODE: u8 = 1;
        }
        #[doc = "Enumerate device parameters\n\nStart enumeration of device parameters. For each param, a param event will be emitted.\n\nseq - a sequence number to place in the reply\nid - the parameter id to enum or PW_ID_ANY for all\nstart - the start index or 0 for the first param\nnum - the maximum number of params to retrieve\nfilter - a param filter or NULL\n\nThis requires X permissions on the device."]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct EnumParams {
            pub seq: i32,
            pub id: pod::utils::Id,
            pub index: u32,
            pub num: u32,
            pub filter: pod::Value,
        }
        impl HasOpCode for EnumParams {
            const OPCODE: u8 = 2;
        }
        #[doc = "Set a parameter on the device\n\nid - the parameter id to set\nflags - extra parameter flags\nparam - the parameter to set\n\nThis requires W and X permissions on the device."]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct SetParam {
            pub id: pod::utils::Id,
            pub flags: u32,
            pub param: pod::Value,
        }
        impl HasOpCode for SetParam {
            const OPCODE: u8 = 3;
        }
        #[doc = "Send a command to the node\n\ncommand - the command to send\n\nThis requires X and W permissions on the node."]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct SendCommand {
            pub command: pod::Value,
        }
        impl HasOpCode for SendCommand {
            const OPCODE: u8 = 4;
        }
    }
    pub mod events {
        use super::*;
        #[doc = "Notify node info\n\ninfo - info about the node"]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Info {
            pub id: u32,
            pub max_input_ports: u32,
            pub max_output_ports: u32,
            pub change_mask: u64,
            pub n_input_ports: u32,
            pub n_output_ports: u32,
            pub state: pod::utils::Id,
            pub error: String,
            pub props: pod::dictionary::Dictionary,
            pub params: pod::pod_struct::Struct,
        }
        impl HasOpCode for Info {
            const OPCODE: u8 = 0;
        }
        #[doc = "Notify a node param\n\nEvent emitted as a result of the enum_params method.\n\nseq - the sequence number of the request\nid - the param id\nindex - the param index\nnext - the param index of the next param\nparam - the parameter"]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Param {
            pub seq: i32,
            pub id: pod::utils::Id,
            pub index: u32,
            pub next: u32,
            pub params: pod::Value,
        }
        impl HasOpCode for Param {
            const OPCODE: u8 = 1;
        }
    }
    #[derive(Debug, Clone, pod_derive :: EventDeserialize)]
    pub enum Event {
        #[doc = "Notify node info\n\ninfo - info about the node"]
        Info(events::Info),
        #[doc = "Notify a node param\n\nEvent emitted as a result of the enum_params method.\n\nseq - the sequence number of the request\nid - the param id\nindex - the param index\nnext - the param index of the next param\nparam - the parameter"]
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
        pub struct Transport {
            pub readfd: pod::utils::Fd,
            pub writefd: pod::utils::Fd,
            pub memid: u32,
            pub offset: u32,
            pub size: u32,
        }
        impl HasOpCode for Transport {
            const OPCODE: u8 = 0;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct SetParam {}
        impl HasOpCode for SetParam {
            const OPCODE: u8 = 1;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct SetIo {
            pub id: pod::utils::Id,
            pub memid: u32,
            pub off: u32,
            pub sz: u32,
        }
        impl HasOpCode for SetIo {
            const OPCODE: u8 = 2;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Event {}
        impl HasOpCode for Event {
            const OPCODE: u8 = 3;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Command {
            pub command: pod::Value,
        }
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
        pub struct PortSetParam {
            pub direction: u32,
            pub port_id: u32,
            pub id: pod::utils::Id,
            pub flags: u32,
            pub param: pod::Value,
        }
        impl HasOpCode for PortSetParam {
            const OPCODE: u8 = 7;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct PortUseBuffers {}
        impl HasOpCode for PortUseBuffers {
            const OPCODE: u8 = 8;
        }
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct PortSetIo {
            pub direction: u32,
            pub port_id: u32,
            pub mix_id: u32,
            pub id: pod::utils::Id,
            pub memid: u32,
            pub off: u32,
            pub sz: u32,
        }
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
    pub mod methods {
        use super::*;
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct AddListener {}
        impl HasOpCode for AddListener {
            const OPCODE: u8 = 0;
        }
        #[doc = "Bind to a global object\n\nBind to the global object with \\a id and use the client proxy with new_id as the proxy. After this call, methods can be send to the remote global object and events can be received\n\nid - the global id to bind to\ntype - the interface type to bind to\nversion - the interface version to use"]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct Bind {
            pub id: u32,
            pub obj_type: String,
            pub version: u32,
            pub new_id: u32,
        }
        impl HasOpCode for Bind {
            const OPCODE: u8 = 1;
        }
        #[doc = "Attempt to destroy a global object\n\nTry to destroy the global object.\n\nid - the global id to destroy.\n\nThe client needs X permissions on the global."]
        #[derive(Debug, Clone, pod_derive :: PodSerialize)]
        pub struct Destroy {
            pub id: u32,
        }
        impl HasOpCode for Destroy {
            const OPCODE: u8 = 2;
        }
    }
    pub mod events {
        use super::*;
        #[doc = "Notify of a new global object\n\nThe registry emits this event when a new global object is available.\n\nid - the global object id\npermissions - the permissions of the object\ntype - the type of the interface\nversion - the version of the interface\nprops - extra properties of the global"]
        #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
        pub struct Global {
            pub id: u32,
            pub permissions: pod::permissions::PermissionFlags,
            pub obj_type: String,
            pub version: u32,
            pub properties: pod::dictionary::Dictionary,
        }
        impl HasOpCode for Global {
            const OPCODE: u8 = 0;
        }
        #[doc = "Notify of a global object removal\n\nEmitted when a global object was removed from the registry.\nIf the client has any bindings to the global, it should destroy those.\n\nid - the id of the global that was removed"]
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
        #[doc = "Notify of a new global object\n\nThe registry emits this event when a new global object is available.\n\nid - the global object id\npermissions - the permissions of the object\ntype - the type of the interface\nversion - the version of the interface\nprops - extra properties of the global"]
        Global(events::Global),
        #[doc = "Notify of a global object removal\n\nEmitted when a global object was removed from the registry.\nIf the client has any bindings to the global, it should destroy those.\n\nid - the id of the global that was removed"]
        GlobalRemove(events::GlobalRemove),
    }
}
