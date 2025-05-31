use std::collections::HashMap;

use libspa_consts::{SpaDirection, SpaEnum, SpaParamType};
use pod::Id;

use crate::{
    context::Context,
    object_map::ObjectType,
    protocol::{
        self, pw_client, pw_client_node, pw_core, pw_device, pw_link, pw_node, pw_port,
        pw_registry, MethodSerialize,
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

    pub fn send<D>(&self, context: &mut Context<D>, message: impl MethodSerialize) {
        let (msg, fds) = protocol::create_msg_with_fds(self.object_id.object_id, &message);
        context.send_msg(&msg, fds.as_slice()).unwrap();
    }

    pub fn hello<D>(&self, context: &mut Context<D>) {
        self.send(context, pw_core::methods::Hello { version: 3 });
    }

    pub fn sync<D>(&self, context: &mut Context<D>, id: u32, seq: u32) {
        self.send(context, pw_core::methods::Sync { id, seq });
    }

    pub fn pong<D>(&self, context: &mut Context<D>, id: u32, seq: u32) {
        self.send(context, pw_core::methods::Pong { id, seq });
    }

    pub fn get_registry<D>(&self, context: &mut Context<D>) -> PwRegistry {
        let new_id = context.new_object(ObjectType::Registry).protocol_id();

        self.send(
            context,
            pw_core::methods::GetRegistry { version: 3, new_id },
        );

        PwRegistry::new(new_id)
    }

    pub fn destroy_object<D>(&self, context: &mut Context<D>, object_id: ObjectId) {
        self.send(
            context,
            pw_core::methods::Destroy {
                id: object_id.protocol_id(),
            },
        );
    }

    pub fn create_object<I: Proxy, D>(
        &self,
        context: &mut Context<D>,
        mut data: pw_core::methods::CreateObject,
    ) -> I {
        let new_id = context.new_object(ObjectType::from_interface_name(&data.interface));
        data.new_id = new_id.object_id;

        self.send(context, data);

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

    pub fn send<D>(&self, context: &mut Context<D>, message: impl MethodSerialize) {
        let (msg, fds) = protocol::create_msg_with_fds(self.object_id.object_id, &message);
        context.send_msg(&msg, fds.as_slice()).unwrap();
    }

    pub fn update_properties<D>(
        &self,
        context: &mut Context<D>,
        properties: HashMap<String, String>,
    ) {
        self.send(context, pw_client::methods::UpdateProperties { properties });
    }

    pub fn get_permissions<D>(&self, context: &mut Context<D>, index: u32, num: u32) {
        self.send(context, pw_client::methods::GetPermissions { index, num });
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

    pub fn send<D>(&self, context: &mut Context<D>, message: impl MethodSerialize) {
        let (msg, fds) = protocol::create_msg_with_fds(self.object_id.object_id, &message);
        context.send_msg(&msg, fds.as_slice()).unwrap();
    }

    pub fn bind<I: Proxy, D>(
        &self,
        context: &mut Context<D>,
        global: &pw_registry::events::Global,
    ) -> I {
        let new_id = context.new_object(global.interface.clone()).protocol_id();

        self.send(
            context,
            pw_registry::methods::Bind {
                id: global.id,
                interface: global.interface.as_interface_name().to_string(),
                version: global.version,
                new_id,
            },
        );

        I::from_id(ObjectId::new(new_id))
    }

    pub fn destroy_global<D>(&self, context: &mut Context<D>, global: u32) {
        self.send(context, pw_registry::methods::Destroy { id: global });
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

    pub fn send<D>(&self, context: &mut Context<D>, message: impl MethodSerialize) {
        let (msg, fds) = protocol::create_msg_with_fds(self.object_id.object_id, &message);
        context.send_msg(&msg, fds.as_slice()).unwrap();
    }

    pub fn enum_param<D>(&self, context: &mut Context<D>, id: SpaParamType) {
        self.send(
            context,
            pw_device::methods::EnumParams {
                seq: 0,
                id: Id(id as u32),
                index: 0,
                num: 0,
                filter: pod::Builder::with(|b| {
                    b.write_none();
                }),
            },
        );
    }

    pub fn set_param<D>(&self, context: &mut Context<D>, param: pod::serialize::OwnedPod) {
        let (obj, _) = pod::PodDeserializer::new(&param.0);
        let id = obj.as_object().unwrap().object_id();

        self.send(
            context,
            pw_device::methods::SetParam {
                id: Id(id),
                flags: 0,
                param,
            },
        );
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

    pub fn send<D>(&self, context: &mut Context<D>, message: impl MethodSerialize) {
        let (msg, fds) = protocol::create_msg_with_fds(self.object_id.object_id, &message);
        context.send_msg(&msg, fds.as_slice()).unwrap();
    }

    pub fn enum_param<D>(&self, context: &mut Context<D>, id: SpaParamType) {
        self.send(
            context,
            pw_node::methods::EnumParams {
                seq: 0,
                id: Id(id as u32),
                index: 0,
                num: 0,
                filter: pod::Builder::with(|b| {
                    b.write_none();
                }),
            },
        );
    }

    pub fn set_param<D>(&self, context: &mut Context<D>, param: pod::serialize::OwnedPod) {
        let (obj, _) = pod::PodDeserializer::new(&param.0);
        let id = obj.as_object().unwrap().object_id();

        self.send(
            context,
            pw_node::methods::SetParam {
                id: Id(id),
                flags: 0,
                param,
            },
        );
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

    pub fn send<D>(&self, context: &mut Context<D>, message: impl MethodSerialize) {
        let (msg, fds) = protocol::create_msg_with_fds(self.object_id.object_id, &message);
        context.send_msg(&msg, fds.as_slice()).unwrap();
    }

    pub fn port_buffers<D>(
        &self,
        context: &mut Context<D>,
        direction: SpaDirection,
        port_id: u32,
        mix_id: u32,
    ) {
        self.send(
            context,
            pw_client_node::methods::PortBuffers {
                direction: SpaEnum::Value(direction),
                port_id,
                mix_id,
                buffers: vec![],
            },
        );
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

    pub fn send<D>(&self, context: &mut Context<D>, message: impl MethodSerialize) {
        let (msg, fds) = protocol::create_msg_with_fds(self.object_id.object_id, &message);
        context.send_msg(&msg, fds.as_slice()).unwrap();
    }

    pub fn enum_params<D>(&self, context: &mut Context<D>, id: SpaParamType) {
        self.send(
            context,
            pw_device::methods::EnumParams {
                seq: 0,
                id: Id(id as u32),
                index: 0,
                num: 0,
                filter: pod::Builder::with(|b| {
                    b.write_none();
                }),
            },
        );
    }
}
