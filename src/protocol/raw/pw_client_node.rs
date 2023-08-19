use super::*;

pub mod methods {
    use super::*;
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct AddListener {}

    impl HasOpCode for AddListener {
        const OPCODE: u8 = 0;
    }

    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct GetNode {}

    impl HasOpCode for GetNode {
        const OPCODE: u8 = 1;
    }

    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct Update {
        pub change_mask: i32,
        pub n_params: i32,
        pub info: pod::Value,
    }

    impl HasOpCode for Update {
        const OPCODE: u8 = 2;
    }

    #[derive(Debug, Clone, pod_derive::PodSerialize)]
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

    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct SetActive {
        pub active: bool,
    }

    impl HasOpCode for SetActive {
        const OPCODE: u8 = 4;
    }

    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct Event {}

    impl HasOpCode for Event {
        const OPCODE: u8 = 5;
    }
}

pub mod events {
    use super::*;

    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
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

    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct SetParam {}

    impl HasOpCode for SetParam {
        const OPCODE: u8 = 1;
    }

    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct SetIo {
        pub id: pod::utils::Id,
        pub memid: u32,
        pub off: u32,
        pub sz: u32,
    }

    impl HasOpCode for SetIo {
        const OPCODE: u8 = 2;
    }

    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct Event {}

    impl HasOpCode for Event {
        const OPCODE: u8 = 3;
    }

    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct Command {
        pub command: pod::Value,
    }

    impl HasOpCode for Command {
        const OPCODE: u8 = 4;
    }

    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct AddPort {}

    impl HasOpCode for AddPort {
        const OPCODE: u8 = 5;
    }

    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct RemovePort {}

    impl HasOpCode for RemovePort {
        const OPCODE: u8 = 6;
    }

    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
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

    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct PortUseBuffers {}

    impl HasOpCode for PortUseBuffers {
        const OPCODE: u8 = 8;
    }

    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
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

    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct SetActivation {}

    impl HasOpCode for SetActivation {
        const OPCODE: u8 = 10;
    }

    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct PortSetMixInfo {}

    impl HasOpCode for PortSetMixInfo {
        const OPCODE: u8 = 11;
    }
}

#[derive(Debug, Clone, pod_derive::EventDeserialize)]
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
