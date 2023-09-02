//! This module deals with SPA pods, providing ways to represent pods using idiomatic types
//! and serialize them into their raw representation, and the other way around.
//!
//! Everything concerning serializing raw pods from rust types is in the [`serialize`] submodule.
//! and everything about deserializing rust types from raw pods is in the [`deserialize`] submodule.
//!
//! The entire serialization and deserialization approach is inspired by and similar to the excellent `serde` crate,
//! but is much more specialized to fit the SPA pod format.

pub mod deserialize;
pub mod serialize;

use std::{
    ffi::c_void,
    io::{Seek, Write},
};

use bitflags::bitflags;
use cookie_factory::{
    bytes::{ne_f32, ne_f64, ne_i32, ne_i64, ne_u32},
    gen_simple,
    sequence::pair,
    GenError,
};
use nom::{
    combinator::map,
    number::{
        complete::{f32, f64, i32, i64, u32},
        Endianness,
    },
    IResult,
};

use deserialize::{BoolVisitor, NoneVisitor, PodDeserialize, PodDeserializer};
use serialize::{PodSerialize, PodSerializer};

use crate::utils::{Choice, Fd, Id, SpaFraction, SpaRectangle};

use self::deserialize::{
    ChoiceDoubleVisitor, ChoiceFdVisitor, ChoiceFloatVisitor, ChoiceFractionVisitor,
    ChoiceIdVisitor, ChoiceIntVisitor, ChoiceLongVisitor, ChoiceRectangleVisitor, DoubleVisitor,
    FdVisitor, FloatVisitor, FractionVisitor, IdVisitor, IntVisitor, LongVisitor, PointerVisitor,
    RectangleVisitor,
};

/// Implementors of this trait are the canonical representation of a specific type of fixed sized SPA pod.
///
/// They can be used as an output type for [`FixedSizedPod`] implementors
/// and take care of the actual serialization/deserialization from/to the type of raw SPA pod they represent.
///
/// The trait is sealed, so it can't be implemented outside of this crate.
/// This is to ensure that no invalid pod can be serialized.
///
/// If you want to have your type convert from and to a fixed sized pod, implement [`FixedSizedPod`] instead and choose
/// a fitting implementor of this trait as the `CanonicalType` instead.
pub trait CanonicalFixedSizedPod: private::CanonicalFixedSizedPodSeal {
    /// The raw type this serializes into.
    #[doc(hidden)]
    const TYPE: spa_sys::SpaType;
    /// The size of the pods body.
    #[doc(hidden)]
    const SIZE: u32;
    #[doc(hidden)]
    fn serialize_body<O: Write>(&self, out: O) -> Result<O, GenError>;
    #[doc(hidden)]
    fn deserialize_body(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
}

mod private {
    /// This trait makes [`super::CanonicalFixedSizedPod`] a "sealed trait", which makes it impossible to implement
    /// ouside of this crate.
    pub trait CanonicalFixedSizedPodSeal {}
    impl CanonicalFixedSizedPodSeal for () {}
    impl CanonicalFixedSizedPodSeal for bool {}
    impl CanonicalFixedSizedPodSeal for i32 {}
    impl CanonicalFixedSizedPodSeal for i64 {}
    impl CanonicalFixedSizedPodSeal for f32 {}
    impl CanonicalFixedSizedPodSeal for f64 {}
    impl CanonicalFixedSizedPodSeal for super::SpaRectangle {}
    impl CanonicalFixedSizedPodSeal for super::SpaFraction {}
    impl CanonicalFixedSizedPodSeal for super::Id {}
    impl CanonicalFixedSizedPodSeal for super::Fd {}
}

impl<T: CanonicalFixedSizedPod + Copy> FixedSizedPod for T {
    type CanonicalType = Self;

    fn as_canonical_type(&self) -> Self::CanonicalType {
        *self
    }

    fn from_canonical_type(canonical: &Self::CanonicalType) -> Self {
        *canonical
    }
}

/// Serialize into a `None` type pod.
impl CanonicalFixedSizedPod for () {
    const TYPE: spa_sys::SpaType = spa_sys::SpaType::None;
    const SIZE: u32 = 0;

    fn serialize_body<O: Write>(&self, out: O) -> Result<O, GenError> {
        Ok(out)
    }

    fn deserialize_body(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        Ok((input, ()))
    }
}

/// Serialize into a `Bool` type pod.
impl CanonicalFixedSizedPod for bool {
    const TYPE: spa_sys::SpaType = spa_sys::SpaType::Bool;
    const SIZE: u32 = 4;

    fn serialize_body<O: Write>(&self, out: O) -> Result<O, GenError> {
        gen_simple(ne_u32(u32::from(*self)), out)
    }

