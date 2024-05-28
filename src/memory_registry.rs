use std::{
    collections::HashMap,
    os::fd::{FromRawFd, OwnedFd},
};

use libspa_consts::{SpaDataType, SpaEnum};

use crate::protocol::pw_core;

pub type MemId = u32;

#[derive(Debug)]
pub struct Mem {
    id: u32,
    mem_type: SpaEnum<SpaDataType>,
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

    pub fn add_mem(&mut self, add_mem: &pw_core::events::AddMem) {
        let fd = add_mem.fd.fd.unwrap();

        let mem_type = add_mem.ty;

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

    pub fn remove_mem(&mut self, remove_mem: &pw_core::events::RemoveMem) {
        self.map.remove(&remove_mem.id);
    }
}
