use std::{fmt, io::Write, mem, os::raw::c_void};

use super::pad_to_8;
use bstr::BStr;
use libspa_consts::{
    SpaChoiceType, SpaControlType, SpaEnum, SpaFormat, SpaFraction, SpaParamBuffers, SpaParamIo,
    SpaParamLatency, SpaParamMeta, SpaParamPortConfig, SpaParamProcessLatency, SpaParamProfile,
    SpaParamRoute, SpaParamTag, SpaProfiler, SpaProp, SpaPropInfo, SpaRectangle, SpaType,
};

#[derive(Debug, thiserror::Error)]
pub enum DeserializeError {
    #[error("Expected type '{expected:?}' got '{got:?}'")]
    UnexpectedType {
        expected: SpaType,
        got: SpaEnum<SpaType>,
    },
    #[error("Unexpected POD end")]
    UnexpectedEnd,
}

pub type Result<T> = std::result::Result<T, DeserializeError>;

trait Primitive {
    fn read_raw(bytes: &[u8]) -> Self;
}

macro_rules! impl_typed_pods {
    ( $($ty: ty),* $(,)? ) => {
        $(
            impl Primitive for $ty {
                fn read_raw(bytes: &[u8]) -> Self {
                    Self::from_ne_bytes(bytes[..mem::size_of::<Self>()].try_into().unwrap())
                }
            }
        )*
    };
}
impl_typed_pods!(i32, u32, i64, u64, f32, f64);

impl Primitive for *const c_void {
    fn read_raw(bytes: &[u8]) -> Self {
        let bytes = &bytes[..mem::size_of::<Self>()];
        bytes as *const [u8] as *const c_void
    }
}

fn read_raw<T: Primitive + Copy>(bytes: &[u8]) -> T {
    T::read_raw(bytes)
}

fn eat_raw<T: Primitive + Copy>(bytes: &[u8]) -> (T, &[u8]) {
    let v = read_raw(bytes);
    (v, &bytes[mem::size_of::<T>()..])
}

#[derive(Clone)]
pub struct OwnedPod {
    size: u32,
    ty: SpaEnum<SpaType>,
    body: Vec<u8>,
}

impl std::fmt::Debug for OwnedPod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_deserializer().fmt(f)
    }
}

impl OwnedPod {
    pub fn to_raw(&self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(self.body.len());
        buff.write_all(&self.size.to_le_bytes()).unwrap();
        buff.write_all(&self.ty.as_raw().to_le_bytes()).unwrap();
        buff.write_all(&self.body).unwrap();
        buff
    }

    pub fn as_deserializer(&self) -> PodDeserializer<'_> {
        PodDeserializer {
            size: self.size,
            ty: self.ty,
            body: self.body.as_ref(),
        }
    }
}

#[derive(Clone)]
pub struct PodDeserializer<'a> {
    size: u32,
    ty: SpaEnum<SpaType>,
    body: &'a [u8],
}

impl<'a> PodDeserializer<'a> {
    pub fn new(buff: &'a [u8]) -> (Self, &'a [u8]) {
        let (size, buff) = eat_raw(buff);
        let (ty, buff) = eat_raw(buff);
        let ty = SpaEnum::from_raw(ty);

        let pod = Self::form_body(size, ty, buff);
        let padded_size = pod.size_with_padding() as usize;

        (pod, &buff[padded_size..])
    }

    fn form_body(size: u32, ty: SpaEnum<SpaType>, body: &'a [u8]) -> Self {
        Self {
            size,
            ty,
            body: &body[..size as usize],
        }
    }

    pub fn ty(&self) -> SpaEnum<SpaType> {
        self.ty
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn to_owned(&self) -> OwnedPod {
        OwnedPod {
            size: self.size(),
            ty: self.ty(),
            body: self.body().to_owned(),
        }
    }

    fn padding(&self) -> u32 {
        pad_to_8(self.size())
    }

    fn size_with_padding(&self) -> u32 {
        self.size() + self.padding()
    }

    pub fn body(&self) -> &'a [u8] {
        self.body
    }

