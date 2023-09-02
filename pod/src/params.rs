use spa_sys::SpaDirection;

use crate::{pod, utils::Id, Object, Property, PropertyFlags, Value};

#[derive(Debug)]
pub struct FormatParamBuilder {
    id: spa_sys::SpaParamType,
    properties: Vec<Property>,
}

impl FormatParamBuilder {
    pub fn format() -> Self {
        Self {
            id: spa_sys::SpaParamType::Format,
            properties: vec![],
        }
    }

    pub fn enum_format() -> Self {
        Self {
            id: spa_sys::SpaParamType::EnumFormat,
            properties: vec![],
        }
    }

    pub fn media_type(mut self, id: spa_sys::SpaMediaType) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaFormat::MediaType as u32,
            flags: PropertyFlags::empty(),
            value: Value::Id(Id(id as u32)),
        });
        self
    }

    pub fn media_subtype(mut self, id: spa_sys::SpaMediaSubtype) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaFormat::MediaSubtype as u32,
            flags: PropertyFlags::empty(),
            value: Value::Id(Id(id as u32)),
        });
        self
    }

    pub fn build(self) -> Object {
        Object {
            type_: spa_sys::SpaType::ObjectFormat,
            id: self.id as u32,
            properties: self.properties,
        }
    }
}

#[derive(Debug)]
pub struct IoParamBuilder {
    id: spa_sys::SpaParamType,
    properties: Vec<Property>,
}

impl IoParamBuilder {
    pub fn io() -> Self {
        Self {
            id: spa_sys::SpaParamType::Io,
            properties: vec![],
        }
    }

    pub fn id(mut self, id: spa_sys::SpaIoType) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaParamIo::Id as u32,
            flags: PropertyFlags::empty(),
            value: Value::Id(Id(id as u32)),
        });
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaParamIo::Size as u32,
            flags: PropertyFlags::empty(),
            value: Value::Int(size as i32),
        });
        self
    }

    pub fn build(self) -> Object {
        Object {
            type_: spa_sys::SpaType::ObjectParamIo,
            id: self.id as u32,
            properties: self.properties,
        }
    }
}

#[derive(Debug)]
pub struct RouteParamBuilder {
    id: spa_sys::SpaParamType,
    properties: Vec<Property>,
}

// https://gitlab.freedesktop.org/pipewire/pipewire/-/blob/master/spa/include/spa/param/route-types.h#L25
impl RouteParamBuilder {
    pub fn enum_route() -> Self {
        Self {
            id: spa_sys::SpaParamType::EnumRoute,
            properties: vec![],
        }
    }

    pub fn route() -> Self {
        Self {
            id: spa_sys::SpaParamType::Route,
            properties: vec![],
        }
    }

    pub fn index(mut self, index: i32) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaParamRoute::Index as u32,
            flags: PropertyFlags::empty(),
            value: Value::Int(index),
        });
        self
    }

    pub fn direction(mut self, direction: SpaDirection) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaParamRoute::Direction as u32,
            flags: PropertyFlags::empty(),
            value: Value::Id(Id(direction as u32)),
        });
        self
    }

    pub fn device(mut self, device: i32) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaParamRoute::Device as u32,
            flags: PropertyFlags::empty(),
            value: Value::Int(device),
        });
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaParamRoute::Name as u32,
            flags: PropertyFlags::empty(),
            value: Value::String(name.into()),
        });
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaParamRoute::Description as u32,
            flags: PropertyFlags::empty(),
            value: Value::String(description.into()),
        });
        self
    }

    pub fn priority(mut self, priority: i32) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaParamRoute::Priority as u32,
            flags: PropertyFlags::empty(),
            value: Value::Int(priority),
        });
        self
    }

    pub fn available(mut self, available: Id) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaParamRoute::Available as u32,
            flags: PropertyFlags::empty(),
            value: Value::Id(available),
        });
        self
    }

    pub fn info(mut self, info: Vec<Value>) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaParamRoute::Info as u32,
            flags: PropertyFlags::empty(),
            value: Value::Struct(info),
        });
        self
    }

    pub fn profiles(mut self, profiles: i32) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaParamRoute::Profiles as u32,
            flags: PropertyFlags::empty(),
            value: Value::Int(profiles),
        });
        self
    }

    pub fn props(mut self, props: pod::Object) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaParamRoute::Props as u32,
            flags: PropertyFlags::empty(),
            value: Value::Object(props),
        });
        self
    }

    pub fn devices(mut self, devices: i32) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaParamRoute::Devices as u32,
            flags: PropertyFlags::empty(),
            value: Value::Int(devices),
        });
        self
    }

    pub fn profile(mut self, profile: i32) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaParamRoute::Profile as u32,
            flags: PropertyFlags::empty(),
            value: Value::Int(profile),
        });
        self
    }

    pub fn save(mut self, save: bool) -> Self {
        self.properties.push(Property {
            key: spa_sys::SpaParamRoute::Save as u32,
            flags: PropertyFlags::empty(),
            value: Value::Bool(save),
        });
        self
    }

    pub fn build(self) -> Object {
        Object {
            type_: spa_sys::SpaType::ObjectParamRoute,
            id: self.id as u32,
            properties: self.properties,
        }
    }
}
