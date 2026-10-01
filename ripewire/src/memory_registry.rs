use std::{
    collections::HashMap,
    io,
    os::fd::{FromRawFd, OwnedFd},
    ptr::NonNull,
};

use libspa_consts::{SpaDataType, SpaEnum};
use log::error;

use crate::protocol::pw_core;

pub type MemId = u32;

#[allow(unused)]
#[derive(Debug)]
pub struct Mem {
    id: u32,
    mem_type: SpaEnum<SpaDataType>,
    flags: pw_core::MemblockFlags,
    fd: OwnedFd,
}

impl Mem {
    pub fn fd(&self) -> &OwnedFd {
        &self.fd
    }
}

#[derive(Debug)]
pub struct MemMap {
    /// Start of the page aligned mapping
    map_ptr: NonNull<libc::c_void>,
    map_len: usize,
    /// Start of the requested region
    ptr: NonNull<u8>,
    len: usize,
}

impl MemMap {
    /// Offset does not have to be page aligned
    pub fn new(fd: &OwnedFd, offset: u32, size: u32) -> io::Result<Self> {
        let page_size = rustix::param::page_size();

        let offset = offset as usize;
        let len = size as usize;

        let map_offset = offset - offset % page_size;
        let map_len = len + (offset - map_offset);

        if map_len == 0 {
            return Err(io::ErrorKind::InvalidInput.into());
        }

        let map_ptr = unsafe {
            rustix::mm::mmap(
                std::ptr::null_mut(),
                map_len,
                rustix::mm::ProtFlags::READ | rustix::mm::ProtFlags::WRITE,
                rustix::mm::MapFlags::SHARED,
                fd,
                map_offset as u64,
            )?
        };

        let map_ptr = NonNull::new(map_ptr).unwrap();
        let ptr = unsafe { map_ptr.cast::<u8>().add(offset - map_offset) };

        Ok(Self {
            map_ptr,
            map_len,
            ptr,
            len,
        })
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// `None` if T does not fit
    pub fn ptr_at<T>(&self, offset: usize) -> Option<NonNull<T>> {
        let end = offset.checked_add(std::mem::size_of::<T>())?;
        if end > self.len {
            return None;
        }

        let ptr = unsafe { self.ptr.add(offset) }.cast::<T>();
        ptr.is_aligned().then_some(ptr)
    }
}

impl Drop for MemMap {
    fn drop(&mut self) {
        unsafe {
            rustix::mm::munmap(self.map_ptr.as_ptr(), self.map_len)
                .inspect_err(|err| error!("{err}"))
                .ok();
        }
    }
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

    pub fn get(&self, id: &MemId) -> Option<&Mem> {
        self.map.get(id)
    }

    /// Call in response to `Core::AddMem`
    pub fn map(&self, id: MemId, offset: u32, size: u32) -> io::Result<MemMap> {
        let mem = self.get(&id).ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, format!("unknown memid {id}"))
        })?;
        MemMap::new(mem.fd(), offset, size)
    }

    pub fn remove_mem(&mut self, remove_mem: &pw_core::events::RemoveMem) {
        self.map.remove(&remove_mem.id);
    }
}
