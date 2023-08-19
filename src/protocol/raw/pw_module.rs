use super::*;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, pod_derive::PodBitflagDeserialize)]
    pub struct ChangeMask: u64 {
        const PROPS = 1;
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

    /// Notify module info
    ///
    /// info - info about the module
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct Info {
        pub id: u32,
        pub name: String,
        pub filename: String,
        pub args: String,
        pub change_mask: ChangeMask,
        pub props: pod::dictionary::Dictionary,
    }

    impl HasOpCode for Info {
        const OPCODE: u8 = 0;
    }
}

#[derive(Debug, Clone, pod_derive::EventDeserialize)]
pub enum Event {
    /// Notify module info
    Info(events::Info),
}
