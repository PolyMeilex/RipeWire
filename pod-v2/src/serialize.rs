use super::pad_to_8;
use libspa_consts::SpaType;
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

#[derive(Clone, Copy)]
struct BuilderFrame {
    array_mode: bool,
    is_first: bool,
}

impl Default for BuilderFrame {
    fn default() -> Self {
        Self {
            array_mode: false,
            is_first: true,
        }
    }
}

pub struct Builder<Buff> {
    buff: Buff,
    frame: BuilderFrame,
}

impl<Buff> Builder<Buff>
where
    Buff: std::io::Write + std::io::Seek,
{
    pub fn new(buff: Buff) -> Self {
        Self {
            buff,
            frame: BuilderFrame::default(),
        }
    }

    pub fn into_inner(self) -> Buff {
        self.buff
    }

    fn write_header(&mut self, size: u32, ty: SpaType) -> io::Result<()> {
        if !self.frame.array_mode || self.frame.is_first {
            self.buff.write_all(&size.to_ne_bytes())?;
            self.buff.write_all(&(ty as u32).to_ne_bytes())?;
            self.frame.is_first = false;
        }
        Ok(())
    }

    fn write_padding(&mut self, padding: u32) -> io::Result<()> {
        if self.frame.array_mode {
            return Ok(());
        }

        for _ in 0..padding {
            self.buff.write_all(&[0])?;
        }
        Ok(())
    }

    pub fn write_none(&mut self) -> &mut Self {
        self.write_header(0, SpaType::None).unwrap();
        self
    }

    fn write_primitive(&mut self, size: u32, ty: SpaType, v: &[u8]) {
        self.write_header(size, ty).unwrap();
        self.buff.write_all(v).unwrap();
        self.write_padding(pad_to_8(size)).unwrap();
    }

    pub fn write_bool(&mut self, v: bool) -> &mut Self {
        self.write_primitive(4, SpaType::Bool, &(v as i32).to_ne_bytes());
        self
    }

    pub fn write_id(&mut self, v: u32) -> &mut Self {
        self.write_primitive(4, SpaType::Id, &v.to_ne_bytes());
        self
    }

    pub fn write_int(&mut self, v: i32) -> &mut Self {
        self.write_primitive(4, SpaType::Int, &v.to_ne_bytes());
        self
    }

    pub fn write_i32(&mut self, v: i32) -> &mut Self {
        self.write_int(v);
        self
    }

    pub fn write_u32(&mut self, v: u32) -> &mut Self {
        self.write_int(v as i32);
        self
    }

    pub fn write_long(&mut self, v: i64) -> &mut Self {
        self.write_primitive(8, SpaType::Long, &v.to_ne_bytes());
        self
    }

    pub fn write_float(&mut self, v: f32) -> &mut Self {
        self.write_primitive(4, SpaType::Float, &v.to_ne_bytes());
        self
    }

    pub fn write_double(&mut self, v: f64) -> &mut Self {
        self.write_primitive(8, SpaType::Double, &v.to_ne_bytes());
        self
    }

    pub fn write_str(&mut self, v: impl AsRef<[u8]>) -> &mut Self {
        let v = v.as_ref();
        let size = v.len() as u32 + 1;
        self.write_header(size, SpaType::String).unwrap();
        self.buff.write_all(v).unwrap();
        self.buff.write_all(&[0]).unwrap();
        self.write_padding(pad_to_8(size)).unwrap();
        self
    }

    pub fn write_bytes(&mut self, v: impl AsRef<[u8]>) -> &mut Self {
        let v = v.as_ref();
        let size = v.len() as u32;
        self.write_header(size, SpaType::Bytes).unwrap();
        self.buff.write_all(v).unwrap();
        self.write_padding(pad_to_8(size)).unwrap();
        self
    }

    pub fn write_rectangle(&mut self, width: u32, height: u32) -> &mut Self {
        self.write_header(8, SpaType::Rectangle).unwrap();
        self.buff.write_all(&width.to_ne_bytes()).unwrap();
        self.buff.write_all(&height.to_ne_bytes()).unwrap();
        self
    }

    pub fn write_fraction(&mut self, num: u32, denom: u32) -> &mut Self {
        self.write_header(8, SpaType::Fraction).unwrap();
        self.buff.write_all(&num.to_ne_bytes()).unwrap();
        self.buff.write_all(&denom.to_ne_bytes()).unwrap();
        self
    }

    pub fn write_bitmap(&mut self, v: impl AsRef<[u8]>) -> &mut Self {
        let v = v.as_ref();
        let size = v.len() as u32;
        self.write_header(size, SpaType::Bitmap).unwrap();
        self.buff.write_all(v).unwrap();
        self.write_padding(pad_to_8(size)).unwrap();
        self
    }

    pub fn write_array_with(&mut self, cb: impl FnOnce(&mut ArrayBuilder<'_, Buff>)) -> &mut Self {
        let mut builder = ArrayBuilder::new(self);
        cb(&mut builder);
        builder.done().unwrap();
        self
    }

    pub fn push_struct_with(&mut self, cb: impl FnOnce(&mut StructBuilder<'_, Buff>)) -> &mut Self {
        let mut builder = StructBuilder::new(self);
        cb(&mut builder);
        builder.done().unwrap();
        self
    }
}

fn lazy_init_size<Buff: io::Write + io::Seek>(
    header_start: u64,
    buff: &mut Buff,
    size: u32,
) -> io::Result<()> {
    // Override first field of the header (size)
    buff.seek(io::SeekFrom::Start(header_start))?;
    buff.write_all(&size.to_ne_bytes())?;
    buff.seek(io::SeekFrom::End(0))?;
    Ok(())
}

pub struct StructBuilder<'a, Buff> {
    builder: &'a mut Builder<Buff>,
    header_start: u64,
    body_start: u64,
    parent_frame: BuilderFrame,
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
    fn new(builder: &'a mut Builder<Buff>) -> Self {
        let header_start = builder.buff.stream_position().unwrap();
        builder.write_header(0, SpaType::Struct).unwrap();
        let body_start = builder.buff.stream_position().unwrap();

        let parent_frame = std::mem::take(&mut builder.frame);

        Self {
            builder,
            header_start,
            body_start,
            parent_frame,
        }
    }

    fn done(self) -> io::Result<()> {
        let pos = self.builder.buff.stream_position()?;
        let size = (pos - self.body_start) as u32;
        lazy_init_size(self.header_start, &mut self.builder.buff, size)?;

        self.builder.frame = self.parent_frame;

        Ok(())
    }
}

pub struct ArrayBuilder<'a, Buff> {
    builder: &'a mut Builder<Buff>,
    header_start: u64,
    body_start: u64,
    parent_frame: BuilderFrame,
}