    fn deserialize_body(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        map(u32(Endianness::Native), |b| b != 0)(input)
    }
}

/// Serialize into a `Int` type pod.
impl CanonicalFixedSizedPod for i32 {
    const TYPE: spa_sys::SpaType = spa_sys::SpaType::Int;
    const SIZE: u32 = 4;

    fn serialize_body<O: Write>(&self, out: O) -> Result<O, GenError> {
        gen_simple(ne_i32(*self), out)
    }

    fn deserialize_body(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        i32(Endianness::Native)(input)
    }
}

/// Serialize into a `Long` type pod.
impl CanonicalFixedSizedPod for i64 {
    const TYPE: spa_sys::SpaType = spa_sys::SpaType::Long;
    const SIZE: u32 = 8;

    fn serialize_body<O: Write>(&self, out: O) -> Result<O, GenError> {
        gen_simple(ne_i64(*self), out)
    }

    fn deserialize_body(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        i64(Endianness::Native)(input)
    }
}

/// Serialize into a `Float` type pod.
impl CanonicalFixedSizedPod for f32 {
    const TYPE: spa_sys::SpaType = spa_sys::SpaType::Float;
    const SIZE: u32 = 4;

    fn serialize_body<O: Write>(&self, out: O) -> Result<O, GenError> {
        gen_simple(ne_f32(*self), out)
    }

    fn deserialize_body(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        f32(Endianness::Native)(input)
    }
}

/// Serialize into a `Double` type pod.
impl CanonicalFixedSizedPod for f64 {
    const TYPE: spa_sys::SpaType = spa_sys::SpaType::Double;
    const SIZE: u32 = 8;

    fn serialize_body<O: Write>(&self, out: O) -> Result<O, GenError> {
        gen_simple(ne_f64(*self), out)
    }

    fn deserialize_body(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        f64(Endianness::Native)(input)
    }
}

/// Serialize into a `Rectangle` type pod.
impl CanonicalFixedSizedPod for SpaRectangle {
    const TYPE: spa_sys::SpaType = spa_sys::SpaType::Rectangle;
    const SIZE: u32 = 8;

    fn serialize_body<O: Write>(&self, out: O) -> Result<O, GenError> {
        gen_simple(pair(ne_u32(self.width), ne_u32(self.height)), out)
    }

    fn deserialize_body(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        map(
            nom::sequence::pair(u32(Endianness::Native), u32(Endianness::Native)),
            |(width, height)| Self { width, height },
        )(input)
    }
}

/// Serialize into a `Fraction` type pod.
impl CanonicalFixedSizedPod for SpaFraction {
    const TYPE: spa_sys::SpaType = spa_sys::SpaType::Fraction;
    const SIZE: u32 = 8;

    fn serialize_body<O: Write>(&self, out: O) -> Result<O, GenError> {
        gen_simple(pair(ne_u32(self.num), ne_u32(self.denom)), out)
    }

    fn deserialize_body(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        map(
            nom::sequence::pair(u32(Endianness::Native), u32(Endianness::Native)),
            |(num, denom)| Self { num, denom },
        )(input)
    }
}

impl CanonicalFixedSizedPod for Id {
    const TYPE: spa_sys::SpaType = spa_sys::SpaType::Id;
    const SIZE: u32 = 4;

    fn serialize_body<O: Write>(&self, out: O) -> Result<O, GenError> {
        gen_simple(ne_u32(self.0), out)
    }

    fn deserialize_body(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        map(u32(Endianness::Native), Id)(input)
    }
}

impl CanonicalFixedSizedPod for Fd {
    const TYPE: spa_sys::SpaType = spa_sys::SpaType::Fd;
    const SIZE: u32 = 8;

    fn serialize_body<O: Write>(&self, out: O) -> Result<O, GenError> {
        gen_simple(ne_i64(self.id), out)
    }

