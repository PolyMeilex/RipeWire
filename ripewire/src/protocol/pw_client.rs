//! The client object represents a client connect to the PipeWire server. Permissions of the client can be managed.
//!
//! The currently connected client always has the Client object with proxy id 1.

use super::*;

pub const OBJECT_ID: u32 = 1;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
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

    #[derive(Debug, Clone)]
    pub struct AddListener {}

    impl MethodSerialize for AddListener {
        const OPCODE: u8 = 0;
        fn serialize(&self, buf: impl Write + Seek) {
            unreachable!()
        }
    }

    /// Is used to send an error to a client.
    ///
    /// This requires W and X permissions on the client.
    #[derive(Debug, Clone)]
    pub struct Error {
        /// A client proxy id to send the error to
        pub id: u32,
        /// A negative errno style error code
        pub res: u32,
        /// An error message
        pub error: String,
    }

    impl MethodSerialize for Error {
        const OPCODE: u8 = 1;
        fn serialize(&self, mut buf: impl Write + Seek) {
            pod::Builder::new(&mut buf).push_struct_with(|b| {
                b.write_u32(self.id);
                b.write_u32(self.res);
                b.write_str(&self.error);
            });
        }
    }

    /// Is used to update the properties of a client.
    ///
    /// This requires W and X permissions on the client.
    #[derive(Debug, Clone)]
    pub struct UpdateProperties {
        /// Properties to update on the client
        pub properties: HashMap<String, String>,
    }

    impl MethodSerialize for UpdateProperties {
        const OPCODE: u8 = 2;
        fn serialize(&self, mut buf: impl Write + Seek) {
            pod::Builder::new(&mut buf).push_struct_with(|b| {
                b.push_struct_with(|b| {
                    b.write_u32(self.properties.len() as u32);
                    for (key, value) in self.properties.iter() {
                        b.write_str(key);
                        b.write_str(value);
                    }
                });
            });
        }
    }

    /// Get the currently configured permissions on the client.
    ///
    /// This requires W and X permissions on the client.
    #[derive(Debug, Clone)]
    pub struct GetPermissions {
        /// The start index of the permissions to get
        pub index: u32,
        /// The number of permissions to get
        pub num: u32,
    }

    impl MethodSerialize for GetPermissions {
        const OPCODE: u8 = 3;
        fn serialize(&self, mut buf: impl Write + Seek) {
            pod::Builder::new(&mut buf).push_struct_with(|b| {
                b.write_u32(self.index);
                b.write_u32(self.num);
            });
        }
    }

    /// Update the permissions of the global objects using the provided array with permissions
    ///
    /// This requires W and X permissions on the client.
    #[derive(Debug, Clone)]
    pub struct UpdatePermissions(pub Vec<Permission>);

    impl MethodSerialize for UpdatePermissions {
        const OPCODE: u8 = 4;
        fn serialize(&self, mut buf: impl Write + Seek) {
            pod::Builder::new(&mut buf).push_struct_with(|b| {
                b.write_u32(self.0.len() as u32);
                for Permission { id, permissions } in self.0.iter() {
                    b.write_u32(*id);
                    b.write_u32(permissions.bits());
                }
            });
        }
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
        deserializer: &mut pod::PodDeserializer,
    ) -> pod::deserialize::Result<Vec<Permission>> {
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
            pod: &mut pod::PodDeserializer,
            fds: &[RawFd],
        ) -> pod::deserialize::Result<Self> {
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
            pod: &mut pod::PodDeserializer,
            fds: &[RawFd],
        ) -> pod::deserialize::Result<Self> {
            let mut pod = pod.as_struct()?;
            Ok(Self {
                index: pod.pop_field()?.as_u32()?,
                permissions: parse_permissions(&mut pod.pop_field()?)?,
            })
        }
    }
}

#[derive(Debug, Clone, pod_derive::EventDeserialize)]
pub enum Event {
    /// Get client information updates. This is emitted when binding to a client or when the client info is updated later.
    Info(events::Info),
    /// Emitted as the reply of the GetPermissions method.
    Permissions(events::Permissions),
}

impl HasInterface for Event {
    const INTERFACE: &'static str = "Client";
}
