use bitflags::bitflags;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
mod bindings {
    include!("./gen/bindings.rs");
}

pub use bindings::*;
pub use num_traits::{FromPrimitive, ToPrimitive};

/// Wrapper type for enums that handles unknown variants gracefully
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SpaEnum<T, RAW = u32> {
    Value(T),
    Unknown(RAW),
}

impl<T: std::fmt::Debug, RAW: std::fmt::Debug> std::fmt::Debug for SpaEnum<T, RAW> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaEnum::Value(v) => v.fmt(f),
            SpaEnum::Unknown(v) => f.debug_tuple("Unknown").field(v).finish(),
        }
    }
}

impl<T, RAW: std::fmt::Debug> SpaEnum<T, RAW> {
    pub fn ok(self) -> Option<T> {
        match self {
            SpaEnum::Value(v) => Some(v),
            SpaEnum::Unknown(_) => None,
        }
    }

    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            SpaEnum::Value(v) => v,
            SpaEnum::Unknown(v) => {
                panic!("called `SpaEnum::unwrap()` on a `Unknown({:?})` value", v)
            }
        }
    }
}

impl<T, RAW> From<T> for SpaEnum<T, RAW> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}

impl<T: num_traits::FromPrimitive> SpaEnum<T, u32> {
    pub fn from_raw(raw: u32) -> Self {
        T::from_u32(raw)
            .map(Self::Value)
            .unwrap_or(Self::Unknown(raw))
    }
}

impl<T: num_traits::FromPrimitive> SpaEnum<T, i32> {
    pub fn from_i32(raw: i32) -> Self {
        T::from_i32(raw)
            .map(Self::Value)
            .unwrap_or(Self::Unknown(raw))
    }
}

impl<T> SpaEnum<T, u32>
where
    T: num_traits::cast::ToPrimitive + Clone,
{
    pub fn as_raw(&self) -> u32 {
        match self {
            SpaEnum::Value(v) => v.to_u32().unwrap(),
            SpaEnum::Unknown(v) => *v,
        }
    }
}

