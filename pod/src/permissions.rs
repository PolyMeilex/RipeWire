use crate::{
    deserialize::{DeserializeError, PodDeserialize, StructPodDeserializer, Visitor},
    pod,
    serialize::PodSerialize,
};

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
    pub id: Option<u32>,
    /// The permissions for the global id
    pub permissions: PermissionFlags,
}

#[derive(Debug, Clone, Default)]
pub struct Permissions(pub Vec<Permission>);

impl PodSerialize for Permissions {
    fn serialize<O: std::io::Write + std::io::Seek>(
        &self,
        serializer: pod::serialize::PodSerializer<O>,
        flatten: bool,
    ) -> Result<pod::serialize::SerializeSuccess<O>, pod::serialize::GenError> {
        let mut s = serializer.serialize_struct(flatten)?;

        s.serialize_field(&(self.0.len() as i32))?;

        for Permission { id, permissions } in self.0.iter() {
            s.serialize_field(&id.map(|id| id as i32).unwrap_or(-1))?;
            s.serialize_field(&permissions.bits())?;
        }

        s.end()
    }
}

impl<'de> PodDeserialize<'de> for Permissions {
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

        impl<'de> Visitor<'de> for TestVisitor {
            type Value = Permissions;
            type ArrayElem = std::convert::Infallible;

            fn visit_struct(
                &self,
                struct_deserializer: &mut StructPodDeserializer<'de>,
            ) -> Result<Self::Value, DeserializeError<&'de [u8]>> {
                let mut list = Vec::new();

                let len: u32 = struct_deserializer.deserialize_field()?.unwrap();

                for _ in 0..len {
                    let id: i32 = struct_deserializer.deserialize_field()?.unwrap();
                    let permissions: u32 = struct_deserializer.deserialize_field()?.unwrap();

                    list.push(Permission {
                        id: u32::try_from(id).ok(),
                        permissions: PermissionFlags::from_bits_retain(permissions),
                    });
                }

                Ok(Permissions(list))
            }
        }

        deserializer.deserialize_struct(TestVisitor)
    }
}
