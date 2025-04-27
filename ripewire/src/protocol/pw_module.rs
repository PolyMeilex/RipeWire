use super::*;

pub mod methods {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct AddListener {}

    impl MethodSerialize for AddListener {
        const OPCODE: u8 = 0;
        fn serialize(&self, buf: impl Write + Seek) {
            unreachable!()
        }
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

    /// Notify module info
    ///
    /// info - info about the module
    #[derive(Debug, Clone)]
    pub struct Info {
        pub id: u32,
        pub name: Option<String>,
        pub filename: Option<String>,
        pub args: Option<String>,
        pub change_mask: ChangeMask,
        pub props: HashMap<String, String>,
    }

    impl EventDeserialize for Info {
        const OPCODE: u8 = 0;

        fn deserialize(
            pod: &mut PodDeserializer,
            fds: &[RawFd],
        ) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_u32()?,
                name: pod.pop_field()?.as_str_or_none()?.map(ToString::to_string),
                filename: pod.pop_field()?.as_str_or_none()?.map(ToString::to_string),
                args: pod.pop_field()?.as_str_or_none()?.map(ToString::to_string),
                change_mask: ChangeMask::from_bits_retain(pod.pop_field()?.as_u64()?),
                props: parse_dict(&mut pod.pop_field()?.as_struct()?)?,
            })
        }
    }
}

#[derive(Debug, Clone, pod_derive::EventDeserialize)]
pub enum Event {
    /// Notify module info
    Info(events::Info),
}

impl HasInterface for Event {
    const INTERFACE: &'static str = "Module";
}
