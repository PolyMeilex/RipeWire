use std::mem::size_of;

use super::pad_to_8;
use bstr::BStr;
use libspa_consts::{SpaChoiceType, SpaFraction, SpaRectangle, SpaType};

macro_rules! arr4 {
    ($b: expr, $off: expr) => {
        [$b[0 + $off], $b[1 + $off], $b[2 + $off], $b[3 + $off]]
    };
    ($b: expr) => {
        arr4!($b, 0)
    };
}
macro_rules! arr8 {
    ($b: expr) => {
        [$b[0], $b[1], $b[2], $b[3], $b[4], $b[5], $b[6], $b[7]]
    };
}

#[repr(C)]
struct SpaPodHeader {
    size: u32,
    ty: SpaType,
}

#[derive(Clone)]
pub struct PodDeserializer<'a> {
    size: u32,
    ty: SpaType,
    body: &'a [u8],
}

impl<'a> PodDeserializer<'a> {
    pub fn new(buff: &'a [u8]) -> (Self, &'a [u8]) {
        let size = u32::from_ne_bytes(arr4!(buff));
        let ty = u32::from_ne_bytes(arr4!(buff, 4));
        let ty = SpaType::from_raw(ty).unwrap();

        Self::form_body(size, true, ty, &buff[size_of::<SpaPodHeader>()..])
    }

    fn form_body(size: u32, padding: bool, ty: SpaType, body: &'a [u8]) -> (Self, &'a [u8]) {
        let padding = if padding { pad_to_8(size) } else { 0 };
        let padded_size = (size + padding) as usize;

        let pod = Self {
            size,
            ty,
            body: &body[..size as usize],
        };

        (pod, &body[padded_size..])
    }

    pub fn ty(&self) -> SpaType {
        self.ty
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn body(&self) -> &'a [u8] {
        self.body
    }

    pub fn kind(&self) -> PodDeserializerKind<'a> {
        match self.ty {
            SpaType::None => PodDeserializerKind::None,
            SpaType::Bool => PodDeserializerKind::Bool(i32::from_ne_bytes(arr4!(self.body)) != 0),
            SpaType::Id => PodDeserializerKind::Id(u32::from_ne_bytes(arr4!(self.body))),
            SpaType::Int => PodDeserializerKind::Int(i32::from_ne_bytes(arr4!(self.body))),
            SpaType::Long => PodDeserializerKind::Long(i64::from_ne_bytes(arr8!(self.body))),
            SpaType::Float => PodDeserializerKind::Float(f32::from_ne_bytes(arr4!(self.body))),
            SpaType::Double => PodDeserializerKind::Double(f64::from_ne_bytes(arr8!(self.body))),
            SpaType::String => PodDeserializerKind::String(self.as_string().unwrap()),
            SpaType::Bytes => PodDeserializerKind::Bytes(self.body),
            SpaType::Rectangle => {
                let rect = SpaRectangle {
                    width: u32::from_ne_bytes(arr4!(self.body)),
                    height: u32::from_ne_bytes(arr4!(self.body, 4)),
                };
                PodDeserializerKind::Rectangle(rect)
            }
            SpaType::Fraction => {
                let rect = SpaFraction {
                    num: u32::from_ne_bytes(arr4!(self.body)),
                    denom: u32::from_ne_bytes(arr4!(self.body, 4)),
                };
                PodDeserializerKind::Fraction(rect)
            }
            SpaType::Bitmap => PodDeserializerKind::Bitmap(self.body),
            SpaType::Array => PodDeserializerKind::Array(self.as_array().unwrap()),
            SpaType::Struct => PodDeserializerKind::Struct(self.as_struct().unwrap()),
            SpaType::Object => PodDeserializerKind::Object(self.as_object().unwrap()),
            // SpaType::Sequence => {},
            SpaType::Fd => PodDeserializerKind::Fd(i64::from_ne_bytes(arr8!(self.body))),
            SpaType::Choice => PodDeserializerKind::Choice(self.as_choice().unwrap()),
            _ => PodDeserializerKind::Unknown(self.clone()),
        }
    }

    pub fn as_string(&self) -> Option<&'a BStr> {
        if self.ty == SpaType::String {
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

    pub fn as_struct(&self) -> Option<PodStructDeserializer<'a>> {
        if self.ty == SpaType::Struct {
            Some(PodStructDeserializer::new(self.size, self.ty, self.body))
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<PodArrayDeserializer<'a>> {
        if self.ty == SpaType::Array {
            Some(PodArrayDeserializer::new(self.size, self.ty, self.body))
        } else {
            None
        }
    }

    pub fn as_object(&self) -> Option<PodObjectDeserializer<'a>> {
        if self.ty == SpaType::Object {
            Some(PodObjectDeserializer::new(self.size, self.ty, self.body))
        } else {
            None
        }
    }

    pub fn as_choice(&self) -> Option<PodChoiceDeserializer<'a>> {
        if self.ty == SpaType::Choice {
            Some(PodChoiceDeserializer::new(self.size, self.ty, self.body))
        } else {
            None
        }
    }
}

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

