use super::*;

pub mod methods {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct AddListener {}

    impl MethodSerializeSimple for AddListener {
        const OPCODE: u8 = 0;
        fn serialize_simple(&self, buf: impl Write + Seek) {
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
    /// This requires X permissions on the node.
    #[derive(Debug, Clone)]
    pub struct SubscribeParams {
        pub ids: Vec<Id>,
    }

    impl MethodSerializeSimple for SubscribeParams {
        const OPCODE: u8 = 1;
        fn serialize_simple(&self, buf: impl Write + Seek) {
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
    /// This requires X permissions on the node.
    #[derive(Debug, Clone)]
    pub struct EnumParams {
        pub seq: i32,
        pub id: Id,
        pub index: u32,
        pub num: u32,
        pub filter: pod::serialize::OwnedPod,
    }

    impl MethodSerializeSimple for EnumParams {
        const OPCODE: u8 = 2;
        fn serialize_simple(&self, buf: impl Write + Seek) {
            pod::Builder::new(buf).push_struct_with(|b| {
                b.write_i32(self.seq);
                b.write_id(self.id);
                b.write_u32(self.index);
                b.write_u32(self.num);
                b.write_pod(&self.filter);
            });
        }
    }

    /// Set a parameter on the node
    ///
    /// id - the parameter id to set
    /// flags - extra parameter flags
    /// param - the parameter to set
    ///
    /// This requires W and X permissions on the node.
    #[derive(Debug, Clone)]
    pub struct SetParam {
        pub id: Id,
        pub flags: u32,
        pub param: pod::serialize::OwnedPod,
    }

    impl MethodSerializeSimple for SetParam {
        const OPCODE: u8 = 3;
        fn serialize_simple(&self, buf: impl Write + Seek) {
            pod::Builder::new(buf).push_struct_with(|b| {
                b.write_id(self.id);
                b.write_u32(self.flags);
                b.write_pod(&self.param);
            });
        }
    }

    /// Send a command to the node
    ///
    /// command - the command to send
    ///
    /// This requires X and W permissions on the node.
    #[derive(Debug, Clone)]
    pub struct SendCommand {
        pub command: pod::serialize::OwnedPod,
    }

    impl MethodSerializeSimple for SendCommand {
        const OPCODE: u8 = 4;
        fn serialize_simple(&self, buf: impl Write + Seek) {
            pod::Builder::new(buf).push_struct_with(|b| {
                b.write_pod(&self.command);
            });
        }
    }
}

pub use events::ChangeMask;
pub mod events {
    use libspa_consts::PwNodeState;

    use super::*;

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct ChangeMask: u64 {
            const INPUT_PORTS = 1 << 0;
            const OUTPUT_PORTS = 1 << 1;
            const STATE = 1 << 2;
            const PROPS = 1 << 3;
            const PARAMS = 1 << 4;
        }
    }

    /// Notify node info
    ///
    /// info - info about the node
    #[derive(Debug, Clone)]
    pub struct Info {
        pub id: u32,
        pub max_input_ports: u32,
        pub max_output_ports: u32,
        pub change_mask: ChangeMask,
        pub n_input_ports: u32,
        pub n_output_ports: u32,
        pub state: SpaEnum<PwNodeState, i32>,
        pub error: Option<String>,
        pub props: PwDictionary,
        pub params: Vec<ParamInfo>,
    }

    impl EventDeserialize for Info {
        const OPCODE: u8 = 0;

        fn deserialize(pod: &mut PodDeserializer, fds: &[RawFd]) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_u32()?,
                max_input_ports: pod.pop_field()?.as_u32()?,
                max_output_ports: pod.pop_field()?.as_u32()?,
                change_mask: ChangeMask::from_bits_retain(pod.pop_field()?.as_u64()?),
                n_input_ports: pod.pop_field()?.as_u32()?,
                n_output_ports: pod.pop_field()?.as_u32()?,
                state: SpaEnum::from_i32(pod.pop_field()?.as_id()? as i32),
                error: pod.pop_field()?.as_str_or_none()?.map(ToString::to_string),
                props: parse_dict(&mut pod.pop_field()?.as_struct()?)?,
                params: parse_params(&mut pod.pop_field()?.as_struct()?)?,
            })
        }
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
    /// Notify node info
    Info(events::Info),
    /// Notify a node param
    ///
    /// Event emitted as a result of the enum_params method.
    Param(events::Param),
}

impl HasInterface for Event {
    const INTERFACE: &'static str = "Node";
}
