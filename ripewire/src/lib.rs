pub mod connection;
pub mod context;
pub mod global_list;
pub mod memory_registry;
pub mod object_map;
pub mod protocol;
pub mod proxy;

pub mod reexports {
    pub use libc;
    pub use libspa_consts;
    pub use pod;
}

use std::os::fd::RawFd;

use protocol::PwDictionary;

pub fn set_blocking(fd: RawFd, blocking: bool) {
    // Save the current flags
    let mut flags = unsafe { libc::fcntl(fd, libc::F_GETFL, 0) };
    if flags == -1 {
        return;
    }

    if blocking {
        flags &= !libc::O_NONBLOCK;
    } else {
        flags |= libc::O_NONBLOCK;
    }

    let _ = unsafe { libc::fcntl(fd, libc::F_SETFL, flags) != -1 };
}

pub fn poll(fd: RawFd, timeout: i32) {
    let fd = libc::pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    };

    let mut fds = [fd];

    unsafe {
        libc::poll(fds.as_mut_ptr(), fds.len() as u64, timeout);
    }
}

pub trait HashMapExt {
    fn from_dict<'a>(v: impl IntoIterator<Item = (&'a str, &'a str)>) -> PwDictionary;
}

impl HashMapExt for PwDictionary {
    fn from_dict<'a>(v: impl IntoIterator<Item = (&'a str, &'a str)>) -> PwDictionary {
        let mut map = PwDictionary::new();
        for (key, value) in v {
            map.insert(key.into(), value.into());
        }
        map
    }
}
