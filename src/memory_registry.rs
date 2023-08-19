use std::{
    collections::HashMap,
    os::fd::{FromRawFd, OwnedFd, RawFd},
};

use libspa_consts::SpaDataType;

use crate::protocol::pw_core;

pub type MemId = u32;

#[derive(Debug)]
pub struct Mem {
    id: u32,
    mem_type: SpaDataType,
    flags: pw_core::MemblockFlags,
    fd: OwnedFd,
}

#[derive(Debug, Default)]
pub struct MemoryRegistry {
    map: HashMap<MemId, Mem>,
}

impl MemoryRegistry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add_mem(&mut self, add_mem: &pw_core::event::AddMem, fds: &[RawFd]) {
        let fd = fds[add_mem.fd.0 as usize];

        let mem_type = SpaDataType::from_raw(add_mem.ty.0).unwrap();

        self.map.insert(
            add_mem.id,
            Mem {
                id: add_mem.id,
                mem_type,
                flags: add_mem.flags,
                fd: unsafe { OwnedFd::from_raw_fd(fd) },
            },
        );
    }

    pub fn remove_mem(&mut self, remove_mem: &pw_core::event::RemoveMem) {
        self.map.remove(&remove_mem.id);
    }
}
