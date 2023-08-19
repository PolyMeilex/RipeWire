//! The client object represents a client connect to the PipeWire server. Permissions of the client can be managed.
//!
//! The currently connected client always has the Client object with proxy id 1.

use super::HasOpCode;

pub const OBJECT_ID: u32 = 1;

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

pub mod events {
    use super::*;

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy, pod_derive::PodBitflagDeserialize)]
        pub struct ChangeMask: u64 {
            const PROPS = 1;
        }
    }

    /// Get client information updates. This is emitted when binding to a client or when the client info is updated later.
    #[derive(Debug, Clone, pod_derive::PodDeserialize)]
    pub struct Info {
        /// The global id of the client
        pub id: u32,
        /// The changes emitted by this event
        pub change_mask: ChangeMask,
        /// properties of this object, valid when change_mask has (1<<0)
        pub properties: pod::dictionary::Dictionary,
    }

    impl HasOpCode for Info {
        const OPCODE: u8 = 0;
    }

    /// Emitted as the reply of the GetPermissions method.
    #[derive(Debug, Clone, pod_derive :: PodDeserialize)]
    pub struct Permissions {
        /// Index of the first permission
        pub index: u32,
        /// Permission entries
        pub permissions: pod::permissions::Permissions,
    }

    impl HasOpCode for Permissions {
        const OPCODE: u8 = 1;
    }
}

#[derive(Debug, Clone, pod_derive::EventDeserialize)]
pub enum Event {
    /// Get client information updates. This is emitted when binding to a client or when the client info is updated later.
    Info(events::Info),
    /// Emitted as the reply of the GetPermissions method.
    Permissions(events::Permissions),
}
