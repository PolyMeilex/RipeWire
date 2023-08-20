use libspa_consts::{SpaParamRoute, SpaParamType, SpaProp, SpaType};
use pod::{Object, Property, PropertyFlags, Value};

use crate::{
    context::Context,
    object_map::ObjectType,
    protocol::{self, pw_client, pw_client_node, pw_core, pw_device, pw_registry},
};

pub trait Proxy {
    type Event;

    fn from_id(id: ObjectId) -> Self;
    fn id(&self) -> ObjectId;
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
}

impl PwCore {
    pub fn new(object_id: u32) -> Self {
        Self {
            object_id: ObjectId::new(object_id),
        }
    }

    pub fn id(&self) -> ObjectId {
        self.object_id.clone()
    }

    pub fn hello<D>(&self, context: &mut Context<D>, data: pw_core::methods::Hello) {
        context
            .send_msg(&protocol::create_msg(0, &data), &[])
            .unwrap();
    }

    pub fn sync<D>(&self, context: &mut Context<D>, data: pw_core::methods::Sync) {
        context
            .send_msg(&protocol::create_msg(0, &data), &[])
            .unwrap();
    }

    pub fn pong<D>(&self, context: &mut Context<D>, data: pw_core::methods::Pong) {
        context
            .send_msg(&protocol::create_msg(0, &data), &[])
            .unwrap();
    }

    pub fn get_registry<D>(
        &self,
        context: &mut Context<D>,
        mut data: pw_core::methods::GetRegistry,
    ) -> PwRegistry {
        let new_id = context.new_object(ObjectType::Registry);
        data.new_id = new_id.object_id;

        context
            .send_msg(&protocol::create_msg(0, &data), &[])
            .unwrap();

        PwRegistry::new(data.new_id)
    }

    pub fn create_object<I: Proxy, D>(
        &self,
        context: &mut Context<D>,
        mut data: pw_core::methods::CreateObject,
    ) -> I {
        let new_id = context.new_object(ObjectType::from_interface_name(&data.obj_type));
        data.new_id = new_id.object_id;

        context
            .send_msg(&protocol::create_msg(0, &data), &[])
            .unwrap();

        I::from_id(new_id)
    }
}

impl Proxy for PwCore {
    type Event = pw_core::Event;

    fn from_id(object_id: ObjectId) -> Self {
        Self { object_id }
    }

    fn id(&self) -> ObjectId {
        self.object_id.clone()
    }
}

#[derive(Debug, Clone)]
pub struct PwClient {
    object_id: ObjectId,
}

impl PwClient {
    pub fn new(object_id: u32) -> Self {
        Self {
            object_id: ObjectId::new(object_id),
        }
    }

    pub fn id(&self) -> ObjectId {
        self.object_id.clone()
    }

    pub fn update_properties<D>(
        &self,
        context: &mut Context<D>,
        data: pw_client::methods::UpdateProperties,
    ) {
        context
            .send_msg(&protocol::create_msg(1, &data), &[])
            .unwrap();
    }

    pub fn get_permissions<D>(
        self,
        context: &mut Context<D>,
        data: pw_client::methods::GetPermissions,
    ) {
        context
            .send_msg(&protocol::create_msg(self.object_id.object_id, &data), &[])
            .unwrap();
    }
}

impl Proxy for PwClient {
    type Event = pw_client::Event;

    fn from_id(object_id: ObjectId) -> Self {
        Self { object_id }
    }

    fn id(&self) -> ObjectId {
        self.object_id.clone()
    }
}

#[derive(Debug, Clone)]
pub struct PwRegistry {
    object_id: ObjectId,
}

impl PwRegistry {
    pub fn new(object_id: u32) -> Self {
        Self {
            object_id: ObjectId::new(object_id),
        }
    }

    pub fn id(&self) -> ObjectId {
        self.object_id.clone()
    }

    pub fn bind<I: Proxy, D>(
        &self,
        context: &mut Context<D>,
        global: &pw_registry::events::Global,
    ) -> I {
        let data = pw_registry::methods::Bind {
            id: global.id,
            obj_type: global.obj_type.clone(),
            version: global.version,
            new_id: context
                .new_object(ObjectType::from_interface_name(&global.obj_type))
                .protocol_id(),
        };

        context
            .send_msg(&protocol::create_msg(self.object_id.object_id, &data), &[])
            .unwrap();

        I::from_id(ObjectId::new(data.new_id))
    }
}

impl Proxy for PwRegistry {
    type Event = pw_registry::Event;

    fn from_id(object_id: ObjectId) -> Self {
        Self { object_id }
    }

    fn id(&self) -> ObjectId {
        self.object_id.clone()
    }
}

#[derive(Debug, Clone)]
pub struct PwDevice {
    object_id: ObjectId,
}

impl Proxy for PwDevice {
    type Event = pw_device::Event;

    fn from_id(object_id: ObjectId) -> Self {
        Self { object_id }
    }

    fn id(&self) -> ObjectId {
        self.object_id.clone()
    }
}

impl PwDevice {
    pub fn id(&self) -> ObjectId {
        self.object_id.clone()
    }

    pub fn set_param<D>(&self, context: &mut Context<D>, param: SpaParamType, value: Value) {
        let msg = pw_device::methods::SetParam {
            id: pod::utils::Id(param as u32),
            flags: 0,
            param: value,
        };

        let msg = protocol::create_msg(self.object_id.object_id, &msg);
        context.send_msg(&msg, &[]).unwrap();
    }

    pub fn set_mute<D>(&self, context: &mut Context<D>, index: i32, device: i32, mute: bool) {
        let value = Value::Object(Object {
            type_: SpaType::ObjectParamRoute as u32,
            id: SpaParamType::Route as u32,
            properties: vec![
                Property {
                    key: SpaParamRoute::Index as u32,
                    flags: PropertyFlags::empty(),
                    value: Value::Int(index),
                },
                Property {
                    key: SpaParamRoute::Device as u32,
                    flags: PropertyFlags::empty(),
                    value: Value::Int(device),
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

        self.set_param(context, SpaParamType::Route, value);
    }
}

#[derive(Debug, Clone)]
pub struct PwClientNode {
    object_id: ObjectId,
}

impl Proxy for PwClientNode {
    type Event = pw_client_node::Event;

    fn from_id(object_id: ObjectId) -> Self {
        Self { object_id }
    }

    fn id(&self) -> ObjectId {
        self.object_id.clone()
    }
}

impl PwClientNode {
    pub fn id(&self) -> ObjectId {
        self.object_id.clone()
    }
}
