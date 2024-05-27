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
    #[derive(Clone)]
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
        pub format: Vec<u8>,
        pub props: HashMap<String, String>,
    }

    impl std::fmt::Debug for Info {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Info")
                .field("id", &self.id)
                .field("output_node_id", &self.output_node_id)
                .field("output_port_id", &self.output_port_id)
                .field("input_node_id", &self.input_node_id)
                .field("input_port_id", &self.input_port_id)
                .field("change_mask", &self.change_mask)
                .field("state", &self.state)
                .field("error", &self.error)
                .field("format", &"...")
                .field("props", &self.props)
                .finish()
        }
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
                error: {
                    let pod = pod.pop_field()?;
                    match pod.kind() {
                        pod_v2::PodDeserializerKind::None => None,
                        pod_v2::PodDeserializerKind::String(v) => Some(v.to_string()),
                        _ => {
                            return Err(pod_v2::deserialize::DeserializeError::UnexpectedType {
                                expected: libspa_consts::SpaType::String,
                                got: pod.ty(),
                            })
                        }
                    }
                },
                format: pod.pop_field()?.body().to_vec(),
                props: parse_dict(&mut pod.pop_field()?.as_struct()?)?,
            })
        }
    }
}

#[derive(Debug, Clone, pod_derive::EventDeserialize2)]
pub enum Event {
    /// Notify link info
    Info(events::Info),
}

impl HasInterface for Event {
    const INTERFACE: &'static str = "Link";
}
