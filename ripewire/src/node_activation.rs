use std::{
    io,
    os::fd::{AsFd, BorrowedFd, OwnedFd},
    ptr::{addr_of, addr_of_mut, NonNull},
    sync::atomic::{AtomicI32, AtomicU32, Ordering},
    time::Duration,
};

use libspa_consts::{
    abi_unstable::{PwNodeActivation, PwNodeActivationStatus},
    SpaEnum, SpaStatus,
};

use crate::memory_registry::MemMap;

/// Current time
pub fn monotonic_ns() -> Duration {
    let ts = rustix::time::clock_gettime(rustix::time::ClockId::Monotonic);
    Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32)
}

/// Read and reset the counter of eventfd
pub fn eventfd_read(fd: BorrowedFd) -> io::Result<u64> {
    let mut count = [0; size_of::<u64>()];
    let read = rustix::io::read(fd, &mut count)?;

    if read != count.len() {
        return Err(io::ErrorKind::UnexpectedEof.into());
    }

    Ok(u64::from_ne_bytes(count))
}

/// Add `count` to the counter of eventfd
pub fn eventfd_write(fd: BorrowedFd, count: u64) -> io::Result<()> {
    let count = count.to_ne_bytes();
    let written = rustix::io::write(fd, &count)?;

    if written != count.len() {
        return Err(io::ErrorKind::WriteZero.into());
    }

    Ok(())
}

#[derive(Debug)]
pub struct Activation {
    ptr: NonNull<PwNodeActivation>,
    _map: MemMap,
}

impl Activation {
    /// `None` if the mapped memory is too small
    pub fn new(map: MemMap) -> Option<Self> {
        Some(Self {
            ptr: map.ptr_at(0)?,
            _map: map,
        })
    }

    fn status_atomic(&self) -> &AtomicU32 {
        unsafe { AtomicU32::from_ptr(addr_of_mut!((*self.ptr.as_ptr()).status)) }
    }

    fn pending_atomic(&self) -> &AtomicI32 {
        unsafe { AtomicI32::from_ptr(addr_of_mut!((*self.ptr.as_ptr()).state[0].pending)) }
    }

    pub fn status(&self) -> SpaEnum<PwNodeActivationStatus> {
        SpaEnum::from_raw(self.status_atomic().load(Ordering::SeqCst))
    }

    pub fn set_status(&self, status: PwNodeActivationStatus) {
        self.status_atomic().store(status as u32, Ordering::SeqCst);
    }

    /// Returns previous status
    pub fn swap_status(&self, status: PwNodeActivationStatus) -> SpaEnum<PwNodeActivationStatus> {
        SpaEnum::from_raw(self.status_atomic().swap(status as u32, Ordering::SeqCst))
    }

    /// Returns `true` on success
    pub fn compare_exchange_status(
        &self,
        current: PwNodeActivationStatus,
        new: PwNodeActivationStatus,
    ) -> bool {
        self.status_atomic()
            .compare_exchange(
                current as u32,
                new as u32,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .is_ok()
    }

    pub fn client_version(&self) -> u32 {
        unsafe { addr_of!((*self.ptr.as_ptr()).client_version).read_volatile() }
    }

    pub fn set_client_version(&self, version: u32) {
        unsafe { addr_of_mut!((*self.ptr.as_ptr()).client_version).write_volatile(version) }
    }

    pub fn server_version(&self) -> u32 {
        unsafe { addr_of!((*self.ptr.as_ptr()).server_version).read_volatile() }
    }

    pub fn set_signal_time(&self, nsec: Duration) {
        let nsec = nsec.as_nanos() as u64;
        unsafe { addr_of_mut!((*self.ptr.as_ptr()).signal_time).write_volatile(nsec) }
    }

    pub fn set_awake_time(&self, nsec: Duration) {
        let nsec = nsec.as_nanos() as u64;
        unsafe { addr_of_mut!((*self.ptr.as_ptr()).awake_time).write_volatile(nsec) }
    }

    pub fn set_finish_time(&self, nsec: Duration) {
        let nsec = nsec.as_nanos() as u64;
        unsafe { addr_of_mut!((*self.ptr.as_ptr()).finish_time).write_volatile(nsec) }
    }

    /// Set result of the process
    pub fn set_process_status(&self, status: SpaStatus) {
        unsafe { addr_of_mut!((*self.ptr.as_ptr()).state[0].status).write_volatile(status.bits()) }
    }

    /// Tell the driver that we are ready to be scheduled by it, has to be done each time the
    /// position io of the node changes, with the clock id of the new position.
    pub fn set_active_driver_id(&self, id: u32) {
        unsafe { addr_of_mut!((*self.ptr.as_ptr()).active_driver_id).write_volatile(id) }
    }

    /// Decrement number of dependencies that the node is still waiting for, and if there are no
    /// more of those wake it up by writing to its eventfd.
    ///
    /// Returns `true` if the node got woken up.
    fn trigger(&self, signalfd: BorrowedFd, nsec: Duration) -> io::Result<bool> {
        let pending = self.pending_atomic().fetch_sub(1, Ordering::SeqCst) - 1;
        if pending != 0 {
            return Ok(false);
        }

        if self.server_version() < 1 {
            self.set_status(PwNodeActivationStatus::Triggered);
        } else if !self.compare_exchange_status(
            PwNodeActivationStatus::NotTriggered,
            PwNodeActivationStatus::Triggered,
        ) {
            // Node is not ready to be scheduled
            return Ok(false);
        }

        self.set_signal_time(nsec);
        eventfd_write(signalfd, 1)?;

        Ok(true)
    }
}

/// Node that has to be triggered when we are done with processing
#[derive(Debug)]
pub struct Target {
    node_id: u32,
    activation: Activation,
    signalfd: OwnedFd,
}

impl Target {
    pub fn new(node_id: u32, activation: Activation, signalfd: OwnedFd) -> Self {
        Self {
            node_id,
            activation,
            signalfd,
        }
    }

    pub fn node_id(&self) -> u32 {
        self.node_id
    }

    pub fn activation(&self) -> &Activation {
        &self.activation
    }

    /// Signal that we are done with processing, returns `true` if the target got woken up.
    pub fn trigger(&self, nsec: Duration) -> io::Result<bool> {
        self.activation.trigger(self.signalfd.as_fd(), nsec)
    }
}
