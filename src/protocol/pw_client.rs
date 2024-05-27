//! The client object represents a client connect to the PipeWire server. Permissions of the client can be managed.
//!
//! The currently connected client always has the Client object with proxy id 1.

use super::*;

pub const OBJECT_ID: u32 = 1;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, pod_derive::PodBitflagDeserialize)]
    pub struct PermissionFlags: u32 {
        /// object can be seen and events can be received
        const R = 0o400;
        /// methods can be called that modify the object
        const W = 0o200;
        /// methods can be called on the object. The W flag must be
        /// present in order to call methods that modify the object.
        const X = 0o100;
        /// metadata can be set on object, Since 0.3.9
        const M = 0o010;
        /// a link can be made between a node that doesn't have
        /// permission to see the other node, Since 0.3.77
        const L = 0o020;
    }
}

#[derive(Debug, Clone)]
pub struct Permission {
    /// The global id
    pub id: u32,
    /// The permissions for the global id
    pub permissions: PermissionFlags,
}

pub mod methods {
    use super::*;

    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct AddListener {}

    impl HasOpCode for AddListener {
        const OPCODE: u8 = 0;
    }

    /// Is used to send an error to a client.
    ///
    /// This requires W and X permissions on the client.
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct Error {
        /// A client proxy id to send the error to
        pub id: u32,
        /// A negative errno style error code
        pub res: u32,
        /// An error message
        pub error: String,
    }

    impl HasOpCode for Error {
        const OPCODE: u8 = 1;
    }

    /// Is used to update the properties of a client.
    ///
    /// This requires W and X permissions on the client.
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct UpdateProperties {
        /// Properties to update on the client
        pub properties: pod::dictionary::Dictionary,
    }

    impl HasOpCode for UpdateProperties {
        const OPCODE: u8 = 2;
    }

    /// Get the currently configured permissions on the client.
    ///
    /// This requires W and X permissions on the client.
    #[derive(Debug, Clone, pod_derive::PodSerialize)]
    pub struct GetPermissions {
        /// The start index of the permissions to get
        pub index: u32,
        /// The number of permissions to get
        pub num: u32,
    }

    impl HasOpCode for GetPermissions {
        const OPCODE: u8 = 3;
    }

    /// Update the permissions of the global objects using the provided array with permissions
    ///
    /// This requires W and X permissions on the client.
    #[derive(Debug, Clone)]
    pub struct UpdatePermissions(pub pod::permissions::Permissions);

    impl pod::serialize::PodSerialize for UpdatePermissions {
        fn serialize<O: std::io::Write + std::io::Seek>(
            &self,
            serializer: pod::serialize::PodSerializer<O>,
            flatten: bool,
        ) -> Result<pod::serialize::SerializeSuccess<O>, pod::serialize::GenError> {
            self.0.serialize(serializer, flatten)
        }
    }

    impl HasOpCode for UpdatePermissions {
        const OPCODE: u8 = 4;
    }
}

pub use events::ChangeMask;
pub mod events {
    use super::*;

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy)]
        pub struct ChangeMask: u64 {
            const PROPS = 1;
        }
    }

    fn parse_permissions(
        deserializer: &mut pod_v2::PodDeserializer,
    ) -> pod_v2::deserialize::Result<Vec<Permission>> {
        let mut pod = deserializer.as_struct()?;

        let len = pod.pop_field()?.as_u32()?;

        let mut list = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let id = pod.pop_field()?.as_i32()?;
            let permissions = pod.pop_field()?.as_u32()?;
            if let Ok(id) = u32::try_from(id) {
                list.push(Permission {
                    id,
                    permissions: PermissionFlags::from_bits_retain(permissions),
                });
            }
        }

        Ok(list)
    }

    /// Get client information updates. This is emitted when binding to a client or when the client info is updated later.
    #[derive(Debug, Clone)]
    pub struct Info {
        /// The global id of the client
        pub id: u32,
        /// The changes emitted by this event
        pub change_mask: ChangeMask,
        /// properties of this object, valid when change_mask has PROPS
        pub properties: HashMap<String, String>,
    }

    impl EventDeserialize for Info {
        const OPCODE: u8 = 0;

        fn deserialize(
            pod: &mut pod_v2::PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                id: pod.pop_field()?.as_u32()?,
                change_mask: ChangeMask::from_bits_retain(pod.pop_field()?.as_u64()?),
                properties: parse_dict(&mut pod.pop_field()?.as_struct()?)?,
            })
        }
    }

    /// Emitted as the reply of the GetPermissions method.
    #[derive(Debug, Clone)]
    pub struct Permissions {
        /// Index of the first permission
        pub index: u32,
        /// Permission entries
        pub permissions: Vec<Permission>,
    }

    impl EventDeserialize for Permissions {
        const OPCODE: u8 = 1;

        fn deserialize(
            pod: &mut pod_v2::PodDeserializer,
            fds: &[RawFd],
        ) -> pod_v2::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                index: pod.pop_field()?.as_u32()?,
                permissions: parse_permissions(&mut pod.pop_field()?)?,
            })
        }
    }
}

#[derive(Debug, Clone, pod_derive::EventDeserialize2)]
pub enum Event {
    /// Get client information updates. This is emitted when binding to a client or when the client info is updated later.
    Info(events::Info),
    /// Emitted as the reply of the GetPermissions method.
    Permissions(events::Permissions),
}

impl HasInterface for Event {
    const INTERFACE: &'static str = "Client";
}
