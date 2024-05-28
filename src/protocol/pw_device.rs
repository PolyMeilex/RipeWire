use super::*;

pub mod methods {
    use super::*;

    #[derive(Debug, Clone, pod_derive::PodSerialize)]
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
    /// This requires X permissions on the device.
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct SubscribeParams {
        pub ids: pod::array::Array<pod::utils::Id>,
    }

    impl HasOpCode for SubscribeParams {
        const OPCODE: u8 = 1;
    }

    /// Enumerate device parameters
    ///
    /// Start enumeration of device parameters. For each param, a param event will be emitted.
    ///
    /// seq - a sequence number to place in the reply
    /// id - the parameter id to enum or PW_ID_ANY for all
    /// start - the start index or 0 for the first param
    /// num - the maximum number of params to retrieve
    /// filter - a param filter or NULL
    ///
    /// This requires X permissions on the device.
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
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

    /// Set a parameter on the device
    ///
    /// id - the parameter id to set
    /// flags - extra parameter flags
    /// param - the parameter to set
    ///
    /// This requires W and X permissions on the device.
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct SetParam {
        pub id: pod::utils::Id,
        pub flags: u32,
        pub param: pod::Value,
    }

    impl HasOpCode for SetParam {
        const OPCODE: u8 = 3;
    }
}

pub use events::{ChangeMask, ParamFlags};
pub mod events {
    use super::*;

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct ChangeMask: u64 {
            const PROPS = 1 << 0;
            const PARAMS = 1 << 1;
        }
    }

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct ParamFlags: u32 {
            /// bit to signal update even when the
            /// read/write flags don't change
            const SERIAL = 1 << 0;
            const READ = 1 << 1;
            const WRITE = 1 << 2;
            const READWRITE = Self::WRITE.bits() | Self::READ.bits();
        }
    }

    #[derive(Debug, Clone)]
    pub struct ParamInfo {
        id: SpaEnum<SpaParamType>,
        flags: ParamFlags,
    }

    fn parse_params(pod: &mut PodDeserializer) -> pod_v2::deserialize::Result<Vec<ParamInfo>> {
        let mut pod = pod.as_struct()?;

        let len = pod.pop_field()?.as_i32()?;

        if len <= 0 {
            return Ok(Vec::new());
        }
        let len = len as u32;

        let mut params = Vec::with_capacity(len as usize);

        for _ in 0..len {
            let id = pod.pop_field()?.as_id()?;
            let flags = pod.pop_field()?.as_u32()?;
            params.push(ParamInfo {
                id: SpaEnum::from_raw(id),
                flags: ParamFlags::from_bits_retain(flags),
            });
        }

        Ok(params)
    }

    /// Notify device info
    ///
    /// info - info about the device
    #[derive(Debug, Clone)]
    pub struct Info {
        pub id: u32,
        pub change_mask: ChangeMask,
        pub props: HashMap<String, String>,
        pub params: Vec<ParamInfo>,
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
                change_mask: ChangeMask::from_bits_retain(pod.pop_field()?.as_u64()?),
                props: parse_dict(&mut pod.pop_field()?.as_struct()?)?,
                params: parse_params(&mut pod.pop_field()?)?,
            })
        }
    }

    /// Notify a device param
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
        /// Bytes of a spa object
        pub params: OwnedPod,
    }

    impl EventDeserialize for Param {
        const OPCODE: u8 = 1;

        fn deserialize(
            pod: &mut PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
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

#[derive(Debug, Clone, pod_derive::EventDeserialize2)]
pub enum Event {
    /// Notify device info
    Info(events::Info),

    /// Notify a device param
    ///
    /// Event emitted as a result of the enum_params method.
    Param(events::Param),
}

impl HasInterface for Event {
    const INTERFACE: &'static str = "Device";
}
