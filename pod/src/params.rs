use crate::{utils::Id, Object, Property, PropertyFlags, Value};

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
            type_: spa_sys::SpaObjectType::Format,
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
            type_: spa_sys::SpaObjectType::ParamIo,
            id: self.id as u32,
            properties: self.properties,
        }
    }
}
