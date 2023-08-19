use super::*;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, pod_derive::PodBitflagDeserialize)]
    pub struct ChangeMask: u64 {
        const STATE = 1;
        const FORMAT = 2;
        const PROPS = 4;
    }
}

pub mod methods {
    use super::*;

    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct AddListener {}

    impl HasOpCode for AddListener {
        const OPCODE: u8 = 0;
    }
}

pub mod events {
    use super::*;

    /// Notify link info
    ///
    /// info - info about the link
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct Info {
        pub id: u32,
        pub output_node_id: u32,
        pub output_port_id: u32,
        pub input_node_id: u32,
        pub input_port_id: u32,
        pub change_mask: ChangeMask,
        pub state: u32,
        pub error: String,
        pub format: pod::Value,
        pub props: pod::dictionary::Dictionary,
    }
    impl HasOpCode for Info {
        const OPCODE: u8 = 0;
    }
}

#[derive(Debug, Clone, pod_derive::EventDeserialize)]
pub enum Event {
    /// Notify link info
    Info(events::Info),
}
