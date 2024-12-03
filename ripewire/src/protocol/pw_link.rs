use super::*;

pub mod methods {
    use super::*;

    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct AddListener {}

    impl HasOpCode for AddListener {
        const OPCODE: u8 = 0;
    }
}

pub use events::ChangeMask;
pub mod events {
    use super::*;

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct ChangeMask: u64 {
            const STATE = 1 << 0;
            const FORMAT = 1 << 1;
            const PROPS = 1 << 3;
        }
    }

    /// Notify link info
    ///
    /// info - info about the link
    #[derive(Debug, Clone)]
    pub struct Info {
        pub id: u32,
        pub output_node_id: u32,
        pub output_port_id: u32,
        pub input_node_id: u32,
        pub input_port_id: u32,
        pub change_mask: ChangeMask,
        pub state: SpaEnum<libspa_consts::PwLinkState, i32>,
        pub error: Option<String>,
        /// Pod bytes
        pub format: OwnedPod,
        pub props: HashMap<String, String>,
    }

    impl EventDeserialize for Info {
        const OPCODE: u8 = 0;

        fn deserialize(
            pod: &mut PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_u32()?,
                output_node_id: pod.pop_field()?.as_u32()?,
                output_port_id: pod.pop_field()?.as_u32()?,
                input_node_id: pod.pop_field()?.as_u32()?,
                input_port_id: pod.pop_field()?.as_u32()?,
                change_mask: ChangeMask::from_bits_retain(pod.pop_field()?.as_u64()?),
                state: SpaEnum::from_i32(pod.pop_field()?.as_i32()?),
                error: pod.pop_field()?.as_str_or_none()?.map(ToString::to_string),
                format: pod.pop_field()?.to_owned(),
                props: parse_dict(&mut pod.pop_field()?.as_struct()?)?,
            })
        }
    }
}

#[derive(Debug, Clone, pod_derive::EventDeserialize)]
pub enum Event {
    /// Notify link info
    Info(events::Info),
}

impl HasInterface for Event {
    const INTERFACE: &'static str = "Link";
}