    pub fn kind(&self) -> PodDeserializerKind<'a> {
        type Kind<'a> = PodDeserializerKind<'a>;

        let SpaEnum::Value(ty) = self.ty else {
            return Kind::Unknown(self.clone());
        };

        match ty {
            SpaType::None => Kind::None,
            SpaType::Bool => Kind::Bool(read_raw::<i32>(self.body) != 0),
            SpaType::Id => Kind::Id(read_raw(self.body)),
            SpaType::Int => Kind::Int(read_raw(self.body)),
            SpaType::Long => Kind::Long(read_raw(self.body)),
            SpaType::Float => Kind::Float(read_raw(self.body)),
            SpaType::Double => Kind::Double(read_raw(self.body)),
            SpaType::String => Kind::String(self.as_str().unwrap()),
            SpaType::Bytes => Kind::Bytes(self.body),
            SpaType::Rectangle => Kind::Rectangle(SpaRectangle {
                width: read_raw(self.body),
                height: read_raw(&self.body[4..]),
            }),
            SpaType::Fraction => Kind::Fraction(SpaFraction {
                num: read_raw(self.body),
                denom: read_raw(&self.body[4..]),
            }),
            SpaType::Bitmap => Kind::Bitmap(self.body),
            SpaType::Array => Kind::Array(PodArrayDeserializer::new(self.body)),
            SpaType::Struct => Kind::Struct(PodStructDeserializer::new(self.body)),
            SpaType::Object => Kind::Object(PodObjectDeserializer::new(self.body)),
            SpaType::Sequence => Kind::Sequence(PodSequenceDeserializer::new(self.body)),
            SpaType::Pointer => {
                let ty = SpaEnum::from_raw(read_raw(self.body));
                let _padding: u32 = read_raw(&self.body[4..]);
                let ptr = read_raw(&self.body[8..]);
                Kind::Pointer { ty, ptr }
            }
            SpaType::Fd => Kind::Fd(read_raw(self.body)),
            SpaType::Choice => Kind::Choice(PodChoiceDeserializer::new(self.body)),
            _ => Kind::Unknown(self.clone()),
        }
    }

    fn unexpected_type(&self, expected: SpaType) -> DeserializeError {
        DeserializeError::UnexpectedType {
            expected,
            got: self.ty(),
        }
    }

    pub fn as_id(&self) -> Result<u32> {
        if let PodDeserializerKind::Id(v) = self.kind() {
            Ok(v)
        } else {
            Err(self.unexpected_type(SpaType::Id))
        }
    }

    pub fn as_i32(&self) -> Result<i32> {
        if let PodDeserializerKind::Int(v) = self.kind() {
            Ok(v)
        } else {
            Err(self.unexpected_type(SpaType::Int))
        }
    }

    pub fn as_u32(&self) -> Result<u32> {
        Ok(self.as_i32()? as u32)
    }

    pub fn as_i64(&self) -> Result<i64> {
        if let PodDeserializerKind::Long(v) = self.kind() {
            Ok(v)
        } else {
            Err(self.unexpected_type(SpaType::Long))
        }
    }

    pub fn as_u64(&self) -> Result<u64> {
        Ok(self.as_i64()? as u64)
    }

    pub fn as_f32(&self) -> Result<f32> {
        if let PodDeserializerKind::Float(v) = self.kind() {
            Ok(v)
        } else {
            Err(self.unexpected_type(SpaType::Float))
        }
    }

    pub fn as_f64(&self) -> Result<f64> {
        if let PodDeserializerKind::Double(v) = self.kind() {
            Ok(v)
        } else {
            Err(self.unexpected_type(SpaType::Double))
        }
    }

    pub fn as_rectangle(&self) -> Result<SpaRectangle> {
        if let PodDeserializerKind::Rectangle(v) = self.kind() {
            Ok(v)
        } else {
            Err(self.unexpected_type(SpaType::Rectangle))
        }
    }

    pub fn as_fraction(&self) -> Result<SpaFraction> {
        if let PodDeserializerKind::Fraction(v) = self.kind() {
            Ok(v)
        } else {
            Err(self.unexpected_type(SpaType::Fraction))
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self.kind(), PodDeserializerKind::None)
    }

    pub fn as_array(&self) -> Result<PodArrayDeserializer<'a>> {
        if let PodDeserializerKind::Array(pod) = self.kind() {
            Ok(pod)
        } else {
            Err(self.unexpected_type(SpaType::Array))
        }
    }

    pub fn as_struct(&self) -> Result<PodStructDeserializer<'a>> {
        if let PodDeserializerKind::Struct(pod) = self.kind() {
            Ok(pod)
        } else {
            Err(self.unexpected_type(SpaType::Struct))
        }
    }

    pub fn as_sequence(&self) -> Result<PodSequenceDeserializer<'a>> {
        if let PodDeserializerKind::Sequence(pod) = self.kind() {
            Ok(pod)
        } else {
            Err(self.unexpected_type(SpaType::Sequence))
        }
    }

    pub fn as_object(&self) -> Result<PodObjectDeserializer<'a>> {
        if let PodDeserializerKind::Object(pod) = self.kind() {
            Ok(pod)
        } else {
            Err(self.unexpected_type(SpaType::Struct))
        }
    }

    pub fn as_bytes(&self) -> Result<&'a [u8]> {
        if let PodDeserializerKind::Bytes(pod) = self.kind() {
            Ok(pod)
        } else {
            Err(self.unexpected_type(SpaType::Bytes))
        }
    }

    pub fn as_str(&self) -> Result<&'a BStr> {
        if self.ty == SpaEnum::Value(SpaType::String) {
            let bytes = &self.body[..self.size as usize];

            let bytes = match bytes.iter().position(|b| *b == 0) {
                Some(end) => &bytes[..end],
                None => bytes,
            };

            Ok(BStr::new(bytes))
        } else {
            Err(self.unexpected_type(SpaType::String))
        }
    }

    pub fn as_str_or_none(&self) -> Result<Option<&'a BStr>> {
        if self.ty == SpaEnum::Value(SpaType::String) {
            let bytes = &self.body[..self.size as usize];

            let bytes = match bytes.iter().position(|b| *b == 0) {
                Some(end) => &bytes[..end],
                None => bytes,
            };

            Ok(Some(BStr::new(bytes)))
        } else if self.ty == SpaEnum::Value(SpaType::None) {
            Ok(None)
        } else {
            Err(self.unexpected_type(SpaType::String))
        }
    }

    pub fn as_fd(&self) -> Result<i64> {
        if let PodDeserializerKind::Fd(v) = self.kind() {
            Ok(v)
        } else {
            Err(self.unexpected_type(SpaType::Fd))
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        if let PodDeserializerKind::Bool(v) = self.kind() {
            Ok(v)
        } else {
            Err(self.unexpected_type(SpaType::Bool))
        }
    }
}

