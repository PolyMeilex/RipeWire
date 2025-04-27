pub mod dbg_print;
pub mod serialize;
pub use serialize::Builder;

pub mod deserialize;
pub use deserialize::{DeserializeError, PodDeserializer, PodDeserializerKind};

pub use libspa_consts;
#[allow(unused)]
pub mod obj_gen;

fn pad_to_8(size: u32) -> u32 {
    if size % 8 == 0 {
        0
    } else {
        8 - (size % 8)
    }
}

/// An enumerated value in a pod
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);

impl From<&Id> for Id {
    fn from(value: &Id) -> Self {
        *value
    }
}

impl From<u32> for Id {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
