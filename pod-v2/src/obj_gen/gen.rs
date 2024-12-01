use super::*;
/// Spa:Pod:Object:Param:PropInfo
struct PropInfo;
impl PropInfo {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /// Spa:Pod:Object:Param:PropInfo:id
    fn id(&self) -> Option<u32> {
        self.get(1u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:PropInfo:name
    fn name(&self) -> Option<&BStr> {
        self.get(2u32)?.as_str().ok()
    }
    /** Spa:Pod:Object:Param:PropInfo:type
        parent: Pod
    */
    fn ty(&self) -> Option<PodDeserializer> {
        self.get(3u32)
    }
    /** Spa:Pod:Object:Param:PropInfo:labels
        parent: Struct
    */
    fn labels(&self) -> Option<PodDeserializer> {
        self.get(4u32)
    }
    /// Spa:Pod:Object:Param:PropInfo:container
    fn container(&self) -> Option<u32> {
        self.get(5u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:PropInfo:params
    fn params(&self) -> Option<bool> {
        self.get(6u32)?.as_bool().ok()
    }
    /// Spa:Pod:Object:Param:PropInfo:description
    fn description(&self) -> Option<&BStr> {
        self.get(7u32)?.as_str().ok()
    }
}
/// Spa:Pod:Object:Param:Props
struct Props;
impl Props {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /// Spa:Pod:Object:Param:Props:device
    fn device(&self) -> Option<&BStr> {
        self.get(257u32)?.as_str().ok()
    }
    /// Spa:Pod:Object:Param:Props:deviceName
    fn device_name(&self) -> Option<&BStr> {
        self.get(258u32)?.as_str().ok()
    }
    /// Spa:Pod:Object:Param:Props:deviceFd
    fn device_fd(&self) -> Option<i64> {
        self.get(259u32)?.as_fd().ok()
    }
    /// Spa:Pod:Object:Param:Props:card
    fn card(&self) -> Option<&BStr> {
        self.get(260u32)?.as_str().ok()
    }
    /// Spa:Pod:Object:Param:Props:cardName
    fn card_name(&self) -> Option<&BStr> {
        self.get(261u32)?.as_str().ok()
    }
    /// Spa:Pod:Object:Param:Props:minLatency
    fn min_latency(&self) -> Option<i32> {
        self.get(262u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:maxLatency
    fn max_latency(&self) -> Option<i32> {
        self.get(263u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:periods
    fn periods(&self) -> Option<i32> {
        self.get(264u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:periodSize
    fn period_size(&self) -> Option<i32> {
        self.get(265u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:periodEvent
    fn period_event(&self) -> Option<bool> {
        self.get(266u32)?.as_bool().ok()
    }
    /// Spa:Pod:Object:Param:Props:live
    fn live(&self) -> Option<bool> {
        self.get(267u32)?.as_bool().ok()
    }
    /// Spa:Pod:Object:Param:Props:rate
    fn rate(&self) -> Option<f64> {
        self.get(268u32)?.as_f64().ok()
    }
    /// Spa:Pod:Object:Param:Props:quality
    fn quality(&self) -> Option<i32> {
        self.get(269u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:bluetoothAudioCodec
    fn bluetooth_audio_codec(&self) -> Option<u32> {
        self.get(270u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Props:bluetoothOffloadActive
    fn bluetooth_offload_active(&self) -> Option<bool> {
        self.get(271u32)?.as_bool().ok()
    }
    /// Spa:Pod:Object:Param:Props:waveType
    fn wave_type(&self) -> Option<u32> {
        self.get(65537u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Props:frequency
    fn frequency(&self) -> Option<i32> {
        self.get(65538u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:volume
    fn volume(&self) -> Option<f32> {
        self.get(65539u32)?.as_f32().ok()
    }
    /// Spa:Pod:Object:Param:Props:mute
    fn mute(&self) -> Option<bool> {
        self.get(65540u32)?.as_bool().ok()
    }
    /// Spa:Pod:Object:Param:Props:patternType
    fn pattern_type(&self) -> Option<u32> {
        self.get(65541u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Props:ditherType
    fn dither_type(&self) -> Option<u32> {
        self.get(65542u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Props:truncate
    fn truncate(&self) -> Option<bool> {
        self.get(65543u32)?.as_bool().ok()
    }
    /** Spa:Pod:Object:Param:Props:channelVolumes
        parent: Array
    */
    fn channel_volumes(&self) -> Option<PodDeserializer> {
        self.get(65544u32)
    }
    /// Spa:Pod:Object:Param:Props:volumeBase
    fn volume_base(&self) -> Option<f32> {
        self.get(65545u32)?.as_f32().ok()
    }
    /// Spa:Pod:Object:Param:Props:volumeStep
    fn volume_step(&self) -> Option<f32> {
        self.get(65546u32)?.as_f32().ok()
    }
    /** Spa:Pod:Object:Param:Props:channelMap
        parent: Array
    */
    fn channel_map(&self) -> Option<PodDeserializer> {
        self.get(65547u32)
    }
    /// Spa:Pod:Object:Param:Props:monitorMute
    fn monitor_mute(&self) -> Option<bool> {
        self.get(65548u32)?.as_bool().ok()
    }
    /** Spa:Pod:Object:Param:Props:monitorVolumes
        parent: Array
    */
    fn monitor_volumes(&self) -> Option<PodDeserializer> {
        self.get(65549u32)
    }
    /// Spa:Pod:Object:Param:Props:latencyOffsetNsec
    fn latency_offset_nsec(&self) -> Option<i64> {
        self.get(65550u32)?.as_i64().ok()
    }
    /// Spa:Pod:Object:Param:Props:softMute
    fn soft_mute(&self) -> Option<bool> {
        self.get(65551u32)?.as_bool().ok()
    }
    /** Spa:Pod:Object:Param:Props:softVolumes
        parent: Array
    */
    fn soft_volumes(&self) -> Option<PodDeserializer> {
        self.get(65552u32)
    }
    /** Spa:Pod:Object:Param:Props:iec958Codecs
        parent: Array
    */
    fn iec958_codecs(&self) -> Option<PodDeserializer> {
        self.get(65553u32)
    }
    /// Spa:Pod:Object:Param:Props:volumeRampSamples
    fn volume_ramp_samples(&self) -> Option<i32> {
        self.get(65554u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:volumeRampStepSamples
    fn volume_ramp_step_samples(&self) -> Option<i32> {
        self.get(65555u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:volumeRampTime
    fn volume_ramp_time(&self) -> Option<i32> {
        self.get(65556u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:volumeRampStepTime
    fn volume_ramp_step_time(&self) -> Option<i32> {
        self.get(65557u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:volumeRampScale
    fn volume_ramp_scale(&self) -> Option<u32> {
        self.get(65558u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Props:brightness
    fn brightness(&self) -> Option<i32> {
        self.get(131073u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:contrast
    fn contrast(&self) -> Option<i32> {
        self.get(131074u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:saturation
    fn saturation(&self) -> Option<i32> {
        self.get(131075u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:hue
    fn hue(&self) -> Option<i32> {
        self.get(131076u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:gamma
    fn gamma(&self) -> Option<i32> {
        self.get(131077u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:exposure
    fn exposure(&self) -> Option<i32> {
        self.get(131078u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:gain
    fn gain(&self) -> Option<i32> {
        self.get(131079u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Props:sharpness
    fn sharpness(&self) -> Option<i32> {
        self.get(131080u32)?.as_i32().ok()
    }
    /** Spa:Pod:Object:Param:Props:params
        parent: Struct
    */
    fn params(&self) -> Option<PodDeserializer> {
        self.get(524289u32)
    }
}
/// Spa:Pod:Object:Param:Format
struct Format;
impl Format {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /// Spa:Pod:Object:Param:Format:mediaType
    fn media_type(&self) -> Option<u32> {
        self.get(1u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:mediaSubtype
    fn media_subtype(&self) -> Option<u32> {
        self.get(2u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:format
    fn audio_format(&self) -> Option<u32> {
        self.get(65537u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:flags
    fn audio_flags(&self) -> Option<u32> {
        self.get(65538u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:rate
    fn audio_rate(&self) -> Option<i32> {
        self.get(65539u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:channels
    fn audio_channels(&self) -> Option<i32> {
        self.get(65540u32)?.as_i32().ok()
    }
    /** Spa:Pod:Object:Param:Format:Audio:position
        parent: Array
    */
    fn audio_position(&self) -> Option<PodDeserializer> {
        self.get(65541u32)
    }
    /// Spa:Pod:Object:Param:Format:Audio:iec958Codec
    fn audio_iec958_codec(&self) -> Option<u32> {
        self.get(65542u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:bitorder
    fn audio_bitorder(&self) -> Option<u32> {
        self.get(65543u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:interleave
    fn audio_interleave(&self) -> Option<i32> {
        self.get(65544u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:bitrate
    fn audio_bitrate(&self) -> Option<i32> {
        self.get(65545u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:blockAlign
    fn audio_block_align(&self) -> Option<i32> {
        self.get(65546u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:AAC:streamFormat
    fn audio_aac_stream_format(&self) -> Option<u32> {
        self.get(65547u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:WMA:profile
    fn audio_wma_profile(&self) -> Option<u32> {
        self.get(65548u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:AMR:bandMode
    fn audio_amr_band_mode(&self) -> Option<u32> {
        self.get(65549u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:format
    fn video_format(&self) -> Option<u32> {
        self.get(131073u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:modifier
    fn video_modifier(&self) -> Option<i64> {
        self.get(131074u32)?.as_i64().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:size
    fn video_size(&self) -> Option<SpaRectangle> {
        self.get(131075u32)?.as_rectangle().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:framerate
    fn video_framerate(&self) -> Option<SpaFraction> {
        self.get(131076u32)?.as_fraction().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:maxFramerate
    fn video_max_framerate(&self) -> Option<SpaFraction> {
        self.get(131077u32)?.as_fraction().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:views
    fn video_views(&self) -> Option<i32> {
        self.get(131078u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:interlaceMode
    fn video_interlace_mode(&self) -> Option<u32> {
        self.get(131079u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:pixelAspectRatio
    fn video_pixel_aspect_ratio(&self) -> Option<SpaFraction> {
        self.get(131080u32)?.as_fraction().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:multiviewMode
    fn video_multiview_mode(&self) -> Option<u32> {
        self.get(131081u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:multiviewFlags
    fn video_multiview_flags(&self) -> Option<u32> {
        self.get(131082u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:chromaSite
    fn video_chroma_site(&self) -> Option<u32> {
        self.get(131083u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:colorRange
    fn video_color_range(&self) -> Option<u32> {
        self.get(131084u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:colorMatrix
    fn video_color_matrix(&self) -> Option<u32> {
        self.get(131085u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:transferFunction
    fn video_transfer_function(&self) -> Option<u32> {
        self.get(131086u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:colorPrimaries
    fn video_color_primaries(&self) -> Option<u32> {
        self.get(131087u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:profile
    fn video_profile(&self) -> Option<i32> {
        self.get(131088u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:level
    fn video_level(&self) -> Option<i32> {
        self.get(131089u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:H264:streamFormat
    fn video_h264_stream_format(&self) -> Option<u32> {
        self.get(131090u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:H264:alignment
    fn video_h264_alignment(&self) -> Option<u32> {
        self.get(131091u32)?.as_id().ok()
    }
}
/// Spa:Pod:Object:Param:Buffers
struct Buffers;
impl Buffers {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /// Spa:Pod:Object:Param:Buffers:buffers
    fn buffers(&self) -> Option<i32> {
        self.get(1u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Buffers:blocks
    fn blocks(&self) -> Option<i32> {
        self.get(2u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Buffers:BlockInfo:size
    fn block_info_size(&self) -> Option<i32> {
        self.get(3u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Buffers:BlockInfo:stride
    fn block_info_stride(&self) -> Option<i32> {
        self.get(4u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Buffers:BlockInfo:align
    fn block_info_align(&self) -> Option<i32> {
        self.get(5u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Buffers:BlockInfo:dataType
    fn block_info_data_type(&self) -> Option<i32> {
        self.get(6u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Buffers:BlockInfo:metaType
    fn block_info_meta_type(&self) -> Option<i32> {
        self.get(7u32)?.as_i32().ok()
    }
}
/// Spa:Pod:Object:Param:Meta
struct Meta;
impl Meta {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /// Spa:Pod:Object:Param:Meta:type
    fn ty(&self) -> Option<u32> {
        self.get(1u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Meta:size
    fn size(&self) -> Option<i32> {
        self.get(2u32)?.as_i32().ok()
    }
}
/// Spa:Pod:Object:Param:IO
struct Io;
impl Io {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /// Spa:Pod:Object:Param:IO:id
    fn id(&self) -> Option<u32> {
        self.get(1u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:IO:size
    fn size(&self) -> Option<i32> {
        self.get(2u32)?.as_i32().ok()
    }
}
/// Spa:Pod:Object:Param:Profile
struct Profile;
impl Profile {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /// Spa:Pod:Object:Param:Profile:index
    fn index(&self) -> Option<i32> {
        self.get(1u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Profile:name
    fn name(&self) -> Option<&BStr> {
        self.get(2u32)?.as_str().ok()
    }
    /// Spa:Pod:Object:Param:Profile:description
    fn description(&self) -> Option<&BStr> {
        self.get(3u32)?.as_str().ok()
    }
    /// Spa:Pod:Object:Param:Profile:priority
    fn priority(&self) -> Option<i32> {
        self.get(4u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Profile:available
    fn available(&self) -> Option<u32> {
        self.get(5u32)?.as_id().ok()
    }
    /** Spa:Pod:Object:Param:Profile:info
        parent: Struct
    */
    fn info(&self) -> Option<PodDeserializer> {
        self.get(6u32)
    }
    /** Spa:Pod:Object:Param:Profile:classes
        parent: Struct
    */
    fn classes(&self) -> Option<PodDeserializer> {
        self.get(7u32)
    }
    /// Spa:Pod:Object:Param:Profile:save
    fn save(&self) -> Option<bool> {
        self.get(8u32)?.as_bool().ok()
    }
}
/// Spa:Pod:Object:Param:PortConfig
struct PortConfig;
impl PortConfig {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /// Spa:Pod:Object:Param:PortConfig:direction
    fn direction(&self) -> Option<u32> {
        self.get(1u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:PortConfig:mode
    fn mode(&self) -> Option<u32> {
        self.get(2u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:PortConfig:monitor
    fn monitor(&self) -> Option<bool> {
        self.get(3u32)?.as_bool().ok()
    }
    /// Spa:Pod:Object:Param:PortConfig:control
    fn control(&self) -> Option<bool> {
        self.get(4u32)?.as_bool().ok()
    }
    /** Spa:Pod:Object:Param:PortConfig:format
        parent: ObjectFormat
    */
    fn format(&self) -> Option<PodDeserializer> {
        self.get(5u32)
    }
}
/// Spa:Pod:Object:Param:Route
struct Route;
impl Route {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /// Spa:Pod:Object:Param:Route:index
    fn index(&self) -> Option<i32> {
        self.get(1u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Route:direction
    fn direction(&self) -> Option<u32> {
        self.get(2u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Route:device
    fn device(&self) -> Option<i32> {
        self.get(3u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Route:name
    fn name(&self) -> Option<&BStr> {
        self.get(4u32)?.as_str().ok()
    }
    /// Spa:Pod:Object:Param:Route:description
    fn description(&self) -> Option<&BStr> {
        self.get(5u32)?.as_str().ok()
    }
    /// Spa:Pod:Object:Param:Route:priority
    fn priority(&self) -> Option<i32> {
        self.get(6u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Route:available
    fn available(&self) -> Option<u32> {
        self.get(7u32)?.as_id().ok()
    }
    /** Spa:Pod:Object:Param:Route:info
        parent: Struct
    */
    fn info(&self) -> Option<PodDeserializer> {
        self.get(8u32)
    }
    /// Spa:Pod:Object:Param:Route:profiles
    fn profiles(&self) -> Option<i32> {
        self.get(9u32)?.as_i32().ok()
    }
    /** Spa:Pod:Object:Param:Route:props
        parent: ObjectProps
    */
    fn props(&self) -> Option<PodDeserializer> {
        self.get(10u32)
    }
    /// Spa:Pod:Object:Param:Route:devices
    fn devices(&self) -> Option<i32> {
        self.get(11u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Route:profile
    fn profile(&self) -> Option<i32> {
        self.get(12u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Route:save
    fn save(&self) -> Option<bool> {
        self.get(13u32)?.as_bool().ok()
    }
}
/// Spa:Pod:Object:Profiler
struct Profiler;
impl Profiler {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /** Spa:Pod:Object:Profiler:info
        parent: Struct
    */
    fn info(&self) -> Option<PodDeserializer> {
        self.get(65537u32)
    }
    /** Spa:Pod:Object:Profiler:clock
        parent: Struct
    */
    fn clock(&self) -> Option<PodDeserializer> {
        self.get(65538u32)
    }
    /** Spa:Pod:Object:Profiler:driverBlock
        parent: Struct
    */
    fn driver_block(&self) -> Option<PodDeserializer> {
        self.get(65539u32)
    }
    /** Spa:Pod:Object:Profiler:followerBlock
        parent: Struct
    */
    fn follower_block(&self) -> Option<PodDeserializer> {
        self.get(131073u32)
    }
}
/// Spa:Pod:Object:Param:Latency
struct Latency;
impl Latency {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /// Spa:Pod:Object:Param:Latency:direction
    fn direction(&self) -> Option<u32> {
        self.get(1u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Latency:minQuantum
    fn min_quantum(&self) -> Option<f32> {
        self.get(2u32)?.as_f32().ok()
    }
    /// Spa:Pod:Object:Param:Latency:maxQuantum
    fn max_quantum(&self) -> Option<f32> {
        self.get(3u32)?.as_f32().ok()
    }
    /// Spa:Pod:Object:Param:Latency:minRate
    fn min_rate(&self) -> Option<i32> {
        self.get(4u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Latency:maxRate
    fn max_rate(&self) -> Option<i32> {
        self.get(5u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Latency:minNs
    fn min_ns(&self) -> Option<i64> {
        self.get(6u32)?.as_i64().ok()
    }
    /// Spa:Pod:Object:Param:Latency:maxNs
    fn max_ns(&self) -> Option<i64> {
        self.get(7u32)?.as_i64().ok()
    }
}
/// Spa:Pod:Object:Param:ProcessLatency
struct ProcessLatency;
impl ProcessLatency {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /// Spa:Pod:Object:Param:ProcessLatency:quantum
    fn quantum(&self) -> Option<f32> {
        self.get(1u32)?.as_f32().ok()
    }
    /// Spa:Pod:Object:Param:ProcessLatency:rate
    fn rate(&self) -> Option<i32> {
        self.get(2u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:ProcessLatency:ns
    fn ns(&self) -> Option<i64> {
        self.get(3u32)?.as_i64().ok()
    }
}
/// Spa:Pod:Object:Param:Tag
struct Tag;
impl Tag {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /// Spa:Pod:Object:Param:Tag:direction
    fn direction(&self) -> Option<u32> {
        self.get(1u32)?.as_id().ok()
    }
    /** Spa:Pod:Object:Param:Tag:info
        parent: Struct
    */
    fn info(&self) -> Option<PodDeserializer> {
        self.get(2u32)
    }
}
