#![allow(unused)]

use libspa_consts::{SpaDataType, SpaEnum, SpaIoType, SpaMetaType, SpaParamType};
use pod::serialize::{PodSerialize, PodSerializer};
use pod_v2::{
    deserialize::{OwnedPod, PodStructDeserializer},
    PodDeserializer,
};
use std::{collections::HashMap, io::Cursor, os::fd::RawFd};

pub trait HasOpCode {
    const OPCODE: u8;
}

#[derive(Debug, thiserror::Error)]
#[error("{interface}.{event}: {error}")]
pub struct EventDeserializeError {
    pub interface: &'static str,
    pub event: &'static str,
    pub error: pod_v2::DeserializeError,
}

pub trait HasInterface {
    const INTERFACE: &'static str;
}

trait EventDeserialize: Sized {
    const OPCODE: u8;
    fn deserialize(
        deserializer: &mut PodDeserializer,
        fds: &[RawFd],
    ) -> pod_v2::deserialize::Result<Self>;
}

impl<T: EventDeserialize> Deserialize for T {
    fn deserialize(
        deserializer: &mut PodDeserializer,
        fds: &[RawFd],
    ) -> pod_v2::deserialize::Result<Self> {
        <T as EventDeserialize>::deserialize(deserializer, fds)
    }
}

pub trait Deserialize: Sized {
    fn deserialize(
        deserializer: &mut PodDeserializer,
        fds: &[RawFd],
    ) -> pod_v2::deserialize::Result<Self>;
}

macro_rules! parse {
    ($pod: expr, Self ( $(|$a: ident| $b: expr),* $(,)? )) => {{
        let mut pod = $pod.as_struct()?;
        Self {
            $(
                $a: {
                    pod.pop_field().and_then(|$a| {
                        Ok($b)
                    })?
                },
            )*
        }
    }};
}

fn parse_dict(
    pod: &mut PodStructDeserializer,
) -> pod_v2::deserialize::Result<HashMap<String, String>> {
    let count = pod.pop_field()?;
    let count = count.as_i32()?;

    let mut map = HashMap::new();
    for _ in 0..count {
        let key = pod.pop_field()?.as_str()?.to_string();
        let value = pod.pop_field()?.as_str()?.to_string();
        map.insert(key, value);
    }

    Ok(map)
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ParamFlags: u32 {
        /// bit to signal update even when the
        /// read/write flags don't change
        const SERIAL = 1 << 0;
        const READ = 1 << 1;
        const WRITE = 1 << 2;
        const READWRITE = Self::WRITE.bits() | Self::READ.bits();
    }
}

#[derive(Debug, Clone)]
pub struct ParamInfo {
    pub id: SpaEnum<SpaParamType>,
    pub flags: ParamFlags,
}

fn parse_params(pod: &mut PodStructDeserializer) -> pod_v2::deserialize::Result<Vec<ParamInfo>> {
    let len = pod.pop_field()?.as_i32()?;

    if len <= 0 {
        return Ok(Vec::new());
    }
    let len = len as u32;

    let mut params = Vec::with_capacity(len as usize);

    for _ in 0..len {
        let id = pod.pop_field()?.as_id()?;
        let flags = pod.pop_field()?.as_u32()?;
        params.push(ParamInfo {
            id: SpaEnum::from_raw(id),
            flags: ParamFlags::from_bits_retain(flags),
        });
    }

    Ok(params)
}

pub mod pw_client;
pub mod pw_client_node;
pub mod pw_core;
pub mod pw_device;
pub mod pw_factory;
pub mod pw_link;
pub mod pw_module;
pub mod pw_node;
pub mod pw_port;
pub mod pw_registry;

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