#[derive(Debug)]
pub enum PodDeserializerKind<'a> {
    None,
    Bool(bool),
    Id(u32),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(&'a BStr),
    Bytes(&'a [u8]),
    Rectangle(SpaRectangle),
    Fraction(SpaFraction),
    Bitmap(&'a [u8]),
    Array(PodArrayDeserializer<'a>),
    Struct(PodStructDeserializer<'a>),
    Object(PodObjectDeserializer<'a>),
    Sequence(PodSequenceDeserializer<'a>),
    Pointer {
        ty: SpaEnum<SpaType>,
        ptr: *const c_void,
    },
    Fd(i64),
    Choice(PodChoiceDeserializer<'a>),
    Unknown(PodDeserializer<'a>),
}

#[derive(Clone)]
pub struct PodArrayDeserializer<'a> {
    child_size: u32,
    child_ty: SpaEnum<SpaType>,
    body: &'a [u8],
}

impl<'a> PodArrayDeserializer<'a> {
    fn new(body: &'a [u8]) -> Self {
        let (child_size, body) = eat_raw::<u32>(body);
        let (child_ty, body) = eat_raw::<u32>(body);
        let child_ty = SpaEnum::from_raw(child_ty);

        Self {
            child_size,
            child_ty,
            body,
        }
    }

    pub fn child_ty(&self) -> SpaEnum<SpaType> {
        self.child_ty
    }

    pub fn pop_element(&mut self) -> Result<PodDeserializer<'a>> {
        if self.body.is_empty() {
            return Err(DeserializeError::UnexpectedEnd);
        }

        let pod = PodDeserializer::form_body(self.child_size, self.child_ty, self.body);
        self.body = &self.body[pod.size() as usize..];

        Ok(pod)
    }
}

impl<'a> Iterator for PodArrayDeserializer<'a> {
    type Item = PodDeserializer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_element().ok()
    }
}

#[derive(Clone)]
pub struct PodStructDeserializer<'a> {
    body: &'a [u8],
}

impl<'a> PodStructDeserializer<'a> {
    fn new(body: &'a [u8]) -> Self {
        Self { body }
    }

    pub fn pop_field(&mut self) -> Result<PodDeserializer<'a>> {
        if self.body.is_empty() {
            return Err(DeserializeError::UnexpectedEnd);
        }

        let (pod, remaining) = PodDeserializer::new(self.body);
        self.body = remaining;
        Ok(pod)
    }
}

impl<'a> Iterator for PodStructDeserializer<'a> {
    type Item = PodDeserializer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_field().ok()
    }
}

#[derive(Clone)]
pub struct PodObjectDeserializer<'a> {
    object_ty: SpaEnum<SpaType>,
    object_id: u32,
    body: &'a [u8],
}

impl<'a> PodObjectDeserializer<'a> {
    fn new(body: &'a [u8]) -> Self {
        let (object_ty, body) = eat_raw::<u32>(body);
        let (object_id, body) = eat_raw::<u32>(body);

        let object_ty = SpaEnum::from_raw(object_ty);

        Self {
            object_id,
            object_ty,
            body,
        }
    }

    pub fn object_ty(&self) -> SpaEnum<SpaType> {
        self.object_ty
    }

    pub fn object_id(&self) -> u32 {
        self.object_id
    }

    pub fn pop_property(&mut self) -> Result<PobObjectPropertyDeserializer<'a>> {
        let remaining = self.body;

        if remaining.is_empty() {
            return Err(DeserializeError::UnexpectedEnd);
        }

        let (key, remaining) = eat_raw::<u32>(remaining);
        let (flags, remaining) = eat_raw::<u32>(remaining);
        let (value, remaining) = PodDeserializer::new(remaining);

        self.body = remaining;

        Ok(PobObjectPropertyDeserializer { key, flags, value })
    }
}

