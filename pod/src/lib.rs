// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

//! The `libspa` crate provides a high-level API to interact with
//! [libspa](https://gitlab.freedesktop.org/pipewire/pipewire/-/tree/master/doc/spa).

pub mod array;
pub mod dictionary;
pub mod params;
pub mod permissions;
pub mod pod;
pub mod pod_struct;
pub mod utils;

pub use pod::*;

// TODO: enum deserializer derive
impl<'de> deserialize::PodDeserialize<'de> for spa_sys::SpaDirection {
    fn deserialize(
        deserializer: deserialize::PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        let (value, res) = deserializer.deserialize_id(deserialize::ValueVisitor)?;

        let Value::Id(id) = value else {
            return Err(deserialize::DeserializeError::UnsupportedType);
        };

        let value = if id.0 == Self::Output as u32 {
            Self::Output
        } else if id.0 == Self::Input as u32 {
            Self::Input
        } else {
            return Err(deserialize::DeserializeError::InvalidType);
        };

        Ok((value, res))
    }
}
