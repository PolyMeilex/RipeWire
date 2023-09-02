use super::*;
use libspa_consts::SpaDirection;

pub mod methods {
    use super::*;
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

    #[derive(Debug, Clone)]
    pub struct NodeInfo {
        pub max_input_ports: u32,
        pub max_output_ports: u32,
        // TODO:
        // SPA_NODE_CHANGE_MASK_FLAGS		(1u<<0)
        // SPA_NODE_CHANGE_MASK_PROPS		(1u<<1)
        // SPA_NODE_CHANGE_MASK_PARAMS		(1u<<2)
        pub change_mask: u64,
        // TODO:
        // SPA_NODE_FLAG_RT			(1u<<0)	/**< node can do real-time processing */
        // SPA_NODE_FLAG_IN_DYNAMIC_PORTS		(1u<<1)	/**< input ports can be added/removed */
        // SPA_NODE_FLAG_OUT_DYNAMIC_PORTS		(1u<<2)	/**< output ports can be added/removed */
        // SPA_NODE_FLAG_IN_PORT_CONFIG		(1u<<3)	/**< input ports can be reconfigured with
        //                       *  PortConfig parameter */
        // SPA_NODE_FLAG_OUT_PORT_CONFIG		(1u<<4)	/**< output ports can be reconfigured with
        //                       *  PortConfig parameter */
        // SPA_NODE_FLAG_NEED_CONFIGURE		(1u<<5)	/**< node needs configuration before it can
        //                       *  be started. */
        // SPA_NODE_FLAG_ASYNC			(1u<<6)	/**< the process function might not
        //                       *  immediately produce or consume data
        //                       *  but might offload the work to a worker
        //                       *  thread. */
        pub flags: u64,
        pub props: pod::dictionary::Dictionary,
        pub params: Vec<(pod::utils::Id, u32)>,
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
            s.serialize_field(&self.change_mask)?;
            s.serialize_field(&self.flags)?;

            s.serialize_flattened(&self.props)?;

            s.serialize_field(&(self.params.len() as i32))?;

            for (id, flags) in self.params.iter() {
                s.serialize_field(id)?;
                s.serialize_field(flags)?;
            }

