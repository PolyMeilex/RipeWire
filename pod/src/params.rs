use spa_sys::{
    SpaDirection, SpaFormat, SpaIoType, SpaMediaSubtype, SpaMediaType, SpaParamAvailability,
    SpaParamIo, SpaParamRoute, SpaParamType, SpaType,
};

use crate::{pod, utils::Id, Object, Property, PropertyFlags, Value};

#[derive(Debug)]
pub struct FormatParamBuilder(Object);

impl FormatParamBuilder {
    pub fn format() -> Self {
        Self(Object {
            type_: SpaType::ObjectFormat,
            id: SpaParamType::Format as u32,
            properties: vec![],
        })
    }

    pub fn enum_format() -> Self {
        Self(Object {
            type_: SpaType::ObjectFormat,
            id: SpaParamType::EnumFormat as u32,
            properties: vec![],
        })
    }

    fn push(mut self, key: SpaFormat, value: Value) -> Self {
        self.0.properties.push(Property {
            key: key as u32,
            flags: PropertyFlags::empty(),
            value,
        });
        self
    }

    pub fn media_type(self, id: SpaMediaType) -> Self {
        self.push(SpaFormat::MediaType, Value::Id(Id(id as u32)))
    }

    pub fn media_subtype(self, id: SpaMediaSubtype) -> Self {
        self.push(SpaFormat::MediaSubtype, Value::Id(Id(id as u32)))
    }

    pub fn build(self) -> Object {
        self.0
    }
}

#[derive(Debug)]
pub struct IoParamBuilder(Object);

impl IoParamBuilder {
    pub fn io() -> Self {
        Self(Object {
            type_: SpaType::ObjectParamIo,
            id: SpaParamType::Io as u32,
            properties: vec![],
        })
    }

    fn push(mut self, key: SpaParamIo, value: Value) -> Self {
        self.0.properties.push(Property {
            key: key as u32,
            flags: PropertyFlags::empty(),
            value,
        });
        self
    }

    pub fn id(self, id: SpaIoType) -> Self {
        self.push(SpaParamIo::Id, Value::Id(Id(id as u32)))
    }

    pub fn size(self, size: u32) -> Self {
        self.push(SpaParamIo::Size, Value::Int(size as i32))
    }

    pub fn build(self) -> Object {
        self.0
    }
}

#[derive(Debug)]
pub struct RouteParamBuilder(Object);

// https://gitlab.freedesktop.org/pipewire/pipewire/-/blob/master/spa/include/spa/param/route-types.h#L25
impl RouteParamBuilder {
    pub fn enum_route() -> Self {
        Self(Object {
            type_: SpaType::ObjectParamRoute,
            id: SpaParamType::EnumRoute as u32,
            properties: vec![],
        })
    }

    pub fn route() -> Self {
        Self(Object {
            type_: SpaType::ObjectParamRoute,
            id: SpaParamType::Route as u32,
            properties: vec![],
        })
    }

    fn push(mut self, key: SpaParamRoute, value: Value) -> Self {
        self.0.properties.push(Property {
            key: key as u32,
            flags: PropertyFlags::empty(),
            value,
        });
        self
    }

    pub fn index(self, index: i32) -> Self {
        self.push(SpaParamRoute::Index, Value::Int(index))
    }

    pub fn direction(self, direction: SpaDirection) -> Self {
        self.push(SpaParamRoute::Direction, Value::Id(Id(direction as u32)))
    }

    pub fn device(self, device: i32) -> Self {
        self.push(SpaParamRoute::Device, Value::Int(device))
    }

    pub fn name(self, name: impl Into<String>) -> Self {
        self.push(SpaParamRoute::Name, Value::String(name.into()))
    }

    pub fn description(self, description: impl Into<String>) -> Self {
        self.push(
            SpaParamRoute::Description,
            Value::String(description.into()),
        )
    }

    pub fn priority(self, priority: i32) -> Self {
        self.push(SpaParamRoute::Priority, Value::Int(priority))
    }

    pub fn available(self, available: SpaParamAvailability) -> Self {
        self.push(SpaParamRoute::Available, Value::Id(Id(available as u32)))
    }

    pub fn info(self, info: Vec<Value>) -> Self {
        self.push(SpaParamRoute::Info, Value::Struct(info))
    }

    pub fn profiles(self, profiles: i32) -> Self {
        self.push(SpaParamRoute::Profiles, Value::Int(profiles))
    }

    pub fn props(self, mut props: pod::Object) -> Self {
        props.id = self.0.id;
        self.push(SpaParamRoute::Props, Value::Object(props))
    }

    pub fn devices(self, devices: i32) -> Self {
        self.push(SpaParamRoute::Devices, Value::Int(devices))
    }

    pub fn profile(self, profile: i32) -> Self {
        self.push(SpaParamRoute::Profile, Value::Int(profile))
    }

    pub fn save(self, save: bool) -> Self {
        self.push(SpaParamRoute::Save, Value::Bool(save))
    }

    pub fn build(self) -> Object {
        self.0
    }
}
