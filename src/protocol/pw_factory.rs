use super::*;

pub mod methods {
    use super::*;

    #[derive(Debug, Clone)]
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
            const PROPS = 1;
        }
    }

    /// Notify factory info
    ///
    /// info - info about the factory
    #[derive(Debug, Clone)]
    pub struct Info {
        pub id: u32,
        pub name: String,
        pub interface: String,
        pub version: u32,
        pub change_mask: ChangeMask,
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
                name: pod.pop_field()?.as_str()?.to_string(),
                interface: pod.pop_field()?.as_str()?.to_string(),
                version: pod.pop_field()?.as_u32()?,
                change_mask: ChangeMask::from_bits_retain(pod.pop_field()?.as_u64()?),
                props: parse_dict(&mut pod.pop_field()?.as_struct()?)?,
            })
        }
    }
}

#[derive(Debug, Clone, pod_derive::EventDeserialize)]
pub enum Event {
    /// Notify factory info
    Info(events::Info),
}

impl HasInterface for Event {
    const INTERFACE: &'static str = "Factory";
}
