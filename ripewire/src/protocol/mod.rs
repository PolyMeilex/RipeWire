#![allow(unused_variables)]

use crate::object_map::ObjectType;
use libspa_consts::{SpaDataType, SpaEnum, SpaIoType, SpaMetaType, SpaParamType};
use pod::{
    deserialize::{OwnedPod, PodStructDeserializer},
    Fd, Id, PodDeserializer,
};
use std::{
    collections::HashMap,
    io::{Seek, Write},
    os::fd::RawFd,
};

pub trait MethodSerialize: Sized {
    const OPCODE: u8;
    fn serialize(&self, buf: impl Write + Seek, fds: &mut Vec<RawFd>);
}

trait MethodSerializeSimple: Sized {
    const OPCODE: u8;
    fn serialize_simple(&self, buf: impl Write + Seek);
}

impl<T: MethodSerializeSimple> MethodSerialize for T {
    const OPCODE: u8 = T::OPCODE;
    fn serialize(&self, buf: impl Write + Seek, _fds: &mut Vec<RawFd>) {
        T::serialize_simple(self, buf);
    }
}

pub trait HasOpCode {
    const OPCODE: u8;
}

#[derive(Debug, thiserror::Error)]
#[error("{interface}.{event}: {error}")]
pub struct EventDeserializeError {
    pub interface: &'static str,
    pub event: &'static str,
    pub error: pod::DeserializeError,
}

pub trait HasInterface {
    const INTERFACE: &'static str;
}

trait EventDeserialize: Sized {
    const OPCODE: u8;
    fn deserialize(
        deserializer: &mut PodDeserializer,
        fds: &[RawFd],
    ) -> pod::deserialize::Result<Self>;
}

impl<T: EventDeserialize> Deserialize for T {
    fn deserialize(
        deserializer: &mut PodDeserializer,
        fds: &[RawFd],
    ) -> pod::deserialize::Result<Self> {
        <T as EventDeserialize>::deserialize(deserializer, fds)
    }
}

pub trait Deserialize: Sized {
    fn deserialize(
        deserializer: &mut PodDeserializer,
        fds: &[RawFd],
    ) -> pod::deserialize::Result<Self>;
}

fn parse_dict(
    pod: &mut PodStructDeserializer,
) -> pod::deserialize::Result<HashMap<String, String>> {
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

fn parse_params(pod: &mut PodStructDeserializer) -> pod::deserialize::Result<Vec<ParamInfo>> {
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
    MSG: MethodSerialize,
{
    create_msg_with_fds(object_id, value).0
}

pub fn create_msg_with_fds<MSG>(object_id: u32, value: &MSG) -> (Vec<u8>, Vec<RawFd>)
where
    MSG: MethodSerialize,
{
    let mut fds = vec![];
    let mut buff = std::io::Cursor::new(vec![]);
    value.serialize(&mut buff, &mut fds);

    let mut pod = buff.into_inner();

    let header = crate::connection::Header {
        object_id,
        opcode: MSG::OPCODE,
        len: pod.len() as u32,
        seq: 0,
        n_fds: fds.len() as u32,
    };

    let mut msg = header.serialize().to_vec();
    msg.append(&mut pod);

    (msg, fds)
}
