use crate::{pod, serialize::PodSerialize, CanonicalFixedSizedPod, FixedSizedPod};

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