impl<T> SpaEnum<T, i32>
where
    T: num_traits::cast::ToPrimitive + Clone,
{
    pub fn as_raw(&self) -> i32 {
        match self {
            SpaEnum::Value(v) => v.to_i32().unwrap(),
            SpaEnum::Unknown(v) => *v,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, num_derive::FromPrimitive, num_derive::ToPrimitive)]
#[repr(u32)]
pub enum SpaType {
    /* Basic types 0x00000 */
    None = 1,
    Bool,
    Id,
    Int,
    Long,
    Float,
    Double,
    String,
    Bytes,
    Rectangle,
    Fraction,
    Bitmap,
    Array,
    Struct,
    Object,
    Sequence,
    Pointer,
    Fd,
    Choice,
    Pod,

    /* Pointers 0x10000 */
    PointerBuffer = 0x10000 + 1,
    PointerMeta,
    PointerDict,

    /* Events 0x20000 */
    EventDevice = 0x20000 + 1,
    EventNode,

    /* Commands 0x30000 */
    CommandDevice = 0x30000 + 1,
    CommandNode,

    /* Objects 0x40000 */
    ObjectPropInfo = 0x40000 + 1,
    ObjectProps,
    ObjectFormat,
    ObjectParamBuffers,
    ObjectParamMeta,
    ObjectParamIo,
    ObjectParamProfile,
    ObjectParamPortConfig,
    ObjectParamRoute,
    ObjectProfiler,
    ObjectParamLatency,
    ObjectParamProcessLatency,
    ObjectParamParamTag,

    /* vendor extensions */
    VendorPipeWire = 0x02000000,

    VendorOther = 0x7f000000,
}

impl SpaType {
    pub fn from_raw(v: u32) -> Option<Self> {
        num_traits::FromPrimitive::from_u32(v)
    }
}

impl SpaFormat {
    pub fn from_raw(v: u32) -> Option<Self> {
        num_traits::FromPrimitive::from_u32(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, num_derive::FromPrimitive)]
#[repr(u32)]
pub enum SpaPointerType {
    Buffer = SpaType::PointerBuffer as u32,
    Meta = SpaType::PointerMeta as u32,
    Dict = SpaType::PointerDict as u32,
}

impl SpaPointerType {
    pub fn from_raw(v: u32) -> Option<Self> {
        num_traits::FromPrimitive::from_u32(v)
    }
}

impl SpaDataType {
    pub fn from_raw(v: u32) -> Option<Self> {
        num_traits::FromPrimitive::from_u32(v)
    }
}

impl SpaChoiceType {
    pub fn from_raw(v: u32) -> Option<Self> {
        num_traits::FromPrimitive::from_u32(v)
    }
}

impl SpaParamType {
    pub fn from_raw(v: u32) -> Option<Self> {
        num_traits::FromPrimitive::from_u32(v)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PwMemblockFlags: u32 {
        /**< memory is readable */
        const READABLE = 1 << 0;
        /**< memory is writable */
        const WRITABLE = 1 << 1;
        /**< seal the fd */
        const SEAL = 1 << 2;
        /**< mmap the fd */
        const MAP = 1 << 3;
        /**< don't close fd */
        const DONT_CLOSE = 1 << 4;
        /**< don't notify events */
        const DONT_NOTIFY = 1 << 5;

        const READWRITE = Self::READABLE.bits() | Self::WRITABLE.bits();
    }
}

bitflags! {
    /// Property flags
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct SpaPropFlags: u32 {
        /// Property is read-only.
        const READONLY = SPA_POD_PROP_FLAG_READONLY;
        /// Property is some sort of hardware parameter.
        const HARDWARE = SPA_POD_PROP_FLAG_HARDWARE;
        /// Property contains a dictionary struct.
        const HINT_DICT = SPA_POD_PROP_FLAG_HINT_DICT;
        /// Property is mandatory, when filtering, both sides need this property or filtering
        /// fails.
        const MANDATORY = SPA_POD_PROP_FLAG_MANDATORY;
        /// Property choices need no fixation.
        const DONT_FIXATE = SPA_POD_PROP_FLAG_DONT_FIXATE;
        /// Drop property, when filtering, both sides need the property or it will be dropped.
        const DROP = SPA_POD_PROP_FLAG_DROP;
    }
}

bitflags! {
    /// Result of processing a node or status of an IO area.
    ///
    /// Fields that store it can also contain a negative errno value, so those stay `i32`,
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SpaStatus: i32 {
        /// Same as [`SpaStatus::empty`], as it has no bits set
        const OK = SPA_STATUS_OK;
        const NEED_DATA = SPA_STATUS_NEED_DATA;
        const HAVE_DATA = SPA_STATUS_HAVE_DATA;
        const STOPPED = SPA_STATUS_STOPPED;
        const DRAINED = SPA_STATUS_DRAINED;
    }
}

bitflags! {
    /// Flags to pass to the use_buffers functions
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SpaNodeBuffersFlags: u32 {
        /// Allocate memory for the buffers. This flag is ignored when the port does not have the
        /// `SPA_PORT_FLAG_CAN_ALLOC_BUFFERS` set.
        const ALLOC = SPA_NODE_BUFFERS_FLAG_ALLOC;
    }
}

/// Well this is private API/ABI, I'm not sure how libpipewire makes sure this does not blow up acrros ABI braking updates
pub mod abi_unstable {
    use super::*;

    /// Versions:
    /// - 0 baseline
    /// - 1 the activation status needs to be CAS
    pub const PW_VERSION_NODE_ACTIVATION: u32 = 1;

    /// Value of [`PwNodeActivation::status`], see its docs for the possible transitions
    #[repr(u32)]
    #[derive(
        Debug, Clone, Copy, PartialEq, Eq, num_derive::FromPrimitive, num_derive::ToPrimitive,
    )]
    pub enum PwNodeActivationStatus {
        NotTriggered = 0,
        Triggered = 1,
        Awake = 2,
        Finished = 3,
        Inactive = 4,
    }

    impl PwNodeActivationStatus {
        /// Node was prepared by the driver but has not triggered its peers yet
        pub fn is_pending_trigger(self) -> bool {
            matches!(self, Self::NotTriggered | Self::Triggered | Self::Awake)
        }
    }

    bitflags! {
        /// Value of [`PwNodeActivation::flags`]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct PwNodeActivationFlags: u32 {
            /// The profiler is running
            const PROFILER = 1 << 0;
            /// The node is async
            const ASYNC = 1 << 1;
        }
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct PwNodeActivationState {
        /// Current status, the result of spa_node_process()
        pub status: std::ffi::c_int,
        /// Required number of signals
        pub required: i32,
        /// Number of pending signals
        pub pending: i32,
    }

    /// Nodes start as INACTIVE, when they are ready to be scheduled, they add their
    /// fd to the loop and change status to FINISHED. When the node shuts down, the
    /// status is set back to INACTIVE.
    ///
    /// We have status changes (using compare-and-swap) from
    ///
    /// - INACTIVE -> FINISHED (node is added to loop and can be scheduled)
    /// - * -> INACTIVE (node can not be scheduled anymore)
    ///
    /// - !INACTIVE -> NOT_TRIGGERED (node is prepared by the driver)
    /// - NOT_TRIGGERED -> TRIGGERED (eventfd is written)
    /// - TRIGGERED -> AWAKE (eventfd is read, node starts processing)
    /// - AWAKE -> FINISHED (node completed processing and triggered the peers)
    #[repr(C)]
    #[derive(Debug)]
    pub struct PwNodeActivation {
        pub status: u32,

        /// Bitfield: version:1, pending_sync:1, pending_new_pos:1
        pub bits: std::ffi::c_uint,

        /// One current state and one next state, as version flag
        pub state: [PwNodeActivationState; 2],

        /// Time at which the node was triggered
        pub signal_time: u64,
        /// Time at which processing actually started
        pub awake_time: u64,
        /// Time at which processing was completed
        pub finish_time: u64,
        /// Previous time at which the node was triggered
        pub prev_signal_time: u64,

        pub reposition: SpaIoSegment,
        pub segment: SpaIoSegment,

        pub segment_owner: [u32; 16],
        pub prev_awake_time: u64,
        pub prev_finish_time: u64,
        /// Must be 0
        pub padding: [u32; 7],

        /// Version of client, see [`PW_VERSION_NODE_ACTIVATION`]
        pub client_version: u32,
        /// Version of server, see [`PW_VERSION_NODE_ACTIVATION`]
        pub server_version: u32,

        /// Driver active on client
        pub active_driver_id: u32,
        /// The current node driver id
        pub driver_id: u32,
        pub flags: u32,

        pub position: SpaIoPosition,

        pub sync_timeout: u64,
        pub sync_left: u64,

        pub cpu_load: [f32; 3],
        pub xrun_count: u32,
        pub xrun_time: u64,
        pub xrun_delay: u64,
        pub max_delay: u64,

        pub command: u32,
        pub reposition_owner: u32,
    }
}
