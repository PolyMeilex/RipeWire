// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

//! The `libspa` crate provides a high-level API to interact with
//! [libspa](https://gitlab.freedesktop.org/pipewire/pipewire/-/tree/master/doc/spa).

pub mod array;
pub mod params;
pub mod pod;
pub mod pod_struct;
pub mod props;
pub mod utils;

pub use pod::*;

pub use serialize::PodSerializer;
