use super::*;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, pod_derive::PodBitflagDeserialize)]
    pub struct ChangeMask: u64 {
        const INPUT_PORTS = 1;
        const OUTPUT_PORTS = 2;
        const STATE = 4;
        const PROPS = 8;
        const PARAMS = 16;
    }
}

pub mod methods {
    use super::*;

    #[derive(Debug, Clone, pod_derive :: PodSerialize)]
    pub struct AddListener {}

    impl HasOpCode for AddListener {
        const OPCODE: u8 = 0;
    }

    /// Subscribe to parameter changes
    ///
    /// Automatically emit param events for the given ids when they are changed.
    ///
    /// ids - an array of param ids
    /// n_ids - the number of ids in `ids`
    ///
    /// This requires X permissions on the node.
    #[derive(Debug, Clone, pod_derive :: PodSerialize)]
    pub struct SubscribeParams {
        pub ids: pod::array::Array<pod::utils::Id>,
    }

    impl HasOpCode for SubscribeParams {
        const OPCODE: u8 = 1;
    }

    /// Enumerate node parameters
    ///
    /// Start enumeration of node parameters. For each param, a param event will be emitted.
    ///
    /// seq - a sequence number to place in the reply
    /// id - the parameter id to enum or PW_ID_ANY for all
    /// start - the start index or 0 for the first param
    /// num - the maximum number of params to retrieve
    /// filter - a param filter or NULL
    ///
    /// This requires X permissions on the node.
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

    /// Set a parameter on the node
    ///
    /// id - the parameter id to set
    /// flags - extra parameter flags
    /// param - the parameter to set
    ///
    /// This requires W and X permissions on the node.
    #[derive(Debug, Clone, pod_derive :: PodSerialize)]
    pub struct SetParam {
        pub id: pod::utils::Id,
        pub flags: u32,
        pub param: pod::Value,
    }

    impl HasOpCode for SetParam {
        const OPCODE: u8 = 3;
    }

    /// Send a command to the node
    ///
    /// command - the command to send
    ///
    /// This requires X and W permissions on the node.
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct SendCommand {
        pub command: pod::Value,
    }

    impl HasOpCode for SendCommand {
        const OPCODE: u8 = 4;
    }
}

pub mod events {
    use super::*;

    /// Notify node info
    ///
    /// info - info about the node
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct Info {
        pub id: u32,
        pub max_input_ports: u32,
        pub max_output_ports: u32,
        pub change_mask: ChangeMask,
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

    /// Notify a node param
    ///
    /// Event emitted as a result of the enum_params method.
    ///
    /// seq - the sequence number of the request
    /// id - the param id
    /// index - the param index
    /// next - the param index of the next param
    /// param - the parameter
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
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
    /// Notify node info
    Info(events::Info),
    /// Notify a node param
    ///
    /// Event emitted as a result of the enum_params method.
    Param(events::Param),
}
