use crate::{
    deserialize::PodDeserialize, pod, serialize::PodSerialize, CanonicalFixedSizedPod,
    FixedSizedPod,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array<E>(pub Vec<E>)
where
    E: CanonicalFixedSizedPod + FixedSizedPod + std::marker::Copy;

impl<E> PodSerialize for Array<E>
where
    E: CanonicalFixedSizedPod + FixedSizedPod + std::marker::Copy,
{
    fn serialize<O: std::io::Write + std::io::Seek>(
        &self,
        serializer: pod::serialize::PodSerializer<O>,
        _flatten: bool,
    ) -> Result<pod::serialize::SerializeSuccess<O>, pod::serialize::GenError> {
        let mut s = serializer.serialize_array(self.0.len() as u32)?;
        for e in self.0.iter() {
            s.serialize_element(e)?;
        }
        s.end()
    }
}

impl<'de, E: FixedSizedPod> PodDeserialize<'de> for Array<E>
where
    E: CanonicalFixedSizedPod + FixedSizedPod + std::marker::Copy,
{
    fn deserialize(
        deserializer: pod::deserialize::PodDeserializer<'de>,
    ) -> Result<
        (Self, pod::deserialize::DeserializeSuccess<'de>),
        pod::deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        let (items, succes) = deserializer.deserialize_array_vec::<E>()?;
        Ok((Self(items), succes))
    }
}
