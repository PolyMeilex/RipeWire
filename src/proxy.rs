use libspa_consts::{SpaParamRoute, SpaParamType, SpaProp, SpaType};
use pod::{Object, Property, PropertyFlags, Value};

use crate::{
    context::{Context, WeakContext},
    protocol::{self, pw_client, pw_core, pw_device, pw_registry},
};

pub trait Proxy {
    fn from_id(id: ObjectId, context: &Context) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectId {
    object_id: u32,
}

impl ObjectId {
    pub fn new(object_id: u32) -> Self {
        Self { object_id }
    }

    pub fn protocol_id(&self) -> u32 {
        self.object_id
    }
}

#[derive(Debug, Clone)]
pub struct PwCore {
    object_id: ObjectId,
    context: WeakContext,
}

impl PwCore {
    pub fn new(object_id: u32, context: &Context) -> Self {
        Self {
            object_id: ObjectId::new(object_id),
            context: context.downgrade(),
        }
    }

    pub fn id(&self) -> ObjectId {
        self.object_id.clone()
    }

    pub fn hello(&self, data: pw_core::methods::Hello) {
        if let Some(context) = self.context.upgrade() {
            context
                .send_msg(&protocol::create_msg(0, &data), &[])
                .unwrap();
        }
    }

    pub fn sync(&self, data: pw_core::methods::Sync) {
        if let Some(context) = self.context.upgrade() {
            context
                .send_msg(&protocol::create_msg(0, &data), &[])
                .unwrap();
        }
    }

    pub fn pong(&self, data: pw_core::methods::Pong) {
        if let Some(context) = self.context.upgrade() {
            context
                .send_msg(&protocol::create_msg(0, &data), &[])
                .unwrap();
        }
    }

    pub fn get_registry(&self, mut data: pw_core::methods::GetRegistry) -> PwRegistry {
        if let Some(context) = self.context.upgrade() {
            let new_id = context.new_object();
            data.new_id = new_id.object_id;

            context
                .send_msg(&protocol::create_msg(0, &data), &[])
                .unwrap();

            PwRegistry::new(data.new_id, &context)
        } else {
            todo!()
        }
    }

    pub fn create_object<I: Proxy>(&self, mut data: pw_core::methods::CreateObject) -> I {
        if let Some(context) = self.context.upgrade() {
            let new_id = context.new_object();
            data.new_id = new_id.object_id;

            context
                .send_msg(&protocol::create_msg(0, &data), &[])
                .unwrap();

            I::from_id(new_id, &context)
        } else {
            todo!()
        }
    }
}

#[derive(Debug, Clone)]
pub struct PwClient {
    object_id: ObjectId,
    context: WeakContext,
}

impl PwClient {
    pub fn new(object_id: u32, context: &Context) -> Self {
        Self {
            object_id: ObjectId::new(object_id),
            context: context.downgrade(),
        }
    }

    pub fn id(&self) -> ObjectId {
        self.object_id.clone()
    }

    pub fn update_properties(&self, data: pw_client::methods::UpdateProperties) {
        if let Some(context) = self.context.upgrade() {
            context
                .send_msg(&protocol::create_msg(1, &data), &[])
                .unwrap();
        }
    }

    pub fn get_permissions(self, data: pw_client::methods::GetPermissions) {
        if let Some(context) = self.context.upgrade() {
            context
                .send_msg(&protocol::create_msg(self.object_id.object_id, &data), &[])
                .unwrap();
        }
    }
}

impl Proxy for PwClient {
    fn from_id(object_id: ObjectId, context: &Context) -> Self {
        Self {
            object_id,
            context: context.downgrade(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PwRegistry {
    object_id: ObjectId,
    context: WeakContext,
}

impl PwRegistry {
    pub fn new(object_id: u32, context: &Context) -> Self {
        Self {
            object_id: ObjectId::new(object_id),
            context: context.downgrade(),
        }
    }

    pub fn id(&self) -> ObjectId {
        self.object_id.clone()
    }

    pub fn bind<I: Proxy>(&self, mut data: pw_registry::methods::Bind) -> I {
        if let Some(context) = self.context.upgrade() {
            let new_id = context.new_object();
            data.new_id = new_id.object_id;

            context
                .send_msg(&protocol::create_msg(self.object_id.object_id, &data), &[])
                .unwrap();

            I::from_id(ObjectId::new(data.new_id), &context)
        } else {
            todo!()
        }
    }
}

#[derive(Debug, Clone)]
pub struct PwDevice {
    object_id: ObjectId,
    context: WeakContext,
}

impl Proxy for PwDevice {
    fn from_id(object_id: ObjectId, context: &Context) -> Self {
        Self {
            object_id,
            context: context.downgrade(),
        }
    }
}

impl PwDevice {
    pub fn id(&self) -> ObjectId {
        self.object_id.clone()
    }

    pub fn set_param(&self, param: SpaParamType, value: Value) {
        if let Some(context) = self.context.upgrade() {
            let msg = pw_device::methods::SetParam {
                id: pod::utils::Id(param as u32),
                flags: 0,
                param: value,
            };

            let msg = protocol::create_msg(self.object_id.object_id, &msg);
            context.send_msg(&msg, &[]).unwrap();
        }
    }

    pub fn set_mute(&self, mute: bool) {
        let value = Value::Object(Object {
            type_: SpaType::ObjectParamRoute as u32,
            id: SpaParamType::Route as u32,
            properties: vec![
                Property {
                    key: SpaParamRoute::Index as u32,
                    flags: PropertyFlags::empty(),
                    value: Value::Int(1),
                },
                Property {
                    key: SpaParamRoute::Device as u32,
                    flags: PropertyFlags::empty(),
                    value: Value::Int(0),
                },
                Property {
                    key: SpaParamRoute::Props as u32,
                    flags: PropertyFlags::empty(),
                    value: Value::Object(Object {
                        type_: SpaType::ObjectProps as u32,
                        id: SpaParamType::Route as u32,
                        properties: vec![Property {
                            key: SpaProp::Mute as u32,
                            flags: PropertyFlags::empty(),
                            value: Value::Bool(mute),
                        }],
                    }),
                },
            ],
        });

        self.set_param(SpaParamType::Route, value);
    }
}

#[derive(Debug, Clone)]
pub struct PwClientNode {
    object_id: ObjectId,
    context: WeakContext,
}

impl Proxy for PwClientNode {
    fn from_id(object_id: ObjectId, context: &Context) -> Self {
        Self {
            object_id,
            context: context.downgrade(),
        }
    }
}

impl PwClientNode {
    pub fn id(&self) -> ObjectId {
        self.object_id.clone()
    }
}