impl<'a> Iterator for PodObjectDeserializer<'a> {
    type Item = PobObjectPropertyDeserializer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_property().ok()
    }
}

pub struct PobObjectPropertyDeserializer<'a> {
    pub key: u32,
    pub flags: u32,
    pub value: PodDeserializer<'a>,
}

#[derive(Clone)]
pub struct PodChoiceDeserializer<'a> {
    choice_ty: SpaEnum<SpaChoiceType>,
    flags: u32,

    child_size: u32,
    child_ty: SpaEnum<SpaType>,
    body: &'a [u8],
}

impl<'a> PodChoiceDeserializer<'a> {
    fn new(body: &'a [u8]) -> Self {
        let (choice_ty, body) = eat_raw::<u32>(body);
        let (flags, body) = eat_raw::<u32>(body);
        let (child_size, body) = eat_raw::<u32>(body);
        let (child_ty, body) = eat_raw::<u32>(body);

        let choice_ty = SpaEnum::from_raw(choice_ty);
        let child_ty = SpaEnum::from_raw(child_ty);

        Self {
            choice_ty,
            flags,
            child_size,
            child_ty,
            body,
        }
    }

    pub fn choice_ty(&self) -> SpaEnum<SpaChoiceType> {
        self.choice_ty
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn child_ty(&self) -> SpaEnum<SpaType> {
        self.child_ty
    }

    pub fn pop_element(&mut self) -> Result<PodDeserializer<'a>> {
        if self.body.is_empty() {
            return Err(DeserializeError::UnexpectedEnd);
        }

        let pod = PodDeserializer::form_body(self.child_size, self.child_ty, self.body);
        self.body = &self.body[pod.size() as usize..];

        Ok(pod)
    }
}

