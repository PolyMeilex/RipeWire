use super::pad_to_8;
use libspa_consts::{SpaFraction, SpaRectangle, SpaType};
use std::io;

pub trait PodWrite {
    fn size(&self) -> u32;
    fn write(&self, w: impl std::io::Write) -> io::Result<()>;
}

pub trait TypedPod {
    const SPA_TYPE: SpaType;
}

pub trait Primitive: TypedPod + PodWrite {}
impl<T: PodWrite + TypedPod> Primitive for T {}

macro_rules! impl_typed_pod {
    ($for: ty, $ty: expr) => {
        impl TypedPod for $for {
            const SPA_TYPE: SpaType = $ty;
        }
    };
}

macro_rules! impl_typed_pods {
    ( $(($for: ty, $ty: expr)),* $(,)? ) => {
        $( impl_typed_pod!($for, $ty); )*
    };
}

macro_rules! impl_primitive_write {
    ($for: ty) => {
        impl PodWrite for $for {
            fn size(&self) -> u32 {
                std::mem::size_of::<Self>() as u32
            }

            fn write(&self, mut w: impl std::io::Write) -> io::Result<()> {
                w.write_all(&self.to_ne_bytes())
            }
        }
    };
}

macro_rules! impl_primitive_writes {
    ( $($for: ty),* $(,)? ) => {
        $( impl_primitive_write!($for); )*
    };
}

pub struct SpaBitmap<'a>(&'a [u8]);

impl_typed_pods![
    ((), SpaType::None),
    (bool, SpaType::Bool),
    (i32, SpaType::Int),
    (i64, SpaType::Long),
    (f32, SpaType::Float),
    (f64, SpaType::Double),
    (&str, SpaType::String),
    (&[u8], SpaType::Bytes),
    (SpaRectangle, SpaType::Rectangle),
    (SpaFraction, SpaType::Fraction),
    (SpaBitmap<'_>, SpaType::Bitmap),
];
impl_primitive_writes![i32, i64, f32, f64];

impl PodWrite for () {
    fn size(&self) -> u32 {
        0
    }

    fn write(&self, _w: impl std::io::Write) -> io::Result<()> {
        Ok(())
    }
}

impl PodWrite for bool {
    fn size(&self) -> u32 {
        std::mem::size_of::<i32>() as u32
    }

    fn write(&self, mut w: impl std::io::Write) -> io::Result<()> {
        let v: i32 = match self {
            true => 1,
            false => 0,
        };
        w.write_all(&v.to_ne_bytes())
    }
}

impl PodWrite for &str {
    fn size(&self) -> u32 {
        (self.len() + 1) as u32
    }

    fn write(&self, mut w: impl std::io::Write) -> io::Result<()> {
        let value = std::ffi::CString::new(*self).unwrap().into_bytes_with_nul();
        w.write_all(&value).unwrap();
        Ok(())
    }
}

impl PodWrite for &[u8] {
    fn size(&self) -> u32 {
        self.len() as u32
    }

    fn write(&self, mut w: impl std::io::Write) -> io::Result<()> {
        w.write_all(self)?;
        Ok(())
    }
}

impl PodWrite for SpaRectangle {
    fn size(&self) -> u32 {
        std::mem::size_of::<Self>() as u32
    }

    fn write(&self, mut w: impl std::io::Write) -> io::Result<()> {
        w.write_all(&self.width.to_ne_bytes())?;
        w.write_all(&self.height.to_ne_bytes())?;
        Ok(())
    }
}

impl PodWrite for SpaFraction {
    fn size(&self) -> u32 {
        std::mem::size_of::<Self>() as u32
    }

    fn write(&self, mut w: impl std::io::Write) -> io::Result<()> {
        w.write_all(&self.num.to_ne_bytes())?;
        w.write_all(&self.denom.to_ne_bytes())?;
        Ok(())
    }
}

impl PodWrite for SpaBitmap<'_> {
    fn size(&self) -> u32 {
        self.0.len() as u32
    }

    fn write(&self, mut w: impl std::io::Write) -> io::Result<()> {
        w.write_all(self.0)?;
        Ok(())
    }
}

pub struct Builder<Buff> {
    buff: Buff,
}

