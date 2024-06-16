use std::collections::HashMap;

use crate::{pod, serialize::PodSerialize};

#[derive(Clone, Default)]
pub struct Dictionary(pub HashMap<String, String>);

impl std::fmt::Debug for Dictionary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<I, Item> From<I> for Dictionary
where
    Item: Into<String>,
    I: IntoIterator<Item = (Item, Item)>,
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
        flatten: bool,
    ) -> Result<pod::serialize::SerializeSuccess<O>, pod::serialize::GenError> {
        let mut s = serializer.serialize_struct(flatten)?;

        s.serialize_field(&(self.0.len() as i32))?;

        for (key, value) in self.0.iter() {
            s.serialize_field(key)?;
            s.serialize_field(value)?;
        }

        s.end()
    }
}
