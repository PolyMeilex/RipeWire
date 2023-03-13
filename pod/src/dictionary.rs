use std::collections::HashMap;

use crate::{
    deserialize::{DeserializeError, PodDeserialize, StructPodDeserializer, Visitor},
    pod,
    serialize::PodSerialize,
};

#[derive(Debug, Clone, Default)]
pub struct Dictionary(pub HashMap<String, String>);

impl<I, Item> From<I> for Dictionary
where
    Item: Into<String>,
    I: Iterator<Item = (Item, Item)>,
{
    fn from(value: I) -> Self {
        let mut map = HashMap::new();
        for (key, value) in value {
            map.insert(key.into(), value.into());
        }
        Self(map)
    }
}

impl PodSerialize for Dictionary {
    fn serialize<O: std::io::Write + std::io::Seek>(
        &self,
        serializer: pod::serialize::PodSerializer<O>,
    ) -> Result<pod::serialize::SerializeSuccess<O>, pod::serialize::GenError> {
        let mut s = serializer.serialize_struct()?;

        s.serialize_field(&(self.0.len() as i32))?;

        for (key, value) in self.0.iter() {
            s.serialize_field(key)?;
            s.serialize_field(value)?;
        }

        s.end()
    }
}

impl<'de> PodDeserialize<'de> for Dictionary {
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
            type Value = Dictionary;
            type ArrayElem = std::convert::Infallible;

            fn visit_struct(
                &self,
                struct_deserializer: &mut StructPodDeserializer<'de>,
            ) -> Result<Self::Value, DeserializeError<&'de [u8]>> {
                let mut map = HashMap::new();

                let len: u32 = struct_deserializer.deserialize_field()?.unwrap();

                for _ in 0..len {
                    let key: String = struct_deserializer.deserialize_field()?.unwrap();
                    let val: String = struct_deserializer.deserialize_field()?.unwrap();

                    map.insert(key, val);
                }

                Ok(Dictionary(map))
            }
        }

        deserializer.deserialize_struct(TestVisitor)
    }
}