impl<'a> Iterator for PodChoiceDeserializer<'a> {
    type Item = PodDeserializer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_element().ok()
    }
}

#[derive(Clone, Debug)]
pub struct PodSequenceDeserializer<'a> {
    unit: u32,
    body: &'a [u8],
}

impl<'a> PodSequenceDeserializer<'a> {
    fn new(body: &'a [u8]) -> Self {
        let (unit, body) = eat_raw::<u32>(body);
        let (pad, body) = eat_raw::<u32>(body);
        debug_assert_eq!(pad, 0);

        Self { unit, body }
    }

    pub fn unit(&self) -> u32 {
        self.unit
    }

    pub fn pop_control(&mut self) -> Result<PodControlDeserializer<'a>> {
        let remaining = self.body;

        if remaining.is_empty() {
            return Err(DeserializeError::UnexpectedEnd);
        }

        let (offset, remaining) = eat_raw::<u32>(remaining);
        let (type_, remaining) = eat_raw::<u32>(remaining);
        let (value, remaining) = PodDeserializer::new(remaining);

        self.body = remaining;

        Ok(PodControlDeserializer {
            offset,
            type_: SpaEnum::from_raw(type_),
            value,
        })
    }
}

impl<'a> Iterator for PodSequenceDeserializer<'a> {
    type Item = PodControlDeserializer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_control().ok()
    }
}

#[derive(Clone, Debug)]
pub struct PodControlDeserializer<'a> {
    offset: u32,
    type_: SpaEnum<SpaControlType>,
    value: PodDeserializer<'a>,
}

impl<'a> PodControlDeserializer<'a> {
    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn type_(&self) -> SpaEnum<SpaControlType> {
        self.type_
    }

    pub fn value(&self) -> &PodDeserializer<'a> {
        &self.value
    }
}

fn list_tuple(
    f: &mut fmt::Formatter<'_>,
    name: &str,
    v: impl Iterator<Item = impl fmt::Debug>,
) -> fmt::Result {
    let mut tuple = f.debug_tuple(name);
    for entry in v {
        tuple.field(&entry);
    }
    tuple.finish()
}

impl<'a> fmt::Debug for PodArrayDeserializer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        list_tuple(f, "Array", self.clone())
    }
}

impl<'a> fmt::Debug for PodStructDeserializer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        list_tuple(f, "Struct", self.clone())
    }
}

impl<'a> fmt::Debug for PobObjectPropertyDeserializer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Property")
            .field("key", &self.key)
            .field("flags", &self.flags)
            .field("value", &self.value)
            .finish()
    }
}

