//! Miscellaneous and utility items.

use bitflags::bitflags;

pub use spa_sys::Fraction;
pub use spa_sys::Rectangle;

use crate::pod::CanonicalFixedSizedPod;

/// An enumerated value in a pod
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);

/// A file descriptor in a pod
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Fd(pub i64);

#[derive(Debug, Eq, PartialEq, Clone)]
/// the flags and choice of a choice pod.
pub struct Choice<T: CanonicalFixedSizedPod>(pub ChoiceFlags, pub ChoiceEnum<T>);

bitflags! {
    /// [`Choice`] flags
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
