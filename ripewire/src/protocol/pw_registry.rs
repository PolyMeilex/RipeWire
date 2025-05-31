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

    /// Bind to a global object
    ///
    /// Bind to the global object with \a id and use the client proxy with new_id as the proxy.
    /// After this call, methods can be sent to the remote global object and events can be received.
    ///
    /// - id: the global id to bind to
    /// - type: the interface type to bind to
    /// - version: the interface version to use
    #[derive(Debug, Clone)]
    pub struct Bind {
        pub id: u32,
        // TODO: ObjectType here?
        pub interface: String,
        pub version: u32,
        pub new_id: u32,
    }

    impl MethodSerializeSimple for Bind {
        const OPCODE: u8 = 1;
        fn serialize_simple(&self, buf: impl Write + Seek) {
            pod::Builder::new(buf).push_struct_with(|b| {
                b.write_u32(self.id);
                b.write_str(&self.interface);
                b.write_u32(self.version);
                b.write_u32(self.new_id);
            });
        }
    }

    /// Attempt to destroy a global object
    ///
    /// Try to destroy the global object.
    ///
    /// - id: the global id to destroy.
    ///
    /// The client needs X permissions on the global.
    #[derive(Debug, Clone)]
    pub struct Destroy {
        pub id: u32,
    }

    impl MethodSerializeSimple for Destroy {
        const OPCODE: u8 = 2;
        fn serialize_simple(&self, buf: impl Write + Seek) {
            pod::Builder::new(buf).push_struct_with(|b| {
                b.write_u32(self.id);
            });
        }
    }
}

pub mod events {
    use super::*;
    use crate::protocol::pw_client::PermissionFlags;

    const INTERFACE: &str = "Registry";

    /// Notify of a new global object
    ///
    /// The registry emits this event when a new global object is available.
    ///
    /// - id: the global object id
    /// - permissions: the permissions of the object
    /// - type: the type of the interface
    /// - version: the version of the interface
    /// - props: extra properties of the global
    #[derive(Debug, Clone)]
    pub struct Global {
        pub id: u32,
        pub permissions: PermissionFlags,
        pub interface: ObjectType,
        pub version: u32,
        pub properties: HashMap<String, String>,
    }

    impl EventDeserialize for Global {
        const OPCODE: u8 = 0;

        fn deserialize(
            pod: &mut pod::PodDeserializer,
            fds: &[RawFd],
        ) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_u32()?,
                permissions: PermissionFlags::from_bits_retain(pod.pop_field()?.as_u32()?),
                interface: ObjectType::from_interface_name(&pod.pop_field()?.as_str()?.to_string()),
                version: pod.pop_field()?.as_u32()?,
                properties: parse_dict(&mut pod.pop_field()?.as_struct()?)?,
            })
        }
    }

    /// Notify of a global object removal
    ///
    /// Emitted when a global object was removed from the registry.
    /// If the client has any bindings to the global, it should destroy those.
    ///
    /// - id: the id of the global that was removed
    #[derive(Debug, Clone)]
    pub struct GlobalRemove {
        pub id: u32,
    }

    impl EventDeserialize for GlobalRemove {
        const OPCODE: u8 = 1;

        fn deserialize(
            pod: &mut pod::PodDeserializer,
            _fds: &[RawFd],
        ) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_u32()?,
            })
        }
    }
}

#[derive(Debug, Clone, pod_derive::EventDeserialize)]
pub enum Event {
    /// Notify of a new global object
    ///
    /// The registry emits this event when a new global object is available.
    Global(events::Global),

    /// Notify of a global object removal
    ///
    /// Emitted when a global object was removed from the registry.
    /// If the client has any bindings to the global, it should destroy those.
    GlobalRemove(events::GlobalRemove),
}

impl HasInterface for Event {
    const INTERFACE: &'static str = "Registry";
}
