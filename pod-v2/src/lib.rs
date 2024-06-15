pub mod dbg_print;
pub mod serialize;
pub use serialize::Builder;

pub mod deserialize;
pub use deserialize::{DeserializeError, PodDeserializer, PodDeserializerKind};

pub use libspa_consts;

fn pad_to_8(size: u32) -> u32 {
    if size % 8 == 0 {
        0
    } else {
        8 - (size % 8)
    }
}
