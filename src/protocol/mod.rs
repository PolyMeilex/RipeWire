#[allow(unused)]
mod generated;
use generated::HasOpCode;

use libspa_consts::SpaParamType;
use pod::{
    serialize::{PodSerialize, PodSerializer},
    Value,
};
use std::io::Cursor;

//
// Core (object_id = 0)
//
pub mod pw_core {
    use super::*;

    pub const OBJECT_ID: u32 = 0;

    pub mod methods {
        use super::*;
        pub use generated::pw_core::methods::{
            CreateObject, Destroy, Error, GetRegistry, Hello, Pong, Sync,
        };

        impl Hello {
            pub fn to_msg(&self) -> Vec<u8> {
                super::create_msg(0, self)
            }
        }

        impl Sync {
            pub fn to_msg(&self) -> Vec<u8> {
                super::create_msg(0, self)
            }
        }

        impl Pong {
            pub fn to_msg(&self) -> Vec<u8> {
                super::create_msg(0, self)
            }
        }

        impl Error {
            pub fn to_msg(&self) -> Vec<u8> {
                super::create_msg(0, self)
            }
        }

        impl GetRegistry {
            pub fn to_msg(&self) -> Vec<u8> {
                super::create_msg(0, self)
            }
        }

        impl CreateObject {
            pub fn to_msg(&self) -> Vec<u8> {
                super::create_msg(0, self)
            }
        }

        impl Destroy {
            pub fn to_msg(&self) -> Vec<u8> {
                super::create_msg(0, self)
            }
        }
    }

    pub use generated::pw_core::Event;
    pub mod event {
        use super::*;
        pub use generated::pw_core::events::*;
    }
}

//
// Client (object_id = 1)
//
pub mod pw_client {
    use super::*;

    pub const OBJECT_ID: u32 = 1;

    pub mod methods {
        use super::*;
        pub use generated::pw_client::methods::{Error, UpdateProperties};

        impl Error {
            pub fn to_msg(&self) -> Vec<u8> {
                super::create_msg(1, self)
            }
        }

        impl UpdateProperties {
            pub fn to_msg(&self) -> Vec<u8> {
                super::create_msg(1, self)
            }
        }
    }

    pub use event::Event;
    pub mod event {
        use super::*;
        use generated::pw_client::events::Info;

        #[derive(Debug, Clone, pod_derive::EventDeserialize)]
        pub enum Event {
            Info(Info),
            // Info(Value),
            Permissions(Value),
        }
    }
}

pub mod pw_device {
    use super::*;

    pub mod methods {
        use super::*;
        pub use generated::pw_device::methods::SetParam;

        pub fn set_param(object_id: u32, param: SpaParamType, value: &Value) -> Vec<u8> {
            let value = Value::Struct(vec![
                Value::Id(pod::utils::Id(param as u32)),
                Value::Int(0),
                value.clone(),
            ]);

            super::manual_create_msg(
                object_id,
                generated::pw_device::methods::SetParam::OPCODE,
                &value,
            )
        }
    }

    pub use event::Event;
    pub mod event {
        use super::*;

        #[derive(Debug, Clone, pod_derive::EventDeserialize)]
        pub enum Event {
            Info(Value),
            Param(Value),
        }
    }
}

pub mod pw_node {
    use super::*;

    pub mod methods {}

    pub use event::Event;
    pub mod event {
        use super::*;

        #[derive(Debug, Clone, pod_derive::EventDeserialize)]
        pub enum Event {
            Info(Value),
            Param(Value),
        }
    }
}

pub mod pw_client_node {
    use super::*;

    pub mod methods {
        use super::*;

        pub use generated::pw_client_node::methods::*;
    }

    pub use event::Event;
    pub mod event {
        use super::*;

        #[derive(Debug, Clone)]
        pub struct PortUpdateInfo {
            pub change_mask: u64,
            pub flags: u64,
            pub rate_num: u32,
            pub rate_denom: u32,
            pub items: Vec<(String, String)>,
            pub params: Vec<(pod::utils::Id, u32)>,
        }

