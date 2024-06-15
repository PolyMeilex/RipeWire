use super::*;
use libspa_consts::SpaDirection;

pub mod methods {
    use super::*;
    use bitflags::Flags;

    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct AddListener {}

    impl HasOpCode for AddListener {
        const OPCODE: u8 = 0;
    }

    /// Get the node object associated with the client-node.
    /// This binds to the server side Node object.
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct GetNode {
        /// The Node version to bind as
        pub version: u32,
        /// The proxy id
        pub new_id: u32,
    }

    impl HasOpCode for GetNode {
        const OPCODE: u8 = 1;
    }

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct NodeInfoChangeMask: u64 {
            const FLAGS = 1 << 0;
            const PROPS = 1 << 1;
            const PARAMS = 1 << 2;
        }
    }

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct NodeFlags: u64 {
            /// Node can do real-time processing
            const RT = 1 << 0;
            /// Input ports can be added/removed
            const IN_DYNAMIC_PORTS = 1 << 1;
            /// Output ports can be added/removed
            const OUT_DYNAMIC_PORTS = 1 << 2;
            /// Input ports can be reconfigured with PortConfig parameter
            const IN_PORT_CONFIG = 1 << 3;
            /// Output ports can be reconfigured with PortConfig parameter
            const OUT_PORT_CONFIG = 1 << 4;
            /// Node needs configuration before it can be started.
            const NEED_CONFIGURE = 1 << 5;
            /// the process function might not
            /// immediately produce or consume data
            /// but might offload the work to a worker
            /// thread.
            const ASYNC = 1 << 6;

        }
    }

    #[derive(Debug, Clone)]
    pub struct NodeInfo {
        pub max_input_ports: u32,
        pub max_output_ports: u32,
        pub change_mask: NodeInfoChangeMask,
        pub flags: NodeFlags,
        pub props: pod::dictionary::Dictionary,
        pub params: Vec<ParamInfo>,
    }

    impl NodeInfo {
        fn deserialize(pod: &mut PodStructDeserializer) -> pod_v2::deserialize::Result<Self> {
            Ok(Self {
                max_input_ports: pod.pop_field()?.as_u32()?,
                max_output_ports: pod.pop_field()?.as_u32()?,
                change_mask: NodeInfoChangeMask::from_bits_retain(pod.pop_field()?.as_u64()?),
                flags: NodeFlags::from_bits_retain(pod.pop_field()?.as_u64()?),
                props: pod::dictionary::Dictionary(parse_dict(pod)?),
                params: parse_params(pod)?,
            })
        }
    }

    impl pod::serialize::PodSerialize for NodeInfo {
        fn serialize<O: std::io::Write + std::io::Seek>(
            &self,
            mut serializer: pod::serialize::PodSerializer<O>,
            flatten: bool,
        ) -> Result<pod::serialize::SerializeSuccess<O>, pod::serialize::GenError> {
            let mut s = serializer.serialize_struct(flatten)?;

            s.serialize_field(&self.max_input_ports)?;
            s.serialize_field(&self.max_output_ports)?;
            s.serialize_field(&self.change_mask.bits())?;
            s.serialize_field(&self.flags.bits())?;

            s.serialize_flattened(&self.props)?;

            s.serialize_field(&(self.params.len() as i32))?;

            for ParamInfo { id, flags } in self.params.iter() {
                s.serialize_field(&id.as_raw())?;
                s.serialize_field(&flags.bits())?;
            }

            s.end()
        }
    }

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct UpdateChangeMask: u32 {
            const PARAMS = 1 << 0;
            const INFO = 1 << 1;
        }
    }

    /// Update the params and info of the node.
    #[derive(Debug, Clone)]
    pub struct Update {
        /// A bitfield of changed items
        pub change_mask: UpdateChangeMask,
        /// Number of update params, valid when change_mask has (1<<0)
        pub params: Vec<OwnedPod>,
        // An updated param
        pub info: NodeInfo,
    }

    impl HasOpCode for Update {
        const OPCODE: u8 = 2;
    }

    impl pod::serialize::PodSerialize for Update {
        fn serialize<O: std::io::Write + std::io::Seek>(
            &self,
            mut serializer: pod::serialize::PodSerializer<O>,
            flatten: bool,
        ) -> Result<pod::serialize::SerializeSuccess<O>, pod::serialize::GenError> {
            let mut s = serializer.serialize_struct(flatten)?;
            s.serialize_field(&self.change_mask.bits())?;
            if !self.params.is_empty() {
                // s.serialize_field(&self.n_params)?;
                todo!()
            } else {
                s.serialize_field(&0u32)?;
            }
            s.serialize_field(&self.info)?;
            s.end()
        }
    }

    impl Update {
        pub fn deserialize(pod: &mut PodDeserializer) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                change_mask: UpdateChangeMask::from_bits_retain(pod.pop_field()?.as_u32()?),
                params: {
                    let n_params = pod.pop_field()?.as_i32()?;

                    if let Ok(n_params) = usize::try_from(n_params) {
                        let mut params = Vec::with_capacity(n_params);
                        for _ in 0..n_params {
                            params.push(pod.pop_field()?.to_owned());
                        }
                        params
                    } else {
                        Vec::new()
                    }
                },
                info: NodeInfo::deserialize(&mut pod.pop_field()?.as_struct()?)?,
            })
        }
    }

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct PortInfoChangeMask: u64 {
            const FLAGS = 1 << 0;
            const RATE = 1 << 1;
            const PROPS = 1 << 2;
            const PARAMS = 1 << 3;
        }
    }

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct PortFlags: u64 {
            /// Port can be removed
            const SPA_PORT_FLAG_REMOVABLE = 1 << 0;
            /// processing on port is optional
            const SPA_PORT_FLAG_OPTIONAL = 1 << 1;
            /// the port can allocate buffer data
            const SPA_PORT_FLAG_CAN_ALLOC_BUFFERS =	1 << 2;
            /// The port can process data in-place and
            /// will need a writable input buffer
            const SPA_PORT_FLAG_IN_PLACE = 1 << 3;
            /// The port does not keep a ref on the buffer.
            /// This means the node will always completely
            /// consume the input buffer and it will be
            /// recycled after process.
            const SPA_PORT_FLAG_NO_REF = 1 << 4;
            /// Output buffers from this port are
            /// timestamped against a live clock.
            const SPA_PORT_FLAG_LIVE = 1 << 5;
            /// Connects to some device
            const SPA_PORT_FLAG_PHYSICAL = 1 << 6;
            /// Data was not created from this port
            /// or will not be made available on another
            /// port
            const SPA_PORT_FLAG_TERMINAL = 1 << 7;
            /// Data pointer on buffers can be changed.
            /// Only the buffer data marked as DYNAMIC
            /// can be changed.
            const SPA_PORT_FLAG_DYNAMIC_DATA = 1 << 8;
        }
    }

    // This is not a method, just part of PortuUpdate
    #[derive(Debug, Clone)]
    pub struct PortInfo {
        /// Bitmask of changed items
        pub change_mask: PortInfoChangeMask,
        /// Flags, see struct spa_port_info, when change_mask has (1<<0)
        pub flags: PortFlags,
        /// Updated rate numerator
        pub rate_num: u32,
        /// Updated rate denominator, when info.change_mask has (1<<1)
        pub rate_denom: u32,
        /// Updated properties, valid when info.change_mask has (1<<2)
        pub items: pod::dictionary::Dictionary,
        /// Updated struct spa_param_info, valid when info.change_mask has (1<<3)
        pub params: Vec<(pod::utils::Id, u32)>,
    }

    impl pod::serialize::PodSerialize for PortInfo {
        fn serialize<O: std::io::Write + std::io::Seek>(
            &self,
            mut serializer: pod::serialize::PodSerializer<O>,
            flatten: bool,
        ) -> Result<pod::serialize::SerializeSuccess<O>, pod::serialize::GenError> {
            let mut s = serializer.serialize_struct(flatten)?;

            s.serialize_field(&self.change_mask.bits())?;
            s.serialize_field(&self.flags.bits())?;
            s.serialize_field(&self.rate_num)?;
            s.serialize_field(&self.rate_denom)?;

            s.serialize_flattened(&self.items)?;

            s.serialize_field(&(self.params.len() as i32))?;

            for (id, flags) in self.params.iter() {
                s.serialize_field(id)?;
                s.serialize_field(flags)?;
            }

            s.end()
        }
    }

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct PortUpdateChangeMask: u32 {
            const PARAMS = 1 << 0;
            const INFO = 1 << 1;
        }
    }

    /// Create, Update or destroy a node port.
    ///
    /// When the port is not known on the server, the port is created.
    /// When info is None, the port is destroyed. Otherwise, the port information is updated.
    #[derive(Debug, Clone)]
    pub struct PortUpdate {
        /// The port direction
        pub direction: SpaDirection,
        /// The port id
        pub port_id: u32,
        /// A bitfield of changed items
        pub change_mask: PortUpdateChangeMask,
        /// Updated params
        pub params: Vec<pod::Value>,
        /// An updated [`PortInfo`], valid when change_mask has (1<<1)
        pub info: Option<PortInfo>,
    }

    impl pod::serialize::PodSerialize for PortUpdate {
        fn serialize<O: std::io::Write + std::io::Seek>(
            &self,
            serializer: pod::serialize::PodSerializer<O>,
            flatten: bool,
        ) -> Result<pod::serialize::SerializeSuccess<O>, pod::serialize::GenError> {
            let mut s = serializer.serialize_struct(flatten)?;
            s.serialize_field(&(self.direction as u32))?;
            s.serialize_field(&self.port_id)?;
            s.serialize_field(&self.change_mask.bits())?;

            s.serialize_field(&(self.params.len() as i32))?;

            for param in self.params.iter() {
                s.serialize_field(param)?;
            }

            if let Some(info) = self.info.as_ref() {
                s.serialize_field(info)?;
            } else {
                s.serialize_field(&pod::Value::None)?;
            }

            s.end()
        }
    }

    impl HasOpCode for PortUpdate {
        const OPCODE: u8 = 3;
    }

    /// Set the node active or inactive.
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct SetActive {
        /// The new state of the node
        pub active: bool,
    }

    impl HasOpCode for SetActive {
        const OPCODE: u8 = 4;
    }

    /// Emit an event on the node.
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct Event {
        /// the event to emit. See enum spa_node_event
        event: pod::Value,
    }

    impl HasOpCode for Event {
        const OPCODE: u8 = 5;
    }

    /// This method is used by the client when it has allocated buffers for a port.
    ///
    /// It is usually called right after the UseBuffers event to let the server know about the the newly allocated buffer memory.
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct PortBuffers {
        // TODO:
    }

    impl HasOpCode for PortBuffers {
        const OPCODE: u8 = 6;
    }
}

