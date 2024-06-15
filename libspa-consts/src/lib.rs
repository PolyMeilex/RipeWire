use bitflags::bitflags;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
mod bindings {
    include!("./gen/bindings.rs");
}

pub use bindings::*;

/// Wrapper type for enums that handles unknown variants gracefully
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpaEnum<T, RAW = u32> {
    Value(T),
    Unknown(RAW),
}

impl<T, RAW: std::fmt::Debug> SpaEnum<T, RAW> {
    pub fn ok(self) -> Option<T> {
        match self {
            SpaEnum::Value(v) => Some(v),
            SpaEnum::Unknown(_) => None,
        }
    }

    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            SpaEnum::Value(v) => v,
            SpaEnum::Unknown(v) => {
                panic!("called `SpaEnum::unwrap()` on a `Unknown({:?})` value", v)
            }
        }
    }
}

impl<T: num_traits::FromPrimitive> SpaEnum<T, u32> {
    pub fn from_raw(raw: u32) -> Self {
        T::from_u32(raw)
            .map(Self::Value)
            .unwrap_or(Self::Unknown(raw))
    }
}

impl<T: num_traits::FromPrimitive> SpaEnum<T, i32> {
    pub fn from_i32(raw: i32) -> Self {
        T::from_i32(raw)
            .map(Self::Value)
            .unwrap_or(Self::Unknown(raw))
    }
}

impl<T> SpaEnum<T, u32>
where
    T: num_traits::cast::ToPrimitive + Clone,
{
    pub fn as_raw(&self) -> u32 {
        match self {
            SpaEnum::Value(v) => v.to_u32().unwrap(),
            SpaEnum::Unknown(v) => *v,
        }
    }
}

impl<T> SpaEnum<T, i32>
where
    T: num_traits::cast::ToPrimitive + Clone,
{
    pub fn as_raw(&self) -> i32 {
        match self {
            SpaEnum::Value(v) => v.to_i32().unwrap(),
            SpaEnum::Unknown(v) => *v,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, num_derive::FromPrimitive, num_derive::ToPrimitive)]
#[repr(u32)]
pub enum SpaType {
    /* Basic types 0x00000 */
    None = 1,
    Bool,
    Id,
    Int,
    Long,
    Float,
    Double,
    String,
    Bytes,
    Rectangle,
    Fraction,
    Bitmap,
    Array,
    Struct,
    Object,
    Sequence,
    Pointer,
    Fd,
    Choice,
    Pod,

    /* Pointers 0x10000 */
    PointerBuffer = 0x10000 + 1,
    PointerMeta,
    PointerDict,

    /* Events 0x20000 */
    EventDevice = 0x20000 + 1,
    EventNode,

    /* Commands 0x30000 */
    CommandDevice = 0x30000 + 1,
    CommandNode,

    /* Objects 0x40000 */
    ObjectPropInfo = 0x40000 + 1,
    ObjectProps,
    ObjectFormat,
    ObjectParamBuffers,
    ObjectParamMeta,
    ObjectParamIo,
    ObjectParamProfile,
    ObjectParamPortConfig,
    ObjectParamRoute,
    ObjectProfiler,
    ObjectParamLatency,
    ObjectParamProcessLatency,
    ObjectParamParamTag,

    /* vendor extensions */
    VendorPipeWire = 0x02000000,

    VendorOther = 0x7f000000,
}

impl SpaType {
    pub fn from_raw(v: u32) -> Option<Self> {
        num_traits::FromPrimitive::from_u32(v)
    }
}

impl SpaFormat {
    pub fn from_raw(v: u32) -> Option<Self> {
        num_traits::FromPrimitive::from_u32(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, num_derive::FromPrimitive)]
#[repr(u32)]
pub enum SpaPointerType {
    Buffer = SpaType::PointerBuffer as u32,
    Meta = SpaType::PointerMeta as u32,
    Dict = SpaType::PointerDict as u32,
}

impl SpaPointerType {
    pub fn from_raw(v: u32) -> Option<Self> {
        num_traits::FromPrimitive::from_u32(v)
    }
}

impl SpaDataType {
    pub fn from_raw(v: u32) -> Option<Self> {
        num_traits::FromPrimitive::from_u32(v)
    }
}

impl SpaChoiceType {
    pub fn from_raw(v: u32) -> Option<Self> {
        num_traits::FromPrimitive::from_u32(v)
    }
}

impl SpaParamType {
    pub fn from_raw(v: u32) -> Option<Self> {
        num_traits::FromPrimitive::from_u32(v)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PwMemblockFlags: u32 {
        /**< memory is readable */
        const READABLE = 1 << 0;
        /**< memory is writable */
        const WRITABLE = 1 << 1;
        /**< seal the fd */
        const SEAL = 1 << 2;
        /**< mmap the fd */
        const MAP = 1 << 3;
        /**< don't close fd */
        const DONT_CLOSE = 1 << 4;
        /**< don't notify events */
        const DONT_NOTIFY = 1 << 5;

        const READWRITE = Self::READABLE.bits() | Self::WRITABLE.bits();
    }
}

bitflags! {
    /// Property flags
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct SpaPropFlags: u32 {
        // These flags are redefinitions from
        // https://gitlab.freedesktop.org/pipewire/pipewire/-/blob/master/spa/include/spa/pod/pod.h
        /// Property is read-only.
        const READONLY = 1;
        /// Property is some sort of hardware parameter.
        const HARDWARE = 2;
        /// Property contains a dictionary struct.
        const HINT_DICT = 4;
        /// Property is mandatory.
        const MANDATORY = 8;
        /// Property choices need no fixation.
        const DONT_FIXATE = 16;
    }
}
