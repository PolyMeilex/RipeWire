use crate::{deserialize::PodDeserialize, pod, serialize::PodSerialize, Value};

#[derive(Debug, Clone, Default)]
pub struct Struct(pub Vec<Value>);

impl PodSerialize for Struct {
    fn serialize<O: std::io::Write + std::io::Seek>(
        &self,
        serializer: pod::serialize::PodSerializer<O>,
        flatten: bool,
    ) -> Result<pod::serialize::SerializeSuccess<O>, pod::serialize::GenError> {
        let mut s = serializer.serialize_struct(flatten)?;

        for value in self.0.iter() {
            s.serialize_field(value)?;
        }

        s.end()
    }
}

impl<'de> PodDeserialize<'de> for Struct {
    fn deserialize(
        deserializer: pod::deserialize::PodDeserializer<'de>,
    ) -> Result<
        (Self, pod::deserialize::DeserializeSuccess<'de>),
        pod::deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        let (value, succes) =
            deserializer.deserialize_struct(crate::pod::deserialize::ValueVisitor)?;

        let Value::Struct(fields) = value else {
            unreachable!();
        };

        Ok((Self(fields), succes))
    }
}