impl<Buff> std::ops::Deref for ArrayBuilder<'_, Buff> {
    type Target = Builder<Buff>;
    fn deref(&self) -> &Self::Target {
        self.builder
    }
}
impl<Buff> std::ops::DerefMut for ArrayBuilder<'_, Buff> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.builder
    }
}

impl<'a, Buff> ArrayBuilder<'a, Buff>
where
    Buff: io::Write + io::Seek,
{
    fn new(builder: &'a mut Builder<Buff>) -> Self {
        let header_start = builder.buff.stream_position().unwrap();
        builder.write_header(0, SpaType::Array).unwrap();
        let body_start = builder.buff.stream_position().unwrap();

        let parent_frame = std::mem::take(&mut builder.frame);
        builder.frame.is_first = true;
        builder.frame.array_mode = true;

        Self {
            builder,
            header_start,
            body_start,
            parent_frame,
        }
    }

    pub fn write_array_with(&mut self, _cb: impl FnOnce(&mut ArrayBuilder<'_, Buff>)) -> &mut Self {
        todo!()
    }

    pub fn push_struct_with(
        &mut self,
        _cb: impl FnOnce(&mut StructBuilder<'_, Buff>),
    ) -> &mut Self {
        todo!()
    }

    fn done(self) -> io::Result<()> {
        let pos = self.builder.buff.stream_position()?;
        let size = (pos - self.body_start) as u32;
        lazy_init_size(self.header_start, &mut self.builder.buff, size)?;

        self.builder.frame = self.parent_frame;
        self.builder.write_padding(pad_to_8(size)).unwrap();

        Ok(())
    }
}
