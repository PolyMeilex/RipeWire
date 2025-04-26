use std::collections::HashMap;

use libspa_consts::{SpaEnum, SpaParamType};
use pod::{Object, Value};

use crate::{
    context::Context,
    object_map::ObjectType,
    protocol::{
        self, pw_client, pw_client_node, pw_core, pw_device, pw_link, pw_node, pw_port, pw_registry,
    },
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

    pub fn hello<D>(&self, context: &mut Context<D>) {
        let data = pw_core::methods::Hello { version: 3 };
        context
            .send_msg(&protocol::create_msg2(0, &data), &[])
            .unwrap();
    }

    pub fn sync<D>(&self, context: &mut Context<D>, id: u32, seq: u32) {
        let data = pw_core::methods::Sync { id, seq };
        context
            .send_msg(&protocol::create_msg2(0, &data), &[])
            .unwrap();
    }

    pub fn pong<D>(&self, context: &mut Context<D>, id: u32, seq: u32) {
        let data = pw_core::methods::Pong { id, seq };
        context
            .send_msg(&protocol::create_msg2(0, &data), &[])
            .unwrap();
    }

    pub fn get_registry<D>(&self, context: &mut Context<D>) -> PwRegistry {
        let data = pw_core::methods::GetRegistry {
            version: 3,
            new_id: context.new_object(ObjectType::Registry).protocol_id(),
        };

        context
            .send_msg(&protocol::create_msg2(0, &data), &[])
            .unwrap();

        PwRegistry::new(data.new_id)
    }

    pub fn destroy_object<D>(&self, context: &mut Context<D>, object_id: ObjectId) {
        let data = pw_core::methods::Destroy {
            id: object_id.protocol_id(),
        };
        context
            .send_msg(&protocol::create_msg2(0, &data), &[])
            .unwrap();
    }

    pub fn create_object<I: Proxy, D>(
        &self,
        context: &mut Context<D>,
        mut data: pw_core::methods::CreateObject,
    ) -> I {
        let new_id = context.new_object(ObjectType::from_interface_name(&data.interface));
        data.new_id = new_id.object_id;

        context
            .send_msg(&protocol::create_msg2(0, &data), &[])
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
        properties: HashMap<String, String>,
    ) {
        let data = pw_client::methods::UpdateProperties { properties };
        context
            .send_msg(&protocol::create_msg2(1, &data), &[])
            .unwrap();
    }

    pub fn get_permissions<D>(&self, context: &mut Context<D>, index: u32, num: u32) {
        let data = pw_client::methods::GetPermissions { index, num };
        context
            .send_msg(&protocol::create_msg2(self.object_id.object_id, &data), &[])
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
            interface: global.interface.as_interface_name().to_string(),
            version: global.version,
            new_id: context.new_object(global.interface.clone()).protocol_id(),
        };

        context
            .send_msg(&protocol::create_msg(self.object_id.object_id, &data), &[])
            .unwrap();

        I::from_id(ObjectId::new(data.new_id))
    }

    pub fn destroy_global<D>(&self, context: &mut Context<D>, global: u32) {
        context
            .send_msg(
                &protocol::create_msg(
                    self.object_id.object_id,
                    &pw_registry::methods::Destroy { id: global },
                ),
                &[],
            )
            .unwrap();
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

    pub fn enum_param<D>(&self, context: &mut Context<D>, id: SpaParamType) {
        let msg = pw_device::methods::EnumParams {
            seq: 0,
            id: pod::utils::Id(id as u32),
            index: 0,
            num: 0,
            filter: pod_v2::Builder::with(|b| {
                b.write_none();
            }),
        };

        let msg = protocol::create_msg2(self.object_id.object_id, &msg);
        context.send_msg(&msg, &[]).unwrap();
    }

    pub fn set_param<D>(&self, context: &mut Context<D>, value: Object) {
        let param = pod_v2::Builder::with(|b| {
            b.write_object_with(SpaEnum::Unknown(value.type_ as u32), value.id, |b| {
                for v in value.properties {
                    b.write_property(v.key, v.flags.bits(), |b| {
                        b.write_pod(&pod_v2::serialize::OwnedPod(
                            pod::PodSerializer::serialize2(&v.value),
                        ));
                    });
                }
            });
        });

        let msg = pw_device::methods::SetParam {
            id: pod::utils::Id(value.id),
            flags: 0,
            param,
        };

        let msg = protocol::create_msg2(self.object_id.object_id, &msg);
        context.send_msg(&msg, &[]).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct PwNode {
    object_id: ObjectId,
}

impl Proxy for PwNode {
    type Event = pw_node::Event;

    fn from_id(object_id: ObjectId) -> Self {
        Self { object_id }
    }

    fn id(&self) -> ObjectId {
        self.object_id.clone()
    }
}

impl PwNode {
    pub fn id(&self) -> ObjectId {
        self.object_id.clone()
    }

    pub fn enum_param<D>(&self, context: &mut Context<D>, id: SpaParamType) {
        let msg = pw_node::methods::EnumParams {
            seq: 0,
            id: pod::utils::Id(id as u32),
            index: 0,
            num: 0,
            filter: pod::Value::None,
        };

        let msg = protocol::create_msg(self.object_id.object_id, &msg);
        context.send_msg(&msg, &[]).unwrap();
    }

    pub fn set_param<D>(&self, context: &mut Context<D>, value: Object) {
        let msg = pw_node::methods::SetParam {
            id: pod::utils::Id(value.id),
            flags: 0,
            param: Value::Object(value),
        };

        let msg = protocol::create_msg(self.object_id.object_id, &msg);
        context.send_msg(&msg, &[]).unwrap();
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

#[derive(Debug, Clone)]
pub struct PwLink {
    object_id: ObjectId,
}

impl Proxy for PwLink {
    type Event = pw_link::Event;

    fn from_id(object_id: ObjectId) -> Self {
        Self { object_id }
    }

    fn id(&self) -> ObjectId {
        self.object_id.clone()
    }
}

impl PwLink {
    pub fn id(&self) -> ObjectId {
        self.object_id.clone()
    }
}

#[derive(Debug, Clone)]
pub struct PwPort {
    object_id: ObjectId,
}

impl Proxy for PwPort {
    type Event = pw_port::Event;

    fn from_id(object_id: ObjectId) -> Self {
        Self { object_id }
    }

    fn id(&self) -> ObjectId {
        self.object_id.clone()
    }
}

impl PwPort {
    pub fn id(&self) -> ObjectId {
        self.object_id.clone()
    }

    pub fn enum_params<D>(&self, context: &mut Context<D>, id: SpaParamType) {
        let msg = pw_device::methods::EnumParams {
            seq: 0,
            id: pod::utils::Id(id as u32),
            index: 0,
            num: 0,
            filter: pod_v2::Builder::with(|b| {
                b.write_none();
            }),
        };

        let msg = protocol::create_msg2(self.object_id.object_id, &msg);
        context.send_msg(&msg, &[]).unwrap();
    }
}
