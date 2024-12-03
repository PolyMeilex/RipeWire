use super::*;
macro_rules! opt_fmt {
    ($f:ident, $self:ident . $key:ident) => {
        if let Some(v) = $self.$key() {
            $f.field(stringify!($key), &v);
        }
    };
}
/// Spa:Pod:Object:Param:PropInfo
pub struct PropInfo<'a>(pub PodObjectDeserializer<'a>);
impl<'a> PropInfo<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        self.0.clone().find(|v| v.key == id).map(|v| v.value)
    }
    /// Spa:Pod:Object:Param:PropInfo:id
    fn id(&self) -> Option<SpaEnum<SpaProp>> {
        self.get(1u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:PropInfo:name
    fn name(&self) -> Option<&BStr> {
        self.get(2u32)?
            .as_str()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /** Spa:Pod:Object:Param:PropInfo:type
        parent: Pod
    */
    fn ty(&self) -> Option<PodDeserializer> {
        self.get(3u32)
    }
    /// Spa:Pod:Object:Param:PropInfo:labels
    fn labels(&self) -> Option<PodStructDeserializer> {
        self.get(4u32)?
            .as_struct()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:PropInfo:container
    fn container(&self) -> Option<u32> {
        self.get(5u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:PropInfo:params
    fn params(&self) -> Option<bool> {
        self.get(6u32)?
            .as_bool()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:PropInfo:description
    fn description(&self) -> Option<&BStr> {
        self.get(7u32)?
            .as_str()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
}
impl<'a> std::fmt::Debug for PropInfo<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("PropInfo");
        opt_fmt!(f, self.id);
        opt_fmt!(f, self.name);
        opt_fmt!(f, self.ty);
        opt_fmt!(f, self.labels);
        opt_fmt!(f, self.container);
        opt_fmt!(f, self.params);
        opt_fmt!(f, self.description);
        f.finish()
    }
}
/// Spa:Pod:Object:Param:Props
pub struct Props<'a>(pub PodObjectDeserializer<'a>);
impl<'a> Props<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        self.0.clone().find(|v| v.key == id).map(|v| v.value)
    }
    /// Spa:Pod:Object:Param:Props:device
    fn device(&self) -> Option<&BStr> {
        self.get(257u32)?
            .as_str()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:deviceName
    fn device_name(&self) -> Option<&BStr> {
        self.get(258u32)?
            .as_str()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:deviceFd
    fn device_fd(&self) -> Option<i64> {
        self.get(259u32)?
            .as_fd()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:card
    fn card(&self) -> Option<&BStr> {
        self.get(260u32)?
            .as_str()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:cardName
    fn card_name(&self) -> Option<&BStr> {
        self.get(261u32)?
            .as_str()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:minLatency
    fn min_latency(&self) -> Option<i32> {
        self.get(262u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:maxLatency
    fn max_latency(&self) -> Option<i32> {
        self.get(263u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:periods
    fn periods(&self) -> Option<i32> {
        self.get(264u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:periodSize
    fn period_size(&self) -> Option<i32> {
        self.get(265u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:periodEvent
    fn period_event(&self) -> Option<bool> {
        self.get(266u32)?
            .as_bool()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:live
    fn live(&self) -> Option<bool> {
        self.get(267u32)?
            .as_bool()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:rate
    fn rate(&self) -> Option<f64> {
        self.get(268u32)?
            .as_f64()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:quality
    fn quality(&self) -> Option<i32> {
        self.get(269u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:bluetoothAudioCodec
    fn bluetooth_audio_codec(&self) -> Option<SpaEnum<SpaBluetoothAudioCodec>> {
        self.get(270u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:bluetoothOffloadActive
    fn bluetooth_offload_active(&self) -> Option<bool> {
        self.get(271u32)?
            .as_bool()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:waveType
    fn wave_type(&self) -> Option<u32> {
        self.get(65537u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:frequency
    fn frequency(&self) -> Option<i32> {
        self.get(65538u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:volume
    fn volume(&self) -> Option<f32> {
        self.get(65539u32)?
            .as_f32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:mute
    fn mute(&self) -> Option<bool> {
        self.get(65540u32)?
            .as_bool()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:patternType
    fn pattern_type(&self) -> Option<u32> {
        self.get(65541u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:ditherType
    fn dither_type(&self) -> Option<u32> {
        self.get(65542u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:truncate
    fn truncate(&self) -> Option<bool> {
        self.get(65543u32)?
            .as_bool()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /** Spa:Pod:Object:Param:Props:channelVolumes
        parent: Array<Spa:floatArray>
    */
    fn channel_volumes(&self) -> Option<PodArrayDeserializer> {
        self.get(65544u32)?
            .as_array()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:volumeBase
    fn volume_base(&self) -> Option<f32> {
        self.get(65545u32)?
            .as_f32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:volumeStep
    fn volume_step(&self) -> Option<f32> {
        self.get(65546u32)?
            .as_f32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /** Spa:Pod:Object:Param:Props:channelMap
        parent: Array<Spa:channelMap>
    */
    fn channel_map(&self) -> Option<PodArrayDeserializer> {
        self.get(65547u32)?
            .as_array()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:monitorMute
    fn monitor_mute(&self) -> Option<bool> {
        self.get(65548u32)?
            .as_bool()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /** Spa:Pod:Object:Param:Props:monitorVolumes
        parent: Array<Spa:floatArray>
    */
    fn monitor_volumes(&self) -> Option<PodArrayDeserializer> {
        self.get(65549u32)?
            .as_array()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:latencyOffsetNsec
    fn latency_offset_nsec(&self) -> Option<i64> {
        self.get(65550u32)?
            .as_i64()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:softMute
    fn soft_mute(&self) -> Option<bool> {
        self.get(65551u32)?
            .as_bool()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /** Spa:Pod:Object:Param:Props:softVolumes
        parent: Array<Spa:floatArray>
    */
    fn soft_volumes(&self) -> Option<PodArrayDeserializer> {
        self.get(65552u32)?
            .as_array()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /** Spa:Pod:Object:Param:Props:iec958Codecs
        parent: Array<Spa:iec958Codec>
    */
    fn iec958_codecs(&self) -> Option<PodArrayDeserializer> {
        self.get(65553u32)?
            .as_array()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:volumeRampSamples
    fn volume_ramp_samples(&self) -> Option<i32> {
        self.get(65554u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:volumeRampStepSamples
    fn volume_ramp_step_samples(&self) -> Option<i32> {
        self.get(65555u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:volumeRampTime
    fn volume_ramp_time(&self) -> Option<i32> {
        self.get(65556u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:volumeRampStepTime
    fn volume_ramp_step_time(&self) -> Option<i32> {
        self.get(65557u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:volumeRampScale
    fn volume_ramp_scale(&self) -> Option<u32> {
        self.get(65558u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:brightness
    fn brightness(&self) -> Option<f32> {
        self.get(131073u32)?
            .as_f32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:contrast
    fn contrast(&self) -> Option<f32> {
        self.get(131074u32)?
            .as_f32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:saturation
    fn saturation(&self) -> Option<f32> {
        self.get(131075u32)?
            .as_f32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:hue
    fn hue(&self) -> Option<i32> {
        self.get(131076u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:gamma
    fn gamma(&self) -> Option<i32> {
        self.get(131077u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:exposure
    fn exposure(&self) -> Option<i32> {
        self.get(131078u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:gain
    fn gain(&self) -> Option<f32> {
        self.get(131079u32)?
            .as_f32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:sharpness
    fn sharpness(&self) -> Option<f32> {
        self.get(131080u32)?
            .as_f32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Props:params
    fn params(&self) -> Option<PodStructDeserializer> {
        self.get(524289u32)?
            .as_struct()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
}
impl<'a> std::fmt::Debug for Props<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Props");
        opt_fmt!(f, self.device);
        opt_fmt!(f, self.device_name);
        opt_fmt!(f, self.device_fd);
        opt_fmt!(f, self.card);
        opt_fmt!(f, self.card_name);
        opt_fmt!(f, self.min_latency);
        opt_fmt!(f, self.max_latency);
        opt_fmt!(f, self.periods);
        opt_fmt!(f, self.period_size);
        opt_fmt!(f, self.period_event);
        opt_fmt!(f, self.live);
        opt_fmt!(f, self.rate);
        opt_fmt!(f, self.quality);
        opt_fmt!(f, self.bluetooth_audio_codec);
        opt_fmt!(f, self.bluetooth_offload_active);
        opt_fmt!(f, self.wave_type);
        opt_fmt!(f, self.frequency);
        opt_fmt!(f, self.volume);
        opt_fmt!(f, self.mute);
        opt_fmt!(f, self.pattern_type);
        opt_fmt!(f, self.dither_type);
        opt_fmt!(f, self.truncate);
        opt_fmt!(f, self.channel_volumes);
        opt_fmt!(f, self.volume_base);
        opt_fmt!(f, self.volume_step);
        opt_fmt!(f, self.channel_map);
        opt_fmt!(f, self.monitor_mute);
        opt_fmt!(f, self.monitor_volumes);
        opt_fmt!(f, self.latency_offset_nsec);
        opt_fmt!(f, self.soft_mute);
        opt_fmt!(f, self.soft_volumes);
        opt_fmt!(f, self.iec958_codecs);
        opt_fmt!(f, self.volume_ramp_samples);
        opt_fmt!(f, self.volume_ramp_step_samples);
        opt_fmt!(f, self.volume_ramp_time);
        opt_fmt!(f, self.volume_ramp_step_time);
        opt_fmt!(f, self.volume_ramp_scale);
        opt_fmt!(f, self.brightness);
        opt_fmt!(f, self.contrast);
        opt_fmt!(f, self.saturation);
        opt_fmt!(f, self.hue);
        opt_fmt!(f, self.gamma);
        opt_fmt!(f, self.exposure);
        opt_fmt!(f, self.gain);
        opt_fmt!(f, self.sharpness);
        opt_fmt!(f, self.params);
        f.finish()
    }
}
/// Spa:Pod:Object:Param:Format
pub struct Format<'a>(pub PodObjectDeserializer<'a>);
impl<'a> Format<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        self.0.clone().find(|v| v.key == id).map(|v| v.value)
    }
    /// Spa:Pod:Object:Param:Format:mediaType
    fn media_type(&self) -> Option<SpaEnum<SpaMediaType>> {
        self.get(1u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:mediaSubtype
    fn media_subtype(&self) -> Option<SpaEnum<SpaMediaSubtype>> {
        self.get(2u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:format
    fn audio_format(&self) -> Option<SpaEnum<SpaAudioFormat>> {
        self.get(65537u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /** Spa:Pod:Object:Param:Format:Audio:flags
        enum: Spa:Flags:AudioFlags
        value-0: "Spa:Flags:AudioFlags:none"
        value-1: "Spa:Flags:AudioFlags:unpositioned"
    */
    fn audio_flags(&self) -> Option<u32> {
        self.get(65538u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:rate
    fn audio_rate(&self) -> Option<i32> {
        self.get(65539u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:channels
    fn audio_channels(&self) -> Option<i32> {
        self.get(65540u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /** Spa:Pod:Object:Param:Format:Audio:position
        parent: Array<Spa:channelMap>
    */
    fn audio_position(&self) -> Option<PodArrayDeserializer> {
        self.get(65541u32)?
            .as_array()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:iec958Codec
    fn audio_iec958_codec(&self) -> Option<SpaEnum<SpaAudioIec958Codec>> {
        self.get(65542u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:bitorder
    fn audio_bitorder(&self) -> Option<SpaEnum<SpaParamBitorder>> {
        self.get(65543u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:interleave
    fn audio_interleave(&self) -> Option<i32> {
        self.get(65544u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:bitrate
    fn audio_bitrate(&self) -> Option<i32> {
        self.get(65545u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:blockAlign
    fn audio_block_align(&self) -> Option<i32> {
        self.get(65546u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:AAC:streamFormat
    fn audio_aac_stream_format(&self) -> Option<SpaEnum<SpaAudioAacStreamFormat>> {
        self.get(65547u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:WMA:profile
    fn audio_wma_profile(&self) -> Option<SpaEnum<SpaAudioWmaProfile>> {
        self.get(65548u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Audio:AMR:bandMode
    fn audio_amr_band_mode(&self) -> Option<SpaEnum<SpaAudioAmrBandMode>> {
        self.get(65549u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:format
    fn video_format(&self) -> Option<SpaEnum<SpaVideoFormat>> {
        self.get(131073u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:modifier
    fn video_modifier(&self) -> Option<i64> {
        self.get(131074u32)?
            .as_i64()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:size
    fn video_size(&self) -> Option<SpaRectangle> {
        self.get(131075u32)?
            .as_rectangle()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:framerate
    fn video_framerate(&self) -> Option<SpaFraction> {
        self.get(131076u32)?
            .as_fraction()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:maxFramerate
    fn video_max_framerate(&self) -> Option<SpaFraction> {
        self.get(131077u32)?
            .as_fraction()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:views
    fn video_views(&self) -> Option<i32> {
        self.get(131078u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:interlaceMode
    fn video_interlace_mode(&self) -> Option<SpaEnum<SpaVideoInterlaceMode>> {
        self.get(131079u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:pixelAspectRatio
    fn video_pixel_aspect_ratio(&self) -> Option<SpaFraction> {
        self.get(131080u32)?
            .as_fraction()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:multiviewMode
    fn video_multiview_mode(&self) -> Option<SpaEnum<SpaVideoMultiviewMode>> {
        self.get(131081u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:multiviewFlags
    fn video_multiview_flags(&self) -> Option<SpaEnum<SpaVideoMultiviewFlags>> {
        self.get(131082u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:chromaSite
    fn video_chroma_site(&self) -> Option<u32> {
        self.get(131083u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:colorRange
    fn video_color_range(&self) -> Option<u32> {
        self.get(131084u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:colorMatrix
    fn video_color_matrix(&self) -> Option<u32> {
        self.get(131085u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:transferFunction
    fn video_transfer_function(&self) -> Option<u32> {
        self.get(131086u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:colorPrimaries
    fn video_color_primaries(&self) -> Option<u32> {
        self.get(131087u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:profile
    fn video_profile(&self) -> Option<i32> {
        self.get(131088u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:level
    fn video_level(&self) -> Option<i32> {
        self.get(131089u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:H264:streamFormat
    fn video_h264_stream_format(&self) -> Option<u32> {
        self.get(131090u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Format:Video:H264:alignment
    fn video_h264_alignment(&self) -> Option<u32> {
        self.get(131091u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
}
impl<'a> std::fmt::Debug for Format<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Format");
        opt_fmt!(f, self.media_type);
        opt_fmt!(f, self.media_subtype);
        opt_fmt!(f, self.audio_format);
        opt_fmt!(f, self.audio_flags);
        opt_fmt!(f, self.audio_rate);
        opt_fmt!(f, self.audio_channels);
        opt_fmt!(f, self.audio_position);
        opt_fmt!(f, self.audio_iec958_codec);
        opt_fmt!(f, self.audio_bitorder);
        opt_fmt!(f, self.audio_interleave);
        opt_fmt!(f, self.audio_bitrate);
        opt_fmt!(f, self.audio_block_align);
        opt_fmt!(f, self.audio_aac_stream_format);
        opt_fmt!(f, self.audio_wma_profile);
        opt_fmt!(f, self.audio_amr_band_mode);
        opt_fmt!(f, self.video_format);
        opt_fmt!(f, self.video_modifier);
        opt_fmt!(f, self.video_size);
        opt_fmt!(f, self.video_framerate);
        opt_fmt!(f, self.video_max_framerate);
        opt_fmt!(f, self.video_views);
        opt_fmt!(f, self.video_interlace_mode);
        opt_fmt!(f, self.video_pixel_aspect_ratio);
        opt_fmt!(f, self.video_multiview_mode);
        opt_fmt!(f, self.video_multiview_flags);
        opt_fmt!(f, self.video_chroma_site);
        opt_fmt!(f, self.video_color_range);
        opt_fmt!(f, self.video_color_matrix);
        opt_fmt!(f, self.video_transfer_function);
        opt_fmt!(f, self.video_color_primaries);
        opt_fmt!(f, self.video_profile);
        opt_fmt!(f, self.video_level);
        opt_fmt!(f, self.video_h264_stream_format);
        opt_fmt!(f, self.video_h264_alignment);
        f.finish()
    }
}
/// Spa:Pod:Object:Param:Buffers
pub struct Buffers<'a>(pub PodObjectDeserializer<'a>);
impl<'a> Buffers<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        self.0.clone().find(|v| v.key == id).map(|v| v.value)
    }
    /// Spa:Pod:Object:Param:Buffers:buffers
    fn buffers(&self) -> Option<i32> {
        self.get(1u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Buffers:blocks
    fn blocks(&self) -> Option<i32> {
        self.get(2u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Buffers:BlockInfo:size
    fn block_info_size(&self) -> Option<i32> {
        self.get(3u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Buffers:BlockInfo:stride
    fn block_info_stride(&self) -> Option<i32> {
        self.get(4u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Buffers:BlockInfo:align
    fn block_info_align(&self) -> Option<i32> {
        self.get(5u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Buffers:BlockInfo:dataType
    fn block_info_data_type(&self) -> Option<i32> {
        self.get(6u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Buffers:BlockInfo:metaType
    fn block_info_meta_type(&self) -> Option<i32> {
        self.get(7u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
}
impl<'a> std::fmt::Debug for Buffers<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Buffers");
        opt_fmt!(f, self.buffers);
        opt_fmt!(f, self.blocks);
        opt_fmt!(f, self.block_info_size);
        opt_fmt!(f, self.block_info_stride);
        opt_fmt!(f, self.block_info_align);
        opt_fmt!(f, self.block_info_data_type);
        opt_fmt!(f, self.block_info_meta_type);
        f.finish()
    }
}
/// Spa:Pod:Object:Param:Meta
pub struct Meta<'a>(pub PodObjectDeserializer<'a>);
impl<'a> Meta<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        self.0.clone().find(|v| v.key == id).map(|v| v.value)
    }
    /** Spa:Pod:Object:Param:Meta:type
        enum: Spa:Pointer:Meta
        value-0: "Spa:Pointer:Meta:Invalid"
        value-1: "Spa:Pointer:Meta:Header"
        value-2: "Spa:Pointer:Meta:Region:VideoCrop"
        value-3: "Spa:Pointer:Meta:Array:Region:VideoDamage"
        value-4: "Spa:Pointer:Meta:Bitmap"
        value-5: "Spa:Pointer:Meta:Cursor"
        value-6: "Spa:Pointer:Meta:Control"
        value-7: "Spa:Pointer:Meta:Busy"
        value-8: "Spa:Pointer:Meta:VideoTransform"
        value-9: "Spa:Pointer:Meta:SyncTimeline"
    */
    fn ty(&self) -> Option<u32> {
        self.get(1u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Meta:size
    fn size(&self) -> Option<i32> {
        self.get(2u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
}
impl<'a> std::fmt::Debug for Meta<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Meta");
        opt_fmt!(f, self.ty);
        opt_fmt!(f, self.size);
        f.finish()
    }
}
/// Spa:Pod:Object:Param:IO
pub struct Io<'a>(pub PodObjectDeserializer<'a>);
impl<'a> Io<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        self.0.clone().find(|v| v.key == id).map(|v| v.value)
    }
    /** Spa:Pod:Object:Param:IO:id
        enum: Spa:Enum:IO
        value-0: "Spa:Enum:IO:Invalid"
        value-1: "Spa:Enum:IO:Buffers"
        value-2: "Spa:Enum:IO:Range"
        value-3: "Spa:Enum:IO:Clock"
        value-4: "Spa:Enum:IO:Latency"
        value-5: "Spa:Enum:IO:Control"
        value-6: "Spa:Enum:IO:Notify"
        value-7: "Spa:Enum:IO:Position"
        value-8: "Spa:Enum:IO:RateMatch"
        value-9: "Spa:Enum:IO:Memory"
        value-10: "Spa:Enum:IO:AsyncBuffers"
    */
    fn id(&self) -> Option<u32> {
        self.get(1u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:IO:size
    fn size(&self) -> Option<i32> {
        self.get(2u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
}
impl<'a> std::fmt::Debug for Io<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Io");
        opt_fmt!(f, self.id);
        opt_fmt!(f, self.size);
        f.finish()
    }
}
/// Spa:Pod:Object:Param:Profile
pub struct Profile<'a>(pub PodObjectDeserializer<'a>);
impl<'a> Profile<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        self.0.clone().find(|v| v.key == id).map(|v| v.value)
    }
    /// Spa:Pod:Object:Param:Profile:index
    fn index(&self) -> Option<i32> {
        self.get(1u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Profile:name
    fn name(&self) -> Option<&BStr> {
        self.get(2u32)?
            .as_str()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Profile:description
    fn description(&self) -> Option<&BStr> {
        self.get(3u32)?
            .as_str()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Profile:priority
    fn priority(&self) -> Option<i32> {
        self.get(4u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /** Spa:Pod:Object:Param:Profile:available
        enum: Spa:Enum:ParamAvailability
        value-0: "Spa:Enum:ParamAvailability:unknown"
        value-1: "Spa:Enum:ParamAvailability:no"
        value-2: "Spa:Enum:ParamAvailability:yes"
    */
    fn available(&self) -> Option<u32> {
        self.get(5u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Profile:info
    fn info(&self) -> Option<PodStructDeserializer> {
        self.get(6u32)?
            .as_struct()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Profile:classes
    fn classes(&self) -> Option<PodStructDeserializer> {
        self.get(7u32)?
            .as_struct()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Profile:save
    fn save(&self) -> Option<bool> {
        self.get(8u32)?
            .as_bool()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
}
impl<'a> std::fmt::Debug for Profile<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Profile");
        opt_fmt!(f, self.index);
        opt_fmt!(f, self.name);
        opt_fmt!(f, self.description);
        opt_fmt!(f, self.priority);
        opt_fmt!(f, self.available);
        opt_fmt!(f, self.info);
        opt_fmt!(f, self.classes);
        opt_fmt!(f, self.save);
        f.finish()
    }
}
/// Spa:Pod:Object:Param:PortConfig
pub struct PortConfig<'a>(pub PodObjectDeserializer<'a>);
impl<'a> PortConfig<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        self.0.clone().find(|v| v.key == id).map(|v| v.value)
    }
    /// Spa:Pod:Object:Param:PortConfig:direction
    fn direction(&self) -> Option<SpaEnum<SpaDirection>> {
        self.get(1u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:PortConfig:mode
    fn mode(&self) -> Option<SpaEnum<SpaParamPortConfigMode>> {
        self.get(2u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:PortConfig:monitor
    fn monitor(&self) -> Option<bool> {
        self.get(3u32)?
            .as_bool()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:PortConfig:control
    fn control(&self) -> Option<bool> {
        self.get(4u32)?
            .as_bool()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:PortConfig:format
    fn format(&self) -> Option<Format> {
        self.get(5u32)?
            .as_object()
            .map(Format)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
}
impl<'a> std::fmt::Debug for PortConfig<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("PortConfig");
        opt_fmt!(f, self.direction);
        opt_fmt!(f, self.mode);
        opt_fmt!(f, self.monitor);
        opt_fmt!(f, self.control);
        opt_fmt!(f, self.format);
        f.finish()
    }
}
/// Spa:Pod:Object:Param:Route
pub struct Route<'a>(pub PodObjectDeserializer<'a>);
impl<'a> Route<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        self.0.clone().find(|v| v.key == id).map(|v| v.value)
    }
    /// Spa:Pod:Object:Param:Route:index
    fn index(&self) -> Option<i32> {
        self.get(1u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Route:direction
    fn direction(&self) -> Option<SpaEnum<SpaDirection>> {
        self.get(2u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Route:device
    fn device(&self) -> Option<i32> {
        self.get(3u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Route:name
    fn name(&self) -> Option<&BStr> {
        self.get(4u32)?
            .as_str()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Route:description
    fn description(&self) -> Option<&BStr> {
        self.get(5u32)?
            .as_str()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Route:priority
    fn priority(&self) -> Option<i32> {
        self.get(6u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /** Spa:Pod:Object:Param:Route:available
        enum: Spa:Enum:ParamAvailability
        value-0: "Spa:Enum:ParamAvailability:unknown"
        value-1: "Spa:Enum:ParamAvailability:no"
        value-2: "Spa:Enum:ParamAvailability:yes"
    */
    fn available(&self) -> Option<u32> {
        self.get(7u32)?
            .as_id()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Route:info
    fn info(&self) -> Option<PodStructDeserializer> {
        self.get(8u32)?
            .as_struct()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /** Spa:Pod:Object:Param:Route:profiles
        parent: Array<Spa:intArray>
    */
    fn profiles(&self) -> Option<PodArrayDeserializer> {
        self.get(9u32)?
            .as_array()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Route:props
    fn props(&self) -> Option<Props> {
        self.get(10u32)?
            .as_object()
            .map(Props)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /** Spa:Pod:Object:Param:Route:devices
        parent: Array<Spa:intArray>
    */
    fn devices(&self) -> Option<PodArrayDeserializer> {
        self.get(11u32)?
            .as_array()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Route:profile
    fn profile(&self) -> Option<i32> {
        self.get(12u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Route:save
    fn save(&self) -> Option<bool> {
        self.get(13u32)?
            .as_bool()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
}
impl<'a> std::fmt::Debug for Route<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Route");
        opt_fmt!(f, self.index);
        opt_fmt!(f, self.direction);
        opt_fmt!(f, self.device);
        opt_fmt!(f, self.name);
        opt_fmt!(f, self.description);
        opt_fmt!(f, self.priority);
        opt_fmt!(f, self.available);
        opt_fmt!(f, self.info);
        opt_fmt!(f, self.profiles);
        opt_fmt!(f, self.props);
        opt_fmt!(f, self.devices);
        opt_fmt!(f, self.profile);
        opt_fmt!(f, self.save);
        f.finish()
    }
}
/// Spa:Pod:Object:Profiler
pub struct Profiler<'a>(pub PodObjectDeserializer<'a>);
impl<'a> Profiler<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        self.0.clone().find(|v| v.key == id).map(|v| v.value)
    }
    /// Spa:Pod:Object:Profiler:info
    fn info(&self) -> Option<PodStructDeserializer> {
        self.get(65537u32)?
            .as_struct()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Profiler:clock
    fn clock(&self) -> Option<PodStructDeserializer> {
        self.get(65538u32)?
            .as_struct()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Profiler:driverBlock
    fn driver_block(&self) -> Option<PodStructDeserializer> {
        self.get(65539u32)?
            .as_struct()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Profiler:followerBlock
    fn follower_block(&self) -> Option<PodStructDeserializer> {
        self.get(131073u32)?
            .as_struct()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Profiler:followerClock
    fn follower_clock(&self) -> Option<PodStructDeserializer> {
        self.get(131074u32)?
            .as_struct()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
}
impl<'a> std::fmt::Debug for Profiler<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Profiler");
        opt_fmt!(f, self.info);
        opt_fmt!(f, self.clock);
        opt_fmt!(f, self.driver_block);
        opt_fmt!(f, self.follower_block);
        opt_fmt!(f, self.follower_clock);
        f.finish()
    }
}
/// Spa:Pod:Object:Param:Latency
pub struct Latency<'a>(pub PodObjectDeserializer<'a>);
impl<'a> Latency<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        self.0.clone().find(|v| v.key == id).map(|v| v.value)
    }
    /// Spa:Pod:Object:Param:Latency:direction
    fn direction(&self) -> Option<SpaEnum<SpaDirection>> {
        self.get(1u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Latency:minQuantum
    fn min_quantum(&self) -> Option<f32> {
        self.get(2u32)?
            .as_f32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Latency:maxQuantum
    fn max_quantum(&self) -> Option<f32> {
        self.get(3u32)?
            .as_f32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Latency:minRate
    fn min_rate(&self) -> Option<i32> {
        self.get(4u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Latency:maxRate
    fn max_rate(&self) -> Option<i32> {
        self.get(5u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Latency:minNs
    fn min_ns(&self) -> Option<i64> {
        self.get(6u32)?
            .as_i64()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Latency:maxNs
    fn max_ns(&self) -> Option<i64> {
        self.get(7u32)?
            .as_i64()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
}
impl<'a> std::fmt::Debug for Latency<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Latency");
        opt_fmt!(f, self.direction);
        opt_fmt!(f, self.min_quantum);
        opt_fmt!(f, self.max_quantum);
        opt_fmt!(f, self.min_rate);
        opt_fmt!(f, self.max_rate);
        opt_fmt!(f, self.min_ns);
        opt_fmt!(f, self.max_ns);
        f.finish()
    }
}
/// Spa:Pod:Object:Param:ProcessLatency
pub struct ProcessLatency<'a>(pub PodObjectDeserializer<'a>);
impl<'a> ProcessLatency<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        self.0.clone().find(|v| v.key == id).map(|v| v.value)
    }
    /// Spa:Pod:Object:Param:ProcessLatency:quantum
    fn quantum(&self) -> Option<f32> {
        self.get(1u32)?
            .as_f32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:ProcessLatency:rate
    fn rate(&self) -> Option<i32> {
        self.get(2u32)?
            .as_i32()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:ProcessLatency:ns
    fn ns(&self) -> Option<i64> {
        self.get(3u32)?
            .as_i64()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
}
impl<'a> std::fmt::Debug for ProcessLatency<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("ProcessLatency");
        opt_fmt!(f, self.quantum);
        opt_fmt!(f, self.rate);
        opt_fmt!(f, self.ns);
        f.finish()
    }
}
/// Spa:Pod:Object:Param:Tag
pub struct Tag<'a>(pub PodObjectDeserializer<'a>);
impl<'a> Tag<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        self.0.clone().find(|v| v.key == id).map(|v| v.value)
    }
    /// Spa:Pod:Object:Param:Tag:direction
    fn direction(&self) -> Option<SpaEnum<SpaDirection>> {
        self.get(1u32)?
            .as_id()
            .map(SpaEnum::from_raw)
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
    /// Spa:Pod:Object:Param:Tag:info
    fn info(&self) -> Option<PodStructDeserializer> {
        self.get(2u32)?
            .as_struct()
            .map_err(|err| unreachable!("{err}"))
            .ok()
    }
}
impl<'a> std::fmt::Debug for Tag<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Tag");
        opt_fmt!(f, self.direction);
        opt_fmt!(f, self.info);
        f.finish()
    }
}
