use std::{fmt, mem};

use super::pad_to_8;
use bstr::BStr;
use libspa_consts::{SpaChoiceType, SpaEnum, SpaFraction, SpaRectangle, SpaType};

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

fn read_raw<T: Primitive + Copy>(bytes: &[u8]) -> T {
    T::read_raw(bytes)
}

fn eat_raw<T: Primitive + Copy>(bytes: &[u8]) -> (T, &[u8]) {
    let v = read_raw(bytes);
    (v, &bytes[mem::size_of::<T>()..])
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
            SpaType::String => Kind::String(self.as_string().unwrap()),
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
            // SpaType::Sequence => {},
            SpaType::Fd => Kind::Fd(read_raw(self.body)),
            SpaType::Choice => Kind::Choice(PodChoiceDeserializer::new(self.body)),
            _ => Kind::Unknown(self.clone()),
        }
    }

    pub fn as_string(&self) -> Option<&'a BStr> {
        if self.ty == SpaEnum::Value(SpaType::String) {
            let bytes = &self.body[..self.size as usize];

            let bytes = match bytes.iter().position(|b| *b == 0) {
                Some(end) => &bytes[..end],
                None => bytes,
            };

            Some(BStr::new(bytes))
        } else {
            None
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
    // Seq
    // Pointer
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

    pub fn pop_element(&mut self) -> Option<PodDeserializer<'a>> {
        if self.body.is_empty() {
            return None;
        }

        let pod = PodDeserializer::form_body(self.child_size, self.child_ty, self.body);
        self.body = &self.body[pod.size() as usize..];

        Some(pod)
    }
}

impl<'a> Iterator for PodArrayDeserializer<'a> {
    type Item = PodDeserializer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_element()
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

    pub fn pop_field(&mut self) -> Option<PodDeserializer<'a>> {
        if self.body.is_empty() {
            return None;
        }

        let (pod, remaining) = PodDeserializer::new(self.body);
        self.body = remaining;
        Some(pod)
    }
}

impl<'a> Iterator for PodStructDeserializer<'a> {
    type Item = PodDeserializer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_field()
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

    pub fn pop_property(&mut self) -> Option<PobObjectPropertyDeserializer<'a>> {
        let remaining = self.body;

        if remaining.is_empty() {
            return None;
        }

        let (key, remaining) = eat_raw::<u32>(remaining);
        let (flags, remaining) = eat_raw::<u32>(remaining);
        let (pod, remaining) = PodDeserializer::new(remaining);

        self.body = remaining;

        Some(PobObjectPropertyDeserializer { key, flags, pod })
    }
}

impl<'a> Iterator for PodObjectDeserializer<'a> {
    type Item = PobObjectPropertyDeserializer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_property()
    }
}

pub struct PobObjectPropertyDeserializer<'a> {
    pub key: u32,
    pub flags: u32,
    pub pod: PodDeserializer<'a>,
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

    pub fn pop_element(&mut self) -> Option<PodDeserializer<'a>> {
        if self.body.is_empty() {
            return None;
        }

        let pod = PodDeserializer::form_body(self.child_size, self.child_ty, self.body);
        self.body = &self.body[pod.size() as usize..];

        Some(pod)
    }
}

impl<'a> Iterator for PodChoiceDeserializer<'a> {
    type Item = PodDeserializer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_element()
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
        self.pod.fmt(f)
    }
}

impl<'a> fmt::Debug for PodObjectDeserializer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        list_tuple(f, "Object", self.clone())
    }
}

impl<'a> fmt::Debug for PodChoiceDeserializer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        list_tuple(f, "Choice", self.clone())
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
            PodDeserializerKind::Choice(v) => list_tuple(f, "Choice", v.clone()),
            PodDeserializerKind::Array(v) => list_tuple(f, "Array", v.clone()),
            PodDeserializerKind::Struct(v) => list_tuple(f, "Struct", v.clone()),
            PodDeserializerKind::Object(v) => list_tuple(f, "Object", v.clone()),
            _ => self.kind().fmt(f),
        }
    }
}