            s.end()
        }
    }

    /// Update the params and info of the node.
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct Update {
        /// A bitfield of changed items
        pub change_mask: u32,
        /// Number of update params, valid when change_mask has (1<<0)
        pub n_params: u32,
        // An updated param
        pub info: NodeInfo,
    }

    impl HasOpCode for Update {
        const OPCODE: u8 = 2;
    }

    // This is not a method, just part of PortuUpdate
    #[derive(Debug, Clone)]
    pub struct PortInfo {
        /// Bitmask of changed items
        pub change_mask: u64,
        /// Flags, see struct spa_port_info, when change_mask has (1<<0)
        pub flags: u64,
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

            s.serialize_field(&self.change_mask)?;
            s.serialize_field(&self.flags)?;
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
        pub change_mask: u32,
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
            s.serialize_field(&self.change_mask)?;

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
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct Transport {
        /// The eventfd to start processing
        #[fd]
        pub readfd: pod::utils::Fd,
        /// The eventfd to signal when the driver completes and profiling is enabled.
        #[fd]
        pub writefd: pod::utils::Fd,
        /// The index of the memfd of the activation record
        pub memid: u32,
        /// The offset in memfd of the start of the activation record
        pub offset: u32,
        /// The size of the activation record
        pub size: u32,
    }

    impl HasOpCode for Transport {
        const OPCODE: u8 = 0;
    }

    /// Set a parameter on the Node
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct SetParam {
        /// The param id to set.
        pub id: pod::utils::Id,
        /// Extra flags
        pub flags: u32,
        /// The param object to set
        pub param: pod::Value,
    }

    impl HasOpCode for SetParam {
        const OPCODE: u8 = 1;
    }

    /// Set an IO area on the node.
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct SetIo {
        /// The io area id to set.
        pub id: pod::utils::Id,
        /// Memid to use, this is signaled with Core::AddMem
        pub memid: u32,
        /// The start offset in the memory area
        pub offset: u32,
        /// The size of the io area
        pub size: u32,
    }

    impl HasOpCode for SetIo {
        const OPCODE: u8 = 2;
    }

    /// Emit an event on the node.
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct Event {
        /// The event to emit. See enum spa_node_event.
        pub event: pod::Value,
    }

    impl HasOpCode for Event {
        const OPCODE: u8 = 3;
    }

    /// Send a command on the node.
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct Command {
        /// The command to send. See enum spa_node_command.
        pub command: pod::Value,
    }

    impl HasOpCode for Command {
        const OPCODE: u8 = 4;
    }

    /// Add a new port to the node
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct AddPort {
        /// The direction of the new port
        pub direction: SpaDirection,
        /// The port id of the new port
        pub port_id: u32,
        /// Optional extra properties for the port
        pub props: pod::dictionary::Dictionary,
    }

    impl HasOpCode for AddPort {
        const OPCODE: u8 = 5;
    }

    /// Remove a port from the node
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct RemovePort {
        /// The direction of the port to remove
        pub direction: SpaDirection,
        /// The port id of the port to remove
        pub port_id: u32,
    }

    impl HasOpCode for RemovePort {
        const OPCODE: u8 = 6;
    }

    /// Set a parameter on the Port of the node.
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct PortSetParam {
        /// The direction of the port
        pub direction: SpaDirection,
        /// The port id of the port
        pub port_id: u32,
        /// The param id to set.
        pub id: pod::utils::Id,
        /// Extra flags
        pub flags: u32,
        /// The param object to set
        pub param: pod::Value,
    }

    impl HasOpCode for PortSetParam {
        const OPCODE: u8 = 7;
    }

    // Not an event
    #[derive(Debug, Clone)]
    pub struct PortBufferData {
        /// The data type, this can be:
        /// - SPA_DATA_MemId to reference a memfd from Core:AddMem
        /// - SPA_DATA_MemPtr to reference this buffer memid
        pub type_: pod::utils::Id,
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
        fn visit<'de>(
            struct_deserializer: &mut pod::deserialize::StructPodDeserializer<'de>,
        ) -> Result<Self, pod::deserialize::DeserializeError<&'de [u8]>> {
            Ok(Self {
                type_: struct_deserializer
                    .deserialize_field()?
                    .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?,
                data: struct_deserializer
                    .deserialize_field()?
                    .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?,
                flags: struct_deserializer
                    .deserialize_field()?
                    .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?,
                mapoffset: struct_deserializer
                    .deserialize_field()?
                    .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?,
                maxsize: struct_deserializer
                    .deserialize_field()?
                    .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?,
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
        pub metas: Vec<(pod::utils::Id, u32)>,
        /// Datablocks
        pub datas: Vec<PortBufferData>,
    }

    impl PortBuffer {
        fn visit<'de>(
            struct_deserializer: &mut pod::deserialize::StructPodDeserializer<'de>,
        ) -> Result<Self, pod::deserialize::DeserializeError<&'de [u8]>> {
            let mem_id = struct_deserializer
                .deserialize_field()?
                .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?;
            let offset = struct_deserializer
                .deserialize_field()?
                .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?;
            let size = struct_deserializer
                .deserialize_field()?
                .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?;
            let n_metas: i32 = struct_deserializer
                .deserialize_field()?
                .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?;

            let mut metas = Vec::new();

            for _ in 0..n_metas.max(0) {
                let type_ = struct_deserializer
                    .deserialize_field()?
                    .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?;
                let size = struct_deserializer
                    .deserialize_field()?
                    .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?;
                metas.push((type_, size));
            }

            let n_datas: i32 = struct_deserializer
                .deserialize_field()?
                .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?;

            let mut datas = Vec::new();
            for _ in 0..n_datas.max(0) {
                datas.push(PortBufferData::visit(struct_deserializer)?);
            }

            Ok(PortBuffer {
                mem_id,
                offset,
                size,
                metas,
                datas,
            })
        }
    }

    /// Use a set of buffers on the mixer port
    #[derive(Debug, Clone)]
    pub struct PortUseBuffers {
        /// The direction of the port
        pub direction: SpaDirection,
        /// The port id of the port
        pub port_id: u32,
        /// The mixer id of the port
        pub mix_id: u32,
        /// Extra flags
        pub flags: u32,
        pub buffers: Vec<PortBuffer>,
    }

    impl PortUseBuffers {
        pub(super) fn load_fds(&mut self, _fds: &[RawFd]) {}
    }

    impl<'de> pod::deserialize::PodDeserialize<'de> for PortUseBuffers {
        fn deserialize(
            deserializer: pod::deserialize::PodDeserializer<'de>,
        ) -> Result<
            (Self, pod::deserialize::DeserializeSuccess<'de>),
            pod::deserialize::DeserializeError<&'de [u8]>,
        >
        where
            Self: Sized,
        {
            struct TestVisitor;
            impl<'de> pod::deserialize::Visitor<'de> for TestVisitor {
                type Value = PortUseBuffers;
                type ArrayElem = std::convert::Infallible;

                fn visit_struct(
                    &self,
                    struct_deserializer: &mut pod::deserialize::StructPodDeserializer<'de>,
                ) -> Result<Self::Value, pod::deserialize::DeserializeError<&'de [u8]>>
                {
                    let direction = struct_deserializer
                        .deserialize_field()?
                        .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?;
                    let port_id = struct_deserializer
                        .deserialize_field()?
                        .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?;
                    let mix_id = struct_deserializer
                        .deserialize_field()?
                        .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?;
                    let flags = struct_deserializer
                        .deserialize_field()?
                        .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?;
                    let n_buffers: i32 = struct_deserializer
                        .deserialize_field()?
                        .ok_or(pod::deserialize::DeserializeError::PropertyMissing)?;

                    let mut buffers = Vec::new();
                    for _ in 0..n_buffers.max(0) {
                        buffers.push(PortBuffer::visit(struct_deserializer)?);
                    }

                    Ok(Self::Value {
                        direction,
                        port_id,
                        mix_id,
                        flags,
                        buffers,
                    })
                }
            }

            deserializer.deserialize_struct(TestVisitor)
        }
    }

    impl HasOpCode for PortUseBuffers {
        const OPCODE: u8 = 8;
    }

    /// Set an IO area on a mixer port.
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct PortSetIo {
        /// The direction of the port
        pub direction: SpaDirection,
        /// The port id of the port
        pub port_id: u32,
        /// The mix id of the port
        pub mix_id: u32,
        /// The IO area to set. See enum spa_io_type
        pub id: pod::utils::Id,
        /// The memid of the io area, added with Core::AddMem
        pub memid: u32,
        /// The offset in the memid
        pub offset: u32,
        /// The size of the IO area
        pub size: u32,
    }

    impl HasOpCode for PortSetIo {
        const OPCODE: u8 = 9;
    }

    /// Notify the client of the activation record of a peer node.
    /// This activation record should be triggered when this node finishes processing.
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct SetActivation {
        /// The node_id of the peer node
        pub node_id: u32,
        /// The eventfd of the peer node
        #[fd]
        pub signalfd: pod::utils::Fd,
        /// The memid of the activation record of the peer from Core:AddMem
        pub memid: u32,
        /// The offset in memid
        pub offset: u32,
        /// The size of the activation record
        pub size: u32,
    }

    impl HasOpCode for SetActivation {
        const OPCODE: u8 = 10;
    }

    /// Notify the node of the peer of a mixer port.
    /// This can be used to track the peer ports of a node.
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct PortSetMixInfo {
        /// The direction of the port
        pub direction: SpaDirection,
        /// The port id of the port
        pub port_id: u32,
        /// The mix id of the port
        pub mix_id: u32,
        /// The id of the peer port
        pub peer_id: u32,
        /// Optional properties
        pub props: pod::dictionary::Dictionary,
    }

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