impl<'a> fmt::Debug for PodObjectDeserializer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DbgKey {
            key: u32,
            object_ty: SpaEnum<SpaType>,
        }

        impl fmt::Debug for DbgKey {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.object_ty {
                    SpaEnum::Value(SpaType::ObjectPropInfo) => {
                        SpaEnum::<SpaPropInfo>::from_raw(self.key).fmt(f)
                    }
                    SpaEnum::Value(SpaType::ObjectProps) => {
                        SpaEnum::<SpaProp>::from_raw(self.key).fmt(f)
                    }
                    SpaEnum::Value(SpaType::ObjectFormat) => {
                        SpaEnum::<SpaFormat>::from_raw(self.key).fmt(f)
                    }
                    SpaEnum::Value(SpaType::ObjectParamBuffers) => {
                        SpaEnum::<SpaParamBuffers>::from_raw(self.key).fmt(f)
                    }
                    SpaEnum::Value(SpaType::ObjectParamMeta) => {
                        SpaEnum::<SpaParamMeta>::from_raw(self.key).fmt(f)
                    }
                    SpaEnum::Value(SpaType::ObjectParamIo) => {
                        SpaEnum::<SpaParamIo>::from_raw(self.key).fmt(f)
                    }
                    SpaEnum::Value(SpaType::ObjectParamProfile) => {
                        SpaEnum::<SpaParamProfile>::from_raw(self.key).fmt(f)
                    }
                    SpaEnum::Value(SpaType::ObjectParamPortConfig) => {
                        SpaEnum::<SpaParamPortConfig>::from_raw(self.key).fmt(f)
                    }
                    SpaEnum::Value(SpaType::ObjectParamRoute) => {
                        SpaEnum::<SpaParamRoute>::from_raw(self.key).fmt(f)
                    }
                    SpaEnum::Value(SpaType::ObjectProfiler) => {
                        SpaEnum::<SpaProfiler>::from_raw(self.key).fmt(f)
                    }
                    SpaEnum::Value(SpaType::ObjectParamLatency) => {
                        SpaEnum::<SpaParamLatency>::from_raw(self.key).fmt(f)
                    }
                    SpaEnum::Value(SpaType::ObjectParamProcessLatency) => {
                        SpaEnum::<SpaParamProcessLatency>::from_raw(self.key).fmt(f)
                    }
                    SpaEnum::Value(SpaType::ObjectParamParamTag) => {
                        SpaEnum::<SpaParamTag>::from_raw(self.key).fmt(f)
                    }
                    _ => self.key.fmt(f),
                }
            }
        }

        struct ObjectProps<'a, 'b>(&'b PodObjectDeserializer<'a>);
        impl<'a, 'b> fmt::Debug for ObjectProps<'a, 'b> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut map = f.debug_map();
                for prop in self.0.clone() {
                    map.key(&DbgKey {
                        key: prop.key,
                        object_ty: self.0.object_ty,
                    });
                    map.value(&prop.value);
                }
                map.finish()
            }
        }

        f.debug_struct("Object")
            .field("object_ty", &self.object_ty)
            .field("object_id", &self.object_id)
            .field("properties", &ObjectProps(self))
            .finish()
    }
}

impl<'a> fmt::Debug for PodChoiceDeserializer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let choices: Vec<_> = self.clone().collect();
        f.debug_struct("Choice")
            .field("choice_ty", &self.choice_ty)
            .field("flags", &self.flags)
            .field("child_ty", &self.child_ty)
            .field("value", &choices)
            .finish()
    }
}

impl<'a> fmt::Debug for PodDeserializer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind() {
            PodDeserializerKind::Rectangle(v) => f
                .debug_struct("Rectangle")
                .field("width", &v.width)
                .field("height", &v.height)
                .finish(),
            PodDeserializerKind::Fraction(v) => f
                .debug_struct("Fraction")
                .field("num", &v.num)
                .field("denom", &v.denom)
                .finish(),
            PodDeserializerKind::Bitmap(v) => list_tuple(f, "Bitmap", v.iter()),
            PodDeserializerKind::Bytes(v) => list_tuple(f, "Bytes", v.iter()),
            PodDeserializerKind::Array(v) => list_tuple(f, "Array", v.clone()),
            PodDeserializerKind::Struct(v) => list_tuple(f, "Struct", v.clone()),
            PodDeserializerKind::Object(v) => v.fmt(f),
            PodDeserializerKind::Choice(v) => v.fmt(f),
            PodDeserializerKind::Unknown(pod) => f
                .debug_struct("UnknownPod")
                .field("type", &pod.ty())
                .finish_non_exhaustive(),
            _ => self.kind().fmt(f),
        }
    }
}