#[allow(unused)]
pub struct PodArrayDeserializer<'a> {
    size: u32,
    ty: SpaType,
    child_size: u32,
    child_ty: SpaType,
    body: &'a [u8],
}

impl<'a> PodArrayDeserializer<'a> {
    fn new(size: u32, ty: SpaType, body: &'a [u8]) -> Self {
        assert_eq!(ty, SpaType::Array);

        let child_size = u32::from_ne_bytes(arr4!(body));
        let body = &body[size_of::<u32>()..];

        let child_ty = u32::from_ne_bytes(arr4!(body));
        let child_ty = SpaType::from_raw(child_ty).unwrap();
        let body = &body[size_of::<u32>()..];

        Self {
            size,
            ty,
            child_size,
            child_ty,
            body,
        }
    }

    pub fn pop_element(&mut self) -> Option<PodDeserializer<'a>> {
        if self.body.is_empty() {
            return None;
        }

        let (pod, remaining) =
            PodDeserializer::form_body(self.child_size, false, self.child_ty, self.body);

        self.body = remaining;

        Some(pod)
    }
}

impl<'a> Iterator for PodArrayDeserializer<'a> {
    type Item = PodDeserializer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_element()
    }
}

#[allow(unused)]
pub struct PodStructDeserializer<'a> {
    size: u32,
    ty: SpaType,
    body: &'a [u8],
}

impl<'a> PodStructDeserializer<'a> {
    fn new(size: u32, ty: SpaType, body: &'a [u8]) -> Self {
        assert_eq!(ty, SpaType::Struct);
        Self { size, ty, body }
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

#[allow(unused)]
pub struct PodObjectDeserializer<'a> {
    size: u32,
    ty: SpaType,
    object_ty: SpaType,
    object_id: u32,
    body: &'a [u8],
}

impl<'a> PodObjectDeserializer<'a> {
    fn new(size: u32, ty: SpaType, body: &'a [u8]) -> Self {
        assert_eq!(ty, SpaType::Object);

        let object_ty = u32::from_ne_bytes(arr4!(body));
        let object_ty = SpaType::from_raw(object_ty).unwrap();
        let body = &body[size_of::<u32>()..];

        let object_id = u32::from_ne_bytes(arr4!(body));
        let body = &body[size_of::<u32>()..];

        Self {
            size,
            ty,
            object_id,
            object_ty,
            body,
        }
    }

    pub fn object_ty(&self) -> SpaType {
        self.object_ty
    }

    pub fn object_id(&self) -> u32 {
        self.object_id
    }

    pub fn pop_property(&mut self) -> Option<PobObjectPropertyDeserializer<'a>> {
        if self.body.is_empty() {
            return None;
        }

        let key = u32::from_ne_bytes(arr4!(self.body));
        self.body = &self.body[size_of::<u32>()..];

        let flags = u32::from_ne_bytes(arr4!(self.body));
        self.body = &self.body[size_of::<u32>()..];

        let (pod, remaining) = PodDeserializer::new(self.body);

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

#[allow(unused)]
pub struct PodChoiceDeserializer<'a> {
    size: u32,
    ty: SpaType,
    choice_ty: SpaChoiceType,
    flags: u32,

    child_size: u32,
    child_ty: SpaType,
    body: &'a [u8],
}

impl<'a> PodChoiceDeserializer<'a> {
    fn new(size: u32, ty: SpaType, body: &'a [u8]) -> Self {
        assert_eq!(ty, SpaType::Choice);

        let choice_ty = u32::from_ne_bytes(arr4!(body));
        let choice_ty = SpaChoiceType::from_raw(choice_ty).unwrap();
        let body = &body[size_of::<u32>()..];

        let flags = u32::from_ne_bytes(arr4!(body));
        let body = &body[size_of::<u32>()..];

        let child_size = u32::from_ne_bytes(arr4!(body));
        let body = &body[size_of::<u32>()..];

        let child_ty = u32::from_ne_bytes(arr4!(body));
        let child_ty = SpaType::from_raw(child_ty).unwrap();
        let body = &body[size_of::<u32>()..];

        Self {
            size,
            ty,
            choice_ty,
            flags,
            child_size,
            child_ty,
            body,
        }
    }

    pub fn choice_ty(&self) -> SpaChoiceType {
        self.choice_ty
    }

    pub fn pop_element(&mut self) -> Option<PodDeserializer<'a>> {
        if self.body.is_empty() {
            return None;
        }

        let (pod, remaining) =
            PodDeserializer::form_body(self.child_size, false, self.child_ty, self.body);

        self.body = remaining;

        Some(pod)
    }
}

impl<'a> Iterator for PodChoiceDeserializer<'a> {
    type Item = PodDeserializer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_element()
    }
}
