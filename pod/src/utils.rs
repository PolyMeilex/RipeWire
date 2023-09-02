//! Miscellaneous and utility items.

use std::os::fd::RawFd;

use bitflags::bitflags;

pub use spa_sys::SpaFraction;
pub use spa_sys::SpaRectangle;

use crate::pod::CanonicalFixedSizedPod;

/// An enumerated value in a pod
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);

/// A file descriptor in a pod
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Fd {
    pub id: i64,
    pub fd: Option<RawFd>,
}

impl Fd {
    pub(crate) fn new(id: i64) -> Self {
        Self { id, fd: None }
    }

    pub fn get(&self) -> RawFd {
        self.id as RawFd
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// the flags and choice of a choice pod.
pub struct Choice<T: CanonicalFixedSizedPod>(pub ChoiceFlags, pub ChoiceEnum<T>);

bitflags! {
    /// [`Choice`] flags
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct ChoiceFlags: u32 {
        // no flags defined yet but we need at least one to keep bitflags! happy
        #[doc(hidden)]
        const _FAKE = 1;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// a choice in a pod.
pub enum ChoiceEnum<T: CanonicalFixedSizedPod> {
    /// no choice.
    None(T),
    /// range.
    Range {
        /// default value.
        default: T,
        /// minimum value.
        min: T,
        /// maximum value.
        max: T,
    },
    /// range with step.
    Step {
        /// default value.
        default: T,
        /// minimum value.
        min: T,
        /// maximum value.
        max: T,
        /// step.
        step: T,
    },
    /// list.
    Enum {
        /// default value.
        default: T,
        /// alternative values.
        alternatives: Vec<T>,
    },
    /// flags.
    Flags {
        /// default value.
        default: T,
        /// possible flags.
        flags: Vec<T>,
    },
}
