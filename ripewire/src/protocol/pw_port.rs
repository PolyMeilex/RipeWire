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

    /// Subscribe to parameter changes
    ///
    /// Automatically emit param events for the given ids when they are changed.
    ///
    /// ids - an array of param ids
    /// n_ids - the number of ids in `ids`
    ///
    /// This requires X permissions on the port.
    #[derive(Debug, Clone)]
    pub struct SubscribeParams {
        pub ids: Vec<Id>,
    }

    impl MethodSerialize for SubscribeParams {
        const OPCODE: u8 = 1;
        fn serialize(&self, buf: impl Write + Seek) {
            pod::Builder::new(buf).push_struct_with(|b| {
                b.write_array_with(|b| {
                    for id in self.ids.iter() {
                        b.write_id(id);
                    }
                });
            });
        }
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
    /// This requires X permissions on the port.
    #[derive(Debug, Clone)]
    pub struct EnumParams {
        pub seq: i32,
        pub id: Id,
        pub index: u32,
        pub num: u32,
        pub filter: pod::serialize::OwnedPod,
    }

    impl MethodSerialize for EnumParams {
        const OPCODE: u8 = 2;
        fn serialize(&self, buf: impl Write + Seek) {
            pod::Builder::new(buf).push_struct_with(|b| {
                b.write_i32(self.seq);
                b.write_id(self.id);
                b.write_u32(self.index);
                b.write_u32(self.num);
                b.write_pod(&self.filter);
            });
        }
    }
}

pub use events::ChangeMask;
pub mod events {
    use super::*;
    use libspa_consts::SpaDirection;

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct ChangeMask: u64 {
            const PROPS = 1;
            const PARAMS = 2;
        }
    }

    /// Notify port info
    ///
    /// info - info about the port
    #[derive(Debug, Clone)]
    pub struct Info {
        pub id: u32,
        pub direction: SpaEnum<SpaDirection>,
        pub change_mask: ChangeMask,
        pub props: HashMap<String, String>,
        pub params: Vec<ParamInfo>,
    }

    impl EventDeserialize for Info {
        const OPCODE: u8 = 0;

        fn deserialize(pod: &mut PodDeserializer, fds: &[RawFd]) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_u32()?,
                direction: SpaEnum::from_raw(pod.pop_field()?.as_u32()?),
                change_mask: ChangeMask::from_bits_retain(pod.pop_field()?.as_u64()?),
                props: parse_dict(&mut pod.pop_field()?.as_struct()?)?,
                params: parse_params(&mut pod.pop_field()?.as_struct()?)?,
            })
        }
    }

    /// Notify a port param
    ///
    /// Event emitted as a result of the enum_params method.
    ///
    /// seq - the sequence number of the request
    /// id - the param id
    /// index - the param index
    /// next - the param index of the next param
    /// param - the parameter
    #[derive(Debug, Clone)]
    pub struct Param {
        pub seq: i32,
        pub id: SpaEnum<SpaParamType>,
        pub index: u32,
        pub next: u32,
        pub params: OwnedPod,
    }

    impl EventDeserialize for Param {
        const OPCODE: u8 = 1;

        fn deserialize(pod: &mut PodDeserializer, fds: &[RawFd]) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                seq: pod.pop_field()?.as_i32()?,
                id: SpaEnum::from_raw(pod.pop_field()?.as_id()?),
                index: pod.pop_field()?.as_u32()?,
                next: pod.pop_field()?.as_u32()?,
                params: pod.pop_field()?.to_owned(),
            })
        }
    }
}

#[derive(Debug, Clone, pod_derive::EventDeserialize)]
pub enum Event {
    /// Notify port info
    Info(events::Info),
    /// Notify a port param
    ///
    /// Event emitted as a result of the enum_params method.
    Param(events::Param),
}

impl HasInterface for Event {
    const INTERFACE: &'static str = "Port";
}