impl<Buff> Builder<Buff>
where
    Buff: std::io::Write + std::io::Seek,
{
    pub fn new(buff: Buff) -> Self {
        Self { buff }
    }

    pub fn into_inner(self) -> Buff {
        self.buff
    }

    fn push_header(&mut self, size: u32, ty: SpaType) -> io::Result<()> {
        self.buff.write_all(&size.to_ne_bytes())?;
        self.buff.write_all(&(ty as u32).to_ne_bytes())?;
        Ok(())
    }

    fn push_padding(&mut self, padding: u32) {
        for _ in 0..padding {
            self.buff.write_all(&[0]).unwrap();
        }
    }

    pub fn push<P: Primitive>(&mut self, value: P) -> &mut Self {
        let size = value.size();
        self.push_header(size, P::SPA_TYPE).unwrap();
        value.write(&mut self.buff).unwrap();
        self.push_padding(pad_to_8(size));
        self
    }

    pub fn push_array(&mut self, child_size: u32, child_type: SpaType) -> ArrayBuilder<'_, Buff> {
        let header_start = self.buff.stream_position().unwrap();
        self.push_header(0, SpaType::Array).unwrap();
        let body_start = self.buff.stream_position().unwrap();
        self.push_header(child_size, child_type).unwrap();

        ArrayBuilder {
            builder: self,
            size_initializer: LazySizeInit { header_start },
            body_start,
        }
    }

    pub fn push_array_with(
        &mut self,
        child_size: u32,
        child_type: SpaType,
        cb: impl FnOnce(&mut ArrayBuilder<'_, Buff>),
    ) -> &mut Self {
        let mut builder = self.push_array(child_size, child_type);
        cb(&mut builder);
        builder.done().unwrap();
        self
    }

    pub fn push_struct(&mut self) -> StructBuilder<'_, Buff> {
        let header_start = self.buff.stream_position().unwrap();
        self.push_header(0, SpaType::Struct).unwrap();
        let body_start = self.buff.stream_position().unwrap();

        StructBuilder {
            builder: self,
            size_initializer: LazySizeInit { header_start },
            body_start,
        }
    }

    pub fn push_struct_with(&mut self, cb: impl FnOnce(&mut StructBuilder<'_, Buff>)) -> &mut Self {
        let mut builder = self.push_struct();
        cb(&mut builder);
        builder.done().unwrap();
        self
    }
}

struct LazySizeInit {
    header_start: u64,
}

impl LazySizeInit {
    fn init_size<Buff: io::Write + io::Seek>(&self, buff: &mut Buff, size: u32) -> io::Result<()> {
        // Override first field of the header (size)
        buff.seek(io::SeekFrom::Start(self.header_start))?;
        buff.write_all(&size.to_ne_bytes())?;
        buff.seek(io::SeekFrom::End(0))?;
        Ok(())
    }
}

pub struct StructBuilder<'a, Buff> {
    builder: &'a mut Builder<Buff>,
    size_initializer: LazySizeInit,
    body_start: u64,
}

impl<Buff> std::ops::Deref for StructBuilder<'_, Buff> {
    type Target = Builder<Buff>;
    fn deref(&self) -> &Self::Target {
        self.builder
    }
}
impl<Buff> std::ops::DerefMut for StructBuilder<'_, Buff> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.builder
    }
}

impl<'a, Buff> StructBuilder<'a, Buff>
where
    Buff: io::Write + io::Seek,
{
    pub fn done(self) -> io::Result<()> {
        let pos = self.builder.buff.stream_position()?;
        let size = (pos - self.body_start) as u32;
        self.size_initializer
            .init_size(&mut self.builder.buff, size)?;
        Ok(())
    }
}

pub struct ArrayBuilder<'a, Buff> {
    builder: &'a mut Builder<Buff>,
    size_initializer: LazySizeInit,
    body_start: u64,
}

impl<'a, Buff> ArrayBuilder<'a, Buff>
where
    Buff: io::Write + io::Seek,
{
    pub fn push<P: Primitive>(&mut self, value: P) -> &mut Self {
        value.write(&mut self.builder.buff).unwrap();
        self
    }

    // TODO: Probably not used anywhere, but technically it should be possible to serialize structs
    // and arrays as well

    pub fn done(self) -> io::Result<()> {
        let pos = self.builder.buff.stream_position()?;
        let size = (pos - self.body_start) as u32;
        self.size_initializer
            .init_size(&mut self.builder.buff, size)?;
        self.builder.push_padding(pad_to_8(size));
        Ok(())
    }
}
