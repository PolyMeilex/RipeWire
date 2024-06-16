use spa_sys::{SpaParamType, SpaProp, SpaType};

use crate::{Object, Property, PropertyFlags, Value};

#[derive(Debug)]
pub struct PropsBuilder(Object);

impl Default for PropsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PropsBuilder {
    pub fn new() -> Self {
        Self(Object {
            type_: SpaType::ObjectProps,
            id: SpaParamType::Invalid as u32,
            properties: vec![],
        })
    }

    fn push(mut self, key: SpaProp, value: Value) -> Self {
        self.0.properties.push(Property {
            key: key as u32,
            flags: PropertyFlags::empty(),
            value,
        });
        self
    }

    pub fn volume(self, volume: f32) -> Self {
        self.push(SpaProp::Volume, Value::Float(volume))
    }

    pub fn mute(self, mute: bool) -> Self {
        self.push(SpaProp::Mute, Value::Bool(mute))
    }

    pub fn soft_mute(self, mute: bool) -> Self {
        self.push(SpaProp::SoftMute, Value::Bool(mute))
    }

    pub fn build(self) -> Object {
        self.0
    }
}