        impl pod::serialize::PodSerialize for PortUpdateInfo {
            fn serialize<O: std::io::Write + std::io::Seek>(
                &self,
                serializer: pod::serialize::PodSerializer<O>,
            ) -> Result<pod::serialize::SerializeSuccess<O>, pod::serialize::GenError> {
                let mut s = serializer.serialize_struct()?;

                s.serialize_field(&self.change_mask)?;
                s.serialize_field(&self.flags)?;
                s.serialize_field(&self.rate_num)?;
                s.serialize_field(&self.rate_denom)?;

                s.serialize_field(&(self.items.len() as i32))?;

                for (key, value) in self.items.iter() {
                    s.serialize_field(key)?;
                    s.serialize_field(value)?;
                }

                s.serialize_field(&(self.params.len() as i32))?;

                for (id, flags) in self.params.iter() {
                    s.serialize_field(id)?;
                    s.serialize_field(flags)?;
                }

                s.end()
            }
        }

        #[derive(Debug, Clone)]
        pub struct PortUpdate {
            pub direction: i32,
            pub port_id: i32,
            pub change_mask: i32,
            pub params: Vec<pod::Value>,
            pub info: Option<PortUpdateInfo>,
        }

        impl pod::serialize::PodSerialize for PortUpdate {
            fn serialize<O: std::io::Write + std::io::Seek>(
                &self,
                serializer: pod::serialize::PodSerializer<O>,
            ) -> Result<pod::serialize::SerializeSuccess<O>, pod::serialize::GenError> {
                let mut s = serializer.serialize_struct()?;
                s.serialize_field(&self.direction)?;
                s.serialize_field(&self.port_id)?;
                s.serialize_field(&self.change_mask)?;

                s.serialize_field(&(self.params.len() as i32))?;

                for param in self.params.iter() {
                    s.serialize_field(param)?;
                }

                if let Some(info) = self.info.as_ref() {
                    s.serialize_field(info)?;
                } else {
                    s.serialize_field(&Value::None)?;
                }

                s.end()
            }
        }

        impl HasOpCode for PortUpdate {
            const OPCODE: u8 = 3;
        }

        use generated::pw_client_node::events::{
            Command, PortSetIo, PortSetParam, SetIo, Transport,
        };

        #[derive(Debug, Clone, pod_derive::EventDeserialize)]
        pub enum Event {
            Transport(Transport),
            SetParam(Value),
            SetIo(SetIo),
            Event(Value),
            Command(Command),
            AddPort(Value),
            RemovePort(Value),
            PortSetParam(PortSetParam),
            PortUseBuffers(Value),
            PortSetIo(PortSetIo),
            SetActivation(Value),
            PortSetMixInfo(Value),
        }
    }
}

//
// Registry (object_id = dynamic)
//
pub mod pw_registry {
    use super::*;

    pub mod methods {
        use super::*;

        #[derive(Debug, Clone, pod_derive::PodSerialize)]
        pub struct Bind {
            pub id: u32,
            pub obj_type: String,
            pub version: u32,
            pub new_id: u32,
        }

        impl HasOpCode for Bind {
            const OPCODE: u8 = 1;
        }

        impl Bind {
            pub fn to_msg(&self, object_id: u32) -> Vec<u8> {
                super::create_msg(object_id, self)
            }
        }

        #[derive(Debug, Clone, pod_derive::PodSerialize)]
        pub struct Destroy {
            pub id: u32,
        }

        impl HasOpCode for Destroy {
            const OPCODE: u8 = 2;
        }

        impl Destroy {
            pub fn to_msg(&self, object_id: u32) -> Vec<u8> {
                super::create_msg(object_id, self)
            }
        }
    }

    pub use generated::pw_registry::Event;
    pub mod event {
        use super::*;
        pub use generated::pw_registry::events::*;
    }
}

pub fn create_msg<MSG>(object_id: u32, value: &MSG) -> Vec<u8>
where
    MSG: PodSerialize + HasOpCode,
{
    manual_create_msg(object_id, MSG::OPCODE, value)
}

pub fn manual_create_msg<MSG>(object_id: u32, opcode: u8, value: &MSG) -> Vec<u8>
where
    MSG: PodSerialize,
{
    let (pod, _size) = PodSerializer::serialize(Cursor::new(Vec::new()), value).unwrap();
    let mut pod = pod.into_inner();

    let header = crate::connection::Header {
        object_id,
        opcode,
        len: pod.len() as u32,
        seq: 0,
        n_fds: 0,
    };

    let mut msg = header.serialize().to_vec();
    msg.append(&mut pod);

    msg
}