    fn deserialize_body(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        map(i64(Endianness::Native), Fd::new)(input)
    }
}

/// Implementors of this trait can be serialized into pods that always have the same size.
/// This lets them be used as elements in `Array` type SPA Pods.
///
/// Implementors of this automatically implement [`PodSerialize`].
///
/// Serialization is accomplished by having the type convert itself into/from the canonical representation of this pod,
/// e.g. `i32` for a `Int` type pod.
///
/// That type then takes care of the actual serialization.
///
/// See the [`CanonicalFixedSizedPod`] trait for a list of possible target types.
///
/// Which type to convert in is specified with the traits [`FixedSizedPod::CanonicalType`] type,
/// while the traits [`as_canonical_type`](`FixedSizedPod::as_canonical_type`)
/// and [`from_canonical_type`](`FixedSizedPod::from_canonical_type`) methods are responsible for the actual conversion.
///
/// # Examples
/// Implementing the trait on a `i32` newtype wrapper:
/// ```rust
/// use libspa::pod::FixedSizedPod;
///
/// struct Newtype(i32);
///
/// impl FixedSizedPod for Newtype {
///     // The pod we want to serialize into is a `Int` type pod, which has `i32` as it's canonical representation.
///     type CanonicalType = i32;
///
///     fn as_canonical_type(&self) -> Self::CanonicalType {
///         // Convert self to the canonical type.
///         self.0
///     }
///
///     fn from_canonical_type(canonical: &Self::CanonicalType) -> Self {
///         // Create a new Self instance from the canonical type.
///         Newtype(*canonical)
///     }
/// }
/// ```
pub trait FixedSizedPod {
    /// The canonical representation of the type of pod that should be serialized to/deserialized from.
    type CanonicalType: CanonicalFixedSizedPod;

    /// Convert `self` to the canonical type.
    fn as_canonical_type(&self) -> Self::CanonicalType;
    /// Convert the canonical type to `Self`.
    fn from_canonical_type(_: &Self::CanonicalType) -> Self;
}

impl<T: FixedSizedPod> PodSerialize for T {
    fn serialize<O: Write + Seek>(
        &self,
        serializer: PodSerializer<O>,
        _flatten: bool,
    ) -> Result<serialize::SerializeSuccess<O>, GenError> {
        serializer.serialized_fixed_sized_pod(self)
    }
}

impl<'de> PodDeserialize<'de> for () {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_none(NoneVisitor)
    }
}

impl<'de> PodDeserialize<'de> for bool {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_bool(BoolVisitor)
    }
}

impl<'de> PodDeserialize<'de> for i32 {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_int(IntVisitor)
    }
}

impl<'de> PodDeserialize<'de> for i64 {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_long(LongVisitor)
    }
}

impl<'de> PodDeserialize<'de> for f32 {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_float(FloatVisitor)
    }
}

impl<'de> PodDeserialize<'de> for f64 {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_double(DoubleVisitor)
    }
}

impl<'de> PodDeserialize<'de> for SpaRectangle {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_rectangle(RectangleVisitor)
    }
}

impl<'de> PodDeserialize<'de> for SpaFraction {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_fraction(FractionVisitor)
    }
}

impl<'de> PodDeserialize<'de> for Id {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_id(IdVisitor)
    }
}

impl<'de> PodDeserialize<'de> for Fd {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_fd(FdVisitor)
    }
}

impl<'de> PodDeserialize<'de> for Choice<i32> {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_choice(ChoiceIntVisitor)
    }
}

impl<'de> PodDeserialize<'de> for Choice<i64> {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_choice(ChoiceLongVisitor)
    }
}

impl<'de> PodDeserialize<'de> for Choice<f32> {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_choice(ChoiceFloatVisitor)
    }
}

impl<'de> PodDeserialize<'de> for Choice<f64> {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_choice(ChoiceDoubleVisitor)
    }
}

impl<'de> PodDeserialize<'de> for Choice<Id> {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_choice(ChoiceIdVisitor)
    }
}

impl<'de> PodDeserialize<'de> for Choice<SpaRectangle> {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_choice(ChoiceRectangleVisitor)
    }
}

impl<'de> PodDeserialize<'de> for Choice<SpaFraction> {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_choice(ChoiceFractionVisitor)
    }
}

impl<'de> PodDeserialize<'de> for Choice<Fd> {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_choice(ChoiceFdVisitor)
    }
}

impl<'de, T> PodDeserialize<'de> for (spa_sys::SpaType, *const T) {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_pointer(PointerVisitor::<T>::default())
    }
}

impl<'de> PodDeserialize<'de> for Value {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer.deserialize_any()
    }
}

/// A typed pod value.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// no value or a NULL pointer.
    None,
    /// a boolean value.
    Bool(bool),
    /// an enumerated value.
    Id(Id),
    /// a 32 bits integer.
    Int(i32),
    /// a 64 bits integer.
    Long(i64),
    /// a 32 bits floating.
    Float(f32),
    /// a 64 bits floating.
    Double(f64),
    /// a string.
    String(String),
    /// a byte array.
    Bytes(Vec<u8>),
    /// a rectangle with width and height.
    Rectangle(SpaRectangle),
    /// a fraction with numerator and denominator.
    Fraction(SpaFraction),
    /// a file descriptor.
    Fd(Fd),
    /// an array of same type objects.
    ValueArray(ValueArray),
    /// a collection of types and objects.
    Struct(Vec<Value>),
    /// an object.
    Object(Object),
    /// a choice.
    Choice(ChoiceValue),
    /// a pointer.
    Pointer(spa_sys::SpaType, *const c_void),
}

