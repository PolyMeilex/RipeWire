/* automatically generated by rust-bindgen 0.64.0 */

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SpaDirection {
    Input = 0,
    Output = 1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpaRectangle {
    pub width: u32,
    pub height: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpaFraction {
    pub num: u32,
    pub denom: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SpaPod {
    pub size: u32,
    pub type_: u32,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, num_derive :: FromPrimitive)]
pub enum SpaChoiceType {
    #[doc = "< no choice, first value is current"]
    None = 0,
    #[doc = "< range: default, min, max"]
    Range = 1,
    #[doc = "< range with step: default, min, max, step"]
    Step = 2,
    #[doc = "< list: default, alternative,..."]
    Enum = 3,
    #[doc = "< flags: default, possible flags,..."]
    Flags = 4,
}
#[repr(u32)]
#[doc = " \\addtogroup spa_buffer\n \\{"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, num_derive :: FromPrimitive)]
pub enum SpaDataType {
    Invalid = 0,
    #[doc = "< pointer to memory, the data field in\n  struct spa_data is set."]
    MemPtr = 1,
    #[doc = "< generic fd, mmap to get to memory"]
    MemFd = 2,
    #[doc = "< fd to dmabuf memory"]
    DmaBuf = 3,
    #[doc = "< memory is identified with an id"]
    MemId = 4,
}
#[repr(u32)]
#[doc = " Different IO area types"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SpaIoType {
    Invalid = 0,
    #[doc = "< area to exchange buffers, struct spa_io_buffers"]
    Buffers = 1,
    #[doc = "< expected byte range, struct spa_io_range"]
    Range = 2,
    #[doc = "< area to update clock information, struct spa_io_clock"]
    Clock = 3,
    #[doc = "< latency reporting, struct spa_io_latency"]
    Latency = 4,
    #[doc = "< area for control messages, struct spa_io_sequence"]
    Control = 5,
    #[doc = "< area for notify messages, struct spa_io_sequence"]
    Notify = 6,
    #[doc = "< position information in the graph, struct spa_io_position"]
    Position = 7,
    #[doc = "< rate matching between nodes, struct spa_io_rate_match"]
    RateMatch = 8,
    #[doc = "< memory pointer, struct spa_io_memory"]
    Memory = 9,
}
#[repr(u32)]
#[doc = " different parameter types that can be queried"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SpaParamType {
    #[doc = "< invalid"]
    Invalid = 0,
    #[doc = "< property information as SPA_TYPE_OBJECT_PropInfo"]
    PropInfo = 1,
    #[doc = "< properties as SPA_TYPE_OBJECT_Props"]
    Props = 2,
    #[doc = "< available formats as SPA_TYPE_OBJECT_Format"]
    EnumFormat = 3,
    #[doc = "< configured format as SPA_TYPE_OBJECT_Format"]
    Format = 4,
    #[doc = "< buffer configurations as SPA_TYPE_OBJECT_ParamBuffers"]
    Buffers = 5,
    #[doc = "< allowed metadata for buffers as SPA_TYPE_OBJECT_ParamMeta"]
    Meta = 6,
    #[doc = "< configurable IO areas as SPA_TYPE_OBJECT_ParamIO"]
    Io = 7,
    #[doc = "< profile enumeration as SPA_TYPE_OBJECT_ParamProfile"]
    EnumProfile = 8,
    #[doc = "< profile configuration as SPA_TYPE_OBJECT_ParamProfile"]
    Profile = 9,
    #[doc = "< port configuration enumeration as SPA_TYPE_OBJECT_ParamPortConfig"]
    EnumPortConfig = 10,
    #[doc = "< port configuration as SPA_TYPE_OBJECT_ParamPortConfig"]
    PortConfig = 11,
    #[doc = "< routing enumeration as SPA_TYPE_OBJECT_ParamRoute"]
    EnumRoute = 12,
    #[doc = "< routing configuration as SPA_TYPE_OBJECT_ParamRoute"]
    Route = 13,
    #[doc = "< Control parameter, a SPA_TYPE_Sequence"]
    Control = 14,
    #[doc = "< latency reporting, a SPA_TYPE_OBJECT_ParamLatency"]
    Latency = 15,
    #[doc = "< processing latency, a SPA_TYPE_OBJECT_ParamProcessLatency"]
    ProcessLatency = 16,
    #[doc = "< tag reporting, a SPA_TYPE_OBJECT_ParamTag. Since 0.3.79"]
    Tag = 17,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SpaParamAvailability {
    #[doc = "< unknown availability"]
    Unknown = 0,
    #[doc = "< not available"]
    No = 1,
    #[doc = "< available"]
    Yes = 2,
}
#[repr(u32)]
#[doc = " properties for SPA_TYPE_OBJECT_ParamIO"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SpaParamIo {
    Start = 0,
    #[doc = "< type ID, uniquely identifies the io area (Id enum spa_io_type)"]
    Id = 1,
    #[doc = "< size of the io area (Int)"]
    Size = 2,
}
#[repr(u32)]
#[doc = " properties for SPA_TYPE_OBJECT_ParamRoute"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SpaParamRoute {
    Start = 0,
    #[doc = "< index of the routing destination (Int)"]
    Index = 1,
    #[doc = "< direction, input/output (Id enum spa_direction)"]
    Direction = 2,
    #[doc = "< device id (Int)"]
    Device = 3,
    #[doc = "< name of the routing destination (String)"]
    Name = 4,
    #[doc = "< description of the destination (String)"]
    Description = 5,
    #[doc = "< priority of the destination (Int)"]
    Priority = 6,
    #[doc = "< availability of the destination\n  (Id enum spa_param_availability)"]
    Available = 7,
    #[doc = "< info (Struct(\n\t\t  Int : n_items,\n\t\t  (String : key,\n\t\t   String : value)*))"]
    Info = 8,
    #[doc = "< associated profile indexes (Array of Int)"]
    Profiles = 9,
    #[doc = "< properties SPA_TYPE_OBJECT_Props"]
    Props = 10,
    #[doc = "< associated device indexes (Array of Int)"]
    Devices = 11,
    #[doc = "< profile id (Int)"]
    Profile = 12,
    #[doc = "< If route should be saved (Bool)"]
    Save = 13,
}
#[repr(u32)]
#[doc = " predefined properties for SPA_TYPE_OBJECT_Props"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SpaProp {
    #[doc = "< an unknown property"]
    Unknown = 1,
    Device = 257,
    DeviceName = 258,
    DeviceFd = 259,
    Card = 260,
    CardName = 261,
    MinLatency = 262,
    MaxLatency = 263,
    Periods = 264,
    PeriodSize = 265,
    PeriodEvent = 266,
    Live = 267,
    Rate = 268,
    Quality = 269,
    BluetoothAudioCodec = 270,
    BluetoothOffloadActive = 271,
    WaveType = 65537,
    Frequency = 65538,
    #[doc = "< a volume (Float), 0.0 silence, 1.0 no attenutation"]
    Volume = 65539,
    #[doc = "< mute (Bool)"]
    Mute = 65540,
    PatternType = 65541,
    DitherType = 65542,
    Truncate = 65543,
    #[doc = "< a volume array, one (linear) volume per channel\n (Array of Float). 0.0 is silence, 1.0 is\n  without attenuation. This is the effective\n  volume that is applied. It can result\n  in a hardware volume and software volume\n  (see softVolumes)"]
    ChannelVolumes = 65544,
    #[doc = "< a volume base (Float)"]
    VolumeBase = 65545,
    #[doc = "< a volume step (Float)"]
    VolumeStep = 65546,
    #[doc = "< a channelmap array\n (Array (Id enum spa_audio_channel))"]
    ChannelMap = 65547,
    #[doc = "< mute (Bool)"]
    MonitorMute = 65548,
    #[doc = "< a volume array, one (linear) volume per\n  channel (Array of Float)"]
    MonitorVolumes = 65549,
    #[doc = "< delay adjustment"]
    LatencyOffsetNsec = 65550,
    #[doc = "< mute (Bool) applied in software"]
    SoftMute = 65551,
    #[doc = "< a volume array, one (linear) volume per channel\n (Array of Float). 0.0 is silence, 1.0 is without\n attenuation. This is the volume applied in\n software, there might be a part applied in\n hardware."]
    SoftVolumes = 65552,
    #[doc = "< enabled IEC958 (S/PDIF) codecs,\n  (Array (Id enum spa_audio_iec958_codec)"]
    Iec958Codecs = 65553,
    #[doc = "< Samples to ramp the volume over"]
    VolumeRampSamples = 65554,
    #[doc = "< Step or incremental Samples to ramp\n  the volume over"]
    VolumeRampStepSamples = 65555,
    #[doc = "< Time in millisec to ramp the volume over"]
    VolumeRampTime = 65556,
    #[doc = "< Step or incremental Time in nano seconds\n  to ramp the"]
    VolumeRampStepTime = 65557,
    #[doc = "< the scale or graph to used to ramp the\n  volume"]
    VolumeRampScale = 65558,
    Brightness = 131073,
    Contrast = 131074,
    Saturation = 131075,
    Hue = 131076,
    Gamma = 131077,
    Exposure = 131078,
    Gain = 131079,
    Sharpness = 131080,
    #[doc = "< simple control params\n    (Struct(\n\t  (String : key,\n\t   Pod    : value)*))"]
    Params = 524289,
}
#[repr(u32)]
#[doc = " media type for SPA_TYPE_OBJECT_Format"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SpaMediaType {
    Unknown = 0,
    Audio = 1,
    Video = 2,
    Image = 3,
    Binary = 4,
    Stream = 5,
    Application = 6,
}
#[repr(u32)]
#[doc = " media subtype for SPA_TYPE_OBJECT_Format"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SpaMediaSubtype {
    Unknown = 0,
    Raw = 1,
    Dsp = 2,
    Iec958 = 3,
    #[doc = " S/PDIF"]
    Dsd = 4,
    #[doc = " S/PDIF"]
    StartAudio = 65536,
    #[doc = " S/PDIF"]
    Mp3 = 65537,
    #[doc = " S/PDIF"]
    Aac = 65538,
    #[doc = " S/PDIF"]
    Vorbis = 65539,
    #[doc = " S/PDIF"]
    Wma = 65540,
    #[doc = " S/PDIF"]
    Ra = 65541,
    #[doc = " S/PDIF"]
    Sbc = 65542,
    #[doc = " S/PDIF"]
    Adpcm = 65543,
    #[doc = " S/PDIF"]
    G723 = 65544,
    #[doc = " S/PDIF"]
    G726 = 65545,
    #[doc = " S/PDIF"]
    G729 = 65546,
    #[doc = " S/PDIF"]
    Amr = 65547,
    #[doc = " S/PDIF"]
    Gsm = 65548,
    #[doc = " S/PDIF"]
    Alac = 65549,
    #[doc = " since 0.3.65"]
    Flac = 65550,
    #[doc = " since 0.3.65"]
    Ape = 65551,
    #[doc = " since 0.3.65"]
    Opus = 65552,
    #[doc = " since 0.3.68"]
    StartVideo = 131072,
    #[doc = " since 0.3.68"]
    H264 = 131073,
    #[doc = " since 0.3.68"]
    Mjpg = 131074,
    #[doc = " since 0.3.68"]
    Dv = 131075,
    #[doc = " since 0.3.68"]
    Mpegts = 131076,
    #[doc = " since 0.3.68"]
    H263 = 131077,
    #[doc = " since 0.3.68"]
    Mpeg1 = 131078,
    #[doc = " since 0.3.68"]
    Mpeg2 = 131079,
    #[doc = " since 0.3.68"]
    Mpeg4 = 131080,
    #[doc = " since 0.3.68"]
    Xvid = 131081,
    #[doc = " since 0.3.68"]
    Vc1 = 131082,
    #[doc = " since 0.3.68"]
    Vp8 = 131083,
    #[doc = " since 0.3.68"]
    Vp9 = 131084,
    #[doc = " since 0.3.68"]
    Bayer = 131085,
    #[doc = " since 0.3.68"]
    StartImage = 196608,
    #[doc = " since 0.3.68"]
    Jpeg = 196609,
    #[doc = " since 0.3.68"]
    StartBinary = 262144,
    #[doc = " since 0.3.68"]
    StartStream = 327680,
    #[doc = " since 0.3.68"]
    Midi = 327681,
    #[doc = " since 0.3.68"]
    StartApplication = 393216,
    #[doc = "< control stream, data contains\n  spa_pod_sequence with control info."]
    Control = 393217,
}
#[repr(u32)]
#[doc = " properties for audio SPA_TYPE_OBJECT_Format"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, num_derive :: FromPrimitive)]
pub enum SpaFormat {
    Start = 0,
    #[doc = "< media type (Id enum spa_media_type)"]
    MediaType = 1,
    #[doc = "< media subtype (Id enum spa_media_subtype)"]
    MediaSubtype = 2,
    StartAudio = 65536,
    #[doc = "< audio format, (Id enum spa_audio_format)"]
    AudioFormat = 65537,
    #[doc = "< optional flags (Int)"]
    AudioFlags = 65538,
    #[doc = "< sample rate (Int)"]
    AudioRate = 65539,
    #[doc = "< number of audio channels (Int)"]
    AudioChannels = 65540,
    #[doc = "< channel positions (Id enum spa_audio_position)"]
    AudioPosition = 65541,
    #[doc = "< codec used (IEC958) (Id enum spa_audio_iec958_codec)"]
    AudioIec958Codec = 65542,
    #[doc = "< bit order (Id enum spa_param_bitorder)"]
    AudioBitorder = 65543,
    #[doc = "< Interleave bytes (Int)"]
    AudioInterleave = 65544,
    #[doc = "< bit rate (Int)"]
    AudioBitrate = 65545,
    #[doc = "< audio data block alignment (Int)"]
    AudioBlockAlign = 65546,
    #[doc = "< AAC stream format, (Id enum spa_audio_aac_stream_format)"]
    AudioAacStreamFormat = 65547,
    #[doc = "< WMA profile (Id enum spa_audio_wma_profile)"]
    AudioWmaProfile = 65548,
    #[doc = "< AMR band mode (Id enum spa_audio_amr_band_mode)"]
    AudioAmrBandMode = 65549,
    StartVideo = 131072,
    #[doc = "< video format (Id enum spa_video_format)"]
    VideoFormat = 131073,
    #[doc = "< format modifier (Long)\n use only with DMA-BUF and omit for other buffer types"]
    VideoModifier = 131074,
    #[doc = "< size (Rectangle)"]
    VideoSize = 131075,
    #[doc = "< frame rate (Fraction)"]
    VideoFramerate = 131076,
    #[doc = "< maximum frame rate (Fraction)"]
    VideoMaxFramerate = 131077,
    #[doc = "< number of views (Int)"]
    VideoViews = 131078,
    #[doc = "< (Id enum spa_video_interlace_mode)"]
    VideoInterlaceMode = 131079,
    #[doc = "< (Rectangle)"]
    VideoPixelAspectRatio = 131080,
    #[doc = "< (Id enum spa_video_multiview_mode)"]
    VideoMultiviewMode = 131081,
    #[doc = "< (Id enum spa_video_multiview_flags)"]
    VideoMultiviewFlags = 131082,
    #[doc = "< /Id enum spa_video_chroma_site)"]
    VideoChromaSite = 131083,
    #[doc = "< /Id enum spa_video_color_range)"]
    VideoColorRange = 131084,
    #[doc = "< /Id enum spa_video_color_matrix)"]
    VideoColorMatrix = 131085,
    #[doc = "< /Id enum spa_video_transfer_function)"]
    VideoTransferFunction = 131086,
    #[doc = "< /Id enum spa_video_color_primaries)"]
    VideoColorPrimaries = 131087,
    #[doc = "< (Int)"]
    VideoProfile = 131088,
    #[doc = "< (Int)"]
    VideoLevel = 131089,
    #[doc = "< (Id enum spa_h264_stream_format)"]
    VideoH264StreamFormat = 131090,
    #[doc = "< (Id enum spa_h264_alignment)"]
    VideoH264Alignment = 131091,
    StartImage = 196608,
    StartBinary = 262144,
    StartStream = 327680,
    StartApplication = 393216,
}