pub mod events {
    use std::os::fd::RawFd;

    use super::*;

    /// The server will allocate the activation record and eventfd for the node and transfer this to the client with the Transport event.
    ///
    /// The activation record is currently an internal data structure that is not yet ABI stable.
    ///
    /// The writefd is meant to wake up the server after the driver completes so that the profiler can collect the information.
    /// The profiler is active when the pw_node_activation::flags fields has PW_NODE_ACTIVATION_FLAG_PROFILER set.
    /// When the profiler is disabled (or when the node is not driving), this eventfd should not be signaled.
    #[derive(Debug, Clone)]
    pub struct Transport {
        /// The eventfd to start processing
        pub readfd: pod::utils::Fd,
        /// The eventfd to signal when the driver completes and profiling is enabled.
        pub writefd: pod::utils::Fd,
        /// The index of the memfd of the activation record
        pub memid: u32,
        /// The offset in memfd of the start of the activation record
        pub offset: u32,
        /// The size of the activation record
        pub size: u32,
    }

    impl EventDeserialize for Transport {
        const OPCODE: u8 = 0;

        fn deserialize(
            pod: &mut PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                readfd: {
                    let id = pod.pop_field()?.as_fd()?;
                    pod::utils::Fd {
                        id,
                        fd: fds.get(id as usize).copied(),
                    }
                },
                writefd: {
                    let id = pod.pop_field()?.as_fd()?;
                    pod::utils::Fd {
                        id,
                        fd: fds.get(id as usize).copied(),
                    }
                },
                memid: pod.pop_field()?.as_u32()?,
                offset: pod.pop_field()?.as_u32()?,
                size: pod.pop_field()?.as_u32()?,
            })
        }
    }

    /// Set a parameter on the Node
    #[derive(Debug, Clone)]
    pub struct SetParam {
        /// The param id to set.
        pub id: SpaEnum<SpaParamType>,
        /// Extra flags
        pub flags: u32,
        /// The param object to set
        pub param: OwnedPod,
    }

    impl EventDeserialize for SetParam {
        const OPCODE: u8 = 1;

        fn deserialize(
            pod: &mut PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: SpaEnum::from_raw(pod.pop_field()?.as_id()?),
                flags: pod.pop_field()?.as_u32()?,
                param: pod.pop_field()?.to_owned(),
            })
        }
    }

    /// Set an IO area on the node.
    #[derive(Debug, Clone)]
    pub struct SetIo {
        /// The io area id to set.
        pub id: u32,
        /// Memid to use, this is signaled with Core::AddMem
        pub memid: u32,
        /// The start offset in the memory area
        pub offset: u32,
        /// The size of the io area
        pub size: u32,
    }

    impl EventDeserialize for SetIo {
        const OPCODE: u8 = 2;

        fn deserialize(
            pod: &mut PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_id()?,
                memid: pod.pop_field()?.as_u32()?,
                offset: pod.pop_field()?.as_u32()?,
                size: pod.pop_field()?.as_u32()?,
            })
        }
    }

    /// Emit an event on the node.
    #[derive(Debug, Clone)]
    pub struct Event {
        /// The event to emit. See enum spa_node_event.
        pub event: OwnedPod,
    }

    impl EventDeserialize for Event {
        const OPCODE: u8 = 3;

        fn deserialize(
            pod: &mut PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                event: pod.pop_field()?.to_owned(),
            })
        }
    }

    /// Send a command on the node.
    #[derive(Debug, Clone)]
    pub struct Command {
        /// The command to send. See enum spa_node_command.
        pub command: OwnedPod,
    }

    impl EventDeserialize for Command {
        const OPCODE: u8 = 4;

        fn deserialize(
            pod: &mut PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                command: pod.pop_field()?.to_owned(),
            })
        }
    }

    /// Add a new port to the node
    #[derive(Debug, Clone)]
    pub struct AddPort {
        /// The direction of the new port
        pub direction: SpaEnum<SpaDirection>,
        /// The port id of the new port
        pub port_id: u32,
        /// Optional extra properties for the port
        pub props: HashMap<String, String>,
    }

    impl EventDeserialize for AddPort {
        const OPCODE: u8 = 5;

        fn deserialize(
            pod: &mut PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                direction: SpaEnum::from_raw(pod.pop_field()?.as_u32()?),
                port_id: pod.pop_field()?.as_u32()?,
                props: parse_dict(&mut pod.pop_field()?.as_struct()?)?,
            })
        }
    }

    /// Remove a port from the node
    #[derive(Debug, Clone)]
    pub struct RemovePort {
        /// The direction of the port to remove
        pub direction: SpaEnum<SpaDirection>,
        /// The port id of the port to remove
        pub port_id: u32,
    }

    impl EventDeserialize for RemovePort {
        const OPCODE: u8 = 6;

        fn deserialize(
            pod: &mut PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                direction: SpaEnum::from_raw(pod.pop_field()?.as_u32()?),
                port_id: pod.pop_field()?.as_u32()?,
            })
        }
    }

    /// Set a parameter on the Port of the node.
    #[derive(Debug, Clone)]
    pub struct PortSetParam {
        /// The direction of the port
        pub direction: SpaEnum<SpaDirection>,
        /// The port id of the port
        pub port_id: u32,
        /// The param id to set.
        pub id: SpaEnum<SpaParamType>,
        /// Extra flags
        pub flags: u32,
        /// The param object to set
        pub param: OwnedPod,
    }

    impl EventDeserialize for PortSetParam {
        const OPCODE: u8 = 7;

        fn deserialize(
            pod: &mut PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                direction: SpaEnum::from_raw(pod.pop_field()?.as_u32()?),
                port_id: pod.pop_field()?.as_u32()?,
                id: SpaEnum::from_raw(pod.pop_field()?.as_id()?),
                flags: pod.pop_field()?.as_u32()?,
                param: pod.pop_field()?.to_owned(),
            })
        }
    }

    // Not an event
    #[derive(Debug, Clone)]
    pub struct PortBufferData {
        /// The data type, this can be:
        /// - SPA_DATA_MemId to reference a memfd from Core:AddMem
        /// - SPA_DATA_MemPtr to reference this buffer memid
        pub type_: SpaEnum<SpaDataType>,
        /// Contains the memid or offset in the memid
        pub data: u32,
        /// Extra flags for the data
        pub flags: u32,
        /// The offset in memfd
        pub mapoffset: u32,
        /// The maxsize of the memory in memfd
        pub maxsize: u32,
    }

    impl PortBufferData {
        fn deserialize(
            pod: &mut pod_v2::deserialize::PodStructDeserializer,
        ) -> pod_v2::deserialize::Result<Self> {
            Ok(Self {
                type_: SpaEnum::from_raw(pod.pop_field()?.as_id()?),
                data: pod.pop_field()?.as_u32()?,
                flags: pod.pop_field()?.as_u32()?,
                mapoffset: pod.pop_field()?.as_u32()?,
                maxsize: pod.pop_field()?.as_u32()?,
            })
        }
    }

    // Not an event
    #[derive(Debug, Clone)]
    pub struct PortBuffer {
        /// The memory id of the buffer metadata and or data
        pub mem_id: u32,
        /// The offset in memid of the buffer
        pub offset: u32,
        /// The size of the buffer metadata or data
        pub size: u32,
        /// Number of metadata. The buffer memory first contains this number of metadata parts of the given type and size
        pub metas: Vec<(SpaEnum<SpaMetaType>, u32)>,
        /// Datablocks
        pub data_blocks: Vec<PortBufferData>,
    }

    impl PortBuffer {
        fn deserialize(
            pod: &mut pod_v2::deserialize::PodStructDeserializer,
        ) -> pod_v2::deserialize::Result<Self> {
            Ok(Self {
                mem_id: pod.pop_field()?.as_u32()?,
                offset: pod.pop_field()?.as_u32()?,
                size: pod.pop_field()?.as_u32()?,
                metas: {
                    let n_metas = pod.pop_field()?.as_u32()? as usize;
                    let mut metas = Vec::with_capacity(n_metas);
                    for _ in 0..n_metas {
                        let id = SpaEnum::from_raw(pod.pop_field()?.as_u32()?);
                        let size = pod.pop_field()?.as_u32()?;
                        metas.push((id, size));
                    }
                    metas
                },
                data_blocks: {
                    let n_blocks = pod.pop_field()?.as_u32()? as usize;
                    let mut blocks = Vec::with_capacity(n_blocks);
                    for _ in 0..n_blocks {
                        blocks.push(PortBufferData::deserialize(pod)?);
                    }
                    blocks
                },
            })
        }
    }

    /// Use a set of buffers on the mixer port
    #[derive(Debug, Clone)]
    pub struct PortUseBuffers {
        /// The direction of the port
        pub direction: SpaEnum<SpaDirection>,
        /// The port id of the port
        pub port_id: u32,
        /// The mixer id of the port
        pub mix_id: u32,
        /// Extra flags
        pub flags: u32,
        pub buffers: Vec<PortBuffer>,
    }

    impl EventDeserialize for PortUseBuffers {
        const OPCODE: u8 = 8;

        fn deserialize(
            pod: &mut PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                direction: SpaEnum::from_raw(pod.pop_field()?.as_u32()?),
                port_id: pod.pop_field()?.as_u32()?,
                mix_id: pod.pop_field()?.as_u32()?,
                flags: pod.pop_field()?.as_u32()?,
                buffers: {
                    let n_buffers = pod.pop_field()?.as_u32()? as usize;
                    let mut buffers = Vec::with_capacity(n_buffers);
                    for _ in 0..n_buffers {
                        buffers.push(PortBuffer::deserialize(&mut pod)?);
                    }
                    buffers
                },
            })
        }
    }

    /// Set an IO area on a mixer port.
    #[derive(Debug, Clone)]
    pub struct PortSetIo {
        /// The direction of the port
        pub direction: SpaEnum<SpaDirection>,
        /// The port id of the port
        pub port_id: u32,
        /// The mix id of the port
        pub mix_id: u32,
        /// The IO area to set. See enum spa_io_type
        pub id: SpaEnum<SpaIoType>,
        /// The memid of the io area, added with Core::AddMem
        pub memid: u32,
        /// The offset in the memid
        pub offset: u32,
        /// The size of the IO area
        pub size: u32,
    }

    impl EventDeserialize for PortSetIo {
        const OPCODE: u8 = 9;

        fn deserialize(
            pod: &mut PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                direction: SpaEnum::from_raw(pod.pop_field()?.as_u32()?),
                port_id: pod.pop_field()?.as_u32()?,
                mix_id: pod.pop_field()?.as_u32()?,
                id: SpaEnum::from_raw(pod.pop_field()?.as_id()?),
                memid: pod.pop_field()?.as_u32()?,
                offset: pod.pop_field()?.as_u32()?,
                size: pod.pop_field()?.as_u32()?,
            })
        }
    }

    /// Notify the client of the activation record of a peer node.
    /// This activation record should be triggered when this node finishes processing.
    #[derive(Debug, Clone)]
    pub struct SetActivation {
        /// The node_id of the peer node
        pub node_id: u32,
        /// The eventfd of the peer node
        pub signalfd: pod::utils::Fd,
        /// The memid of the activation record of the peer from Core:AddMem
        pub memid: u32,
        /// The offset in memid
        pub offset: u32,
        /// The size of the activation record
        pub size: u32,
    }

    impl EventDeserialize for SetActivation {
        const OPCODE: u8 = 10;

        fn deserialize(
            pod: &mut PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                node_id: pod.pop_field()?.as_u32()?,
                signalfd: {
                    let id = pod.pop_field()?.as_fd()?;
                    pod::utils::Fd {
                        id,
                        fd: usize::try_from(id).ok().and_then(|id| fds.get(id)).copied(),
                    }
                },
                memid: pod.pop_field()?.as_u32()?,
                offset: pod.pop_field()?.as_u32()?,
                size: pod.pop_field()?.as_u32()?,
            })
        }
    }

    /// Notify the node of the peer of a mixer port.
    /// This can be used to track the peer ports of a node.
    #[derive(Debug, Clone)]
    pub struct PortSetMixInfo {
        /// The direction of the port
        pub direction: SpaEnum<SpaDirection>,
        /// The port id of the port
        pub port_id: u32,
        /// The mix id of the port
        pub mix_id: u32,
        /// The id of the peer port
        pub peer_id: u32,
        /// Optional properties
        pub props: HashMap<String, String>,
    }

    impl EventDeserialize for PortSetMixInfo {
        const OPCODE: u8 = 11;

        fn deserialize(
            pod: &mut PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                direction: SpaEnum::from_raw(pod.pop_field()?.as_u32()?),
                port_id: pod.pop_field()?.as_u32()?,
                mix_id: pod.pop_field()?.as_u32()?,
                peer_id: pod.pop_field()?.as_u32()?,
                props: parse_dict(&mut pod.pop_field()?.as_struct()?)?,
            })
        }
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

impl HasInterface for Event {
    const INTERFACE: &'static str = "ClientNode";
}