/// an array of same type objects.
#[derive(Debug, Clone, PartialEq)]
pub enum ValueArray {
    /// an array of none.
    None(Vec<()>),
    /// an array of booleans.
    Bool(Vec<bool>),
    /// an array of Id.
    Id(Vec<Id>),
    /// an array of 32 bits integer.
    Int(Vec<i32>),
    /// an array of 64 bits integer.
    Long(Vec<i64>),
    /// an array of 32 bits floating.
    Float(Vec<f32>),
    /// an array of 64 bits floating.
    Double(Vec<f64>),
    /// an array of Rectangle.
    Rectangle(Vec<SpaRectangle>),
    /// an array of Fraction.
    Fraction(Vec<SpaFraction>),
    /// an array of Fd.
    Fd(Vec<Fd>),
}

/// A typed choice.
#[derive(Debug, Clone, PartialEq)]
pub enum ChoiceValue {
    /// Choice on 32 bits integer values.
    Int(Choice<i32>),
    /// Choice on 64 bits integer values.
    Long(Choice<i64>),
    /// Choice on 32 bits floating values.
    Float(Choice<f32>),
    /// Choice on 64 bits floating values.
    Double(Choice<f64>),
    /// Choice on id values.
    Id(Choice<Id>),
    /// Choice on rectangle values.
    Rectangle(Choice<SpaRectangle>),
    /// Choice on fraction values.
    Fraction(Choice<SpaFraction>),
    /// Choice on fd values.
    Fd(Choice<Fd>),
}

/// An object from a pod.
#[derive(Clone, PartialEq)]
pub struct Object {
    /// the object type.
    pub type_: spa_sys::SpaType,
    /// the object id.
    pub id: u32,
    /// the object properties.
    pub properties: Vec<Property>,
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Object")
            .field("type_", &self.type_)
            .field("id", &self.id)
            .field("properties", &self.properties)
            .finish()
    }
}

/// An object property.
#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    /// key of the property, list of valid keys depends on the objec type.
    pub key: u32,
    /// flags for the property.
    pub flags: PropertyFlags,
    /// value of the property.
    pub value: Value,
}

bitflags! {
    /// Property flags
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct PropertyFlags: u32 {
        // These flags are redefinitions from
        // https://gitlab.freedesktop.org/pipewire/pipewire/-/blob/master/spa/include/spa/pod/pod.h
        /// Property is read-only.
        const READONLY = spa_sys::SpaPropFlags::READONLY.bits();
        /// Property is some sort of hardware parameter.
        const HARDWARE = spa_sys::SpaPropFlags::HARDWARE.bits();
        /// Property contains a dictionary struct.
        const HINT_DICT = spa_sys::SpaPropFlags::HINT_DICT.bits();
        /// Property is mandatory.
        const MANDATORY = spa_sys::SpaPropFlags::MANDATORY.bits();
        /// Property choices need no fixation.
        #[cfg(feature = "v0_3_33")]
        const DONT_FIXATE = spa_sys::SpaPropFlags::DONT_FIXATE.bits();
    }
}

//
// Technically not part of the protocol but widely used
//

use crate::serialize::SerializeSuccess;

impl<'de> PodDeserialize<'de> for u32 {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer
            .deserialize_int(IntVisitor)
            .map(|(v, res)| (v as u32, res))
    }
}

impl PodSerialize for u32 {
    fn serialize<O: Write + Seek>(
        &self,
        serializer: PodSerializer<O>,
        _flatten: bool,
    ) -> Result<SerializeSuccess<O>, GenError> {
        serializer.serialized_fixed_sized_pod(&(*self as i32))
    }
}

impl<'de> PodDeserialize<'de> for u64 {
    fn deserialize(
        deserializer: PodDeserializer<'de>,
    ) -> Result<
        (Self, deserialize::DeserializeSuccess<'de>),
        deserialize::DeserializeError<&'de [u8]>,
    >
    where
        Self: Sized,
    {
        deserializer
            .deserialize_long(LongVisitor)
            .map(|(v, res)| (v as u64, res))
    }
}

impl PodSerialize for u64 {
    fn serialize<O: Write + Seek>(
        &self,
        serializer: PodSerializer<O>,
        _flatten: bool,
    ) -> Result<SerializeSuccess<O>, GenError> {
        serializer.serialized_fixed_sized_pod(&(*self as i64))
    }
}

impl PodSerialize for String {
    fn serialize<O: Write + Seek>(
        &self,
        serializer: PodSerializer<O>,
        _flatten: bool,
    ) -> Result<SerializeSuccess<O>, GenError> {
        serializer.serialize_string(self)
    }
}
