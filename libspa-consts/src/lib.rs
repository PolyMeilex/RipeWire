use bitflags::bitflags;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
mod bindings {
    include!("./gen/bindings.rs");
}

pub use bindings::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Fraction {
    pub num: u32,
    pub denom: u32,
}

#[derive(Debug, Clone, Copy)]
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

    /* vendor extensions */
    VendorPipeWire = 0x02000000,

    VendorOther = 0x7f000000,
}

/// different parameter types that can be queried
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum SpaParamType {
    /**< invalid */
    Invalid,
    /**< property information as SPA_TYPE_OBJECT_PropInfo */
    PropInfo,
    /**< properties as SPA_TYPE_OBJECT_Props */
    Props,
    /**< available formats as SPA_TYPE_OBJECT_Format */
    EnumFormat,
    /**< configured format as SPA_TYPE_OBJECT_Format */
    Format,
    /**< buffer configurations as SPA_TYPE_OBJECT_ParamBuffers*/
    Buffers,
    /**< allowed metadata for buffers as SPA_TYPE_OBJECT_ParamMeta*/
    Meta,
    /**< configurable IO areas as SPA_TYPE_OBJECT_ParamIO */
    IO,
    /**< profile enumeration as SPA_TYPE_OBJECT_ParamProfile */
    EnumProfile,
    /**< profile configuration as SPA_TYPE_OBJECT_ParamProfile */
    Profile,
    /**< port configuration enumeration as SPA_TYPE_OBJECT_ParamPortConfig */
    EnumPortConfig,
    /**< port configuration as SPA_TYPE_OBJECT_ParamPortConfig */
    PortConfig,
    /**< routing enumeration as SPA_TYPE_OBJECT_ParamRoute */
    EnumRoute,
    /**< routing configuration as SPA_TYPE_OBJECT_ParamRoute */
    Route,
    /**< Control parameter, a SPA_TYPE_Sequence */
    Control,
    /**< latency reporting, a SPA_TYPE_OBJECT_ParamLatency */
    Latency,
    /**< processing latency, a SPA_TYPE_OBJECT_ParamProcessLatency */
    ProcessLatency,
}

/// properties for SPA_TYPE_OBJECT_ParamRoute
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum SpaParamRoute {
    /**< index of the routing destination (Int) */
    Index = 1,
    /**< direction, input/output (Id enum spa_direction) */
    Direction,
    /**< device id (Int) */
    Device,
    /**< name of the routing destination (String) */
    Name,
    /**< description of the destination (String) */
    Description,
    /**< priority of the destination (Int) */
    Priority,
    /// < availability of the destination
    Available,
    /**< info (Struct(
     *  (Id enum spa_param_availability)
     *  Int : n_items,
     *  (String : key,
     *  String : value)*)) */
    Info,
    /**< associated profile indexes (Array of Int) */
    Profiles,
    /**< properties SPA_TYPE_OBJECT_Props */
    Props,
    /**< associated device indexes (Array of Int) */
    Devices,
    /**< profile id (Int) */
    Profile,
    /**< If route should be saved (Bool) */
    Save,
}

/// predefined properties for SPA_TYPE_OBJECT_Props
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SpaProp {
    // START = 0,
    Unknown = 1,

    // START_Device = 0x100,
    Device = 0x100 + 1,
    DeviceName,
    DeviceFd,
    Card,
    CardName,
    MinLatency,
    MaxLatency,
    Periods,
    PeriodSize,
    PeriodEvent,
    Live,
    Rate,
    Quality,
    BluetoothAudioCodec,

    // START_Audio = 0x10000,
    WaveType = 0x10000 + 1,
    Frequency,
    Volume,
    Mute,
    PatternType,
    DitherType,
    Truncate,
    ChannelVolumes,
    VolumeBase,
    VolumeStep,
    ChannelMap,
    MonitorMute,
    MonitorVolumes,
    LatencyOffsetNsec,
    SoftMute,
    SoftVolumes,
    Iec958Codecs,

    // START_Video = 0x20000,
    Brightness = 0x20000 + 1,
    Contrast,
    Saturation,
    Hue,
    Gamma,
    Exposure,
    Gain,
    Sharpness,

    // START_Other = 0x80000,
    Params = 0x80000 + 1,
    // START_CUSTOM = 0x1000000,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SpaDataType {
    Invalid,
    /**< pointer to memory, the data field in
     * *  struct spa_data is set. */
    MemPtr,
    /**< generic fd, mmap to get to memory */
    MemFd,
    /**< fd to dmabuf memory */
    DmaBuf,
    /**< memory is identified with an id */
    MemId,
}

impl SpaDataType {
    pub fn from_raw(v: u32) -> Option<Self> {
        let v = match v {
            0 => Self::Invalid,
            1 => Self::MemPtr,
            2 => Self::MemFd,
            3 => Self::DmaBuf,
            4 => Self::MemId,
            _ => return None,
        };

        Some(v)
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
