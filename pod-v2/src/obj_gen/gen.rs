use super::*;
/// Spa:Pod:Object:Param:PropInfo
struct PropInfo<'a>(PodObjectDeserializer<'a>);
impl<'a> PropInfo<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /** Spa:Pod:Object:Param:PropInfo:id
        value-0: "Spa:Pod:Object:Param:Props:"
        value-1: "Spa:Pod:Object:Param:Props:unknown"
        value-257: "Spa:Pod:Object:Param:Props:device"
        value-258: "Spa:Pod:Object:Param:Props:deviceName"
        value-259: "Spa:Pod:Object:Param:Props:deviceFd"
        value-260: "Spa:Pod:Object:Param:Props:card"
        value-261: "Spa:Pod:Object:Param:Props:cardName"
        value-262: "Spa:Pod:Object:Param:Props:minLatency"
        value-263: "Spa:Pod:Object:Param:Props:maxLatency"
        value-264: "Spa:Pod:Object:Param:Props:periods"
        value-265: "Spa:Pod:Object:Param:Props:periodSize"
        value-266: "Spa:Pod:Object:Param:Props:periodEvent"
        value-267: "Spa:Pod:Object:Param:Props:live"
        value-268: "Spa:Pod:Object:Param:Props:rate"
        value-269: "Spa:Pod:Object:Param:Props:quality"
        value-270: "Spa:Pod:Object:Param:Props:bluetoothAudioCodec"
        value-271: "Spa:Pod:Object:Param:Props:bluetoothOffloadActive"
        value-65537: "Spa:Pod:Object:Param:Props:waveType"
        value-65538: "Spa:Pod:Object:Param:Props:frequency"
        value-65539: "Spa:Pod:Object:Param:Props:volume"
        value-65540: "Spa:Pod:Object:Param:Props:mute"
        value-65541: "Spa:Pod:Object:Param:Props:patternType"
        value-65542: "Spa:Pod:Object:Param:Props:ditherType"
        value-65543: "Spa:Pod:Object:Param:Props:truncate"
        value-65544: "Spa:Pod:Object:Param:Props:channelVolumes"
        value-65545: "Spa:Pod:Object:Param:Props:volumeBase"
        value-65546: "Spa:Pod:Object:Param:Props:volumeStep"
        value-65547: "Spa:Pod:Object:Param:Props:channelMap"
        value-65548: "Spa:Pod:Object:Param:Props:monitorMute"
        value-65549: "Spa:Pod:Object:Param:Props:monitorVolumes"
        value-65550: "Spa:Pod:Object:Param:Props:latencyOffsetNsec"
        value-65551: "Spa:Pod:Object:Param:Props:softMute"
        value-65552: "Spa:Pod:Object:Param:Props:softVolumes"
        value-65553: "Spa:Pod:Object:Param:Props:iec958Codecs"
        value-65554: "Spa:Pod:Object:Param:Props:volumeRampSamples"
        value-65555: "Spa:Pod:Object:Param:Props:volumeRampStepSamples"
        value-65556: "Spa:Pod:Object:Param:Props:volumeRampTime"
        value-65557: "Spa:Pod:Object:Param:Props:volumeRampStepTime"
        value-65558: "Spa:Pod:Object:Param:Props:volumeRampScale"
        value-131073: "Spa:Pod:Object:Param:Props:brightness"
        value-131074: "Spa:Pod:Object:Param:Props:contrast"
        value-131075: "Spa:Pod:Object:Param:Props:saturation"
        value-131076: "Spa:Pod:Object:Param:Props:hue"
        value-131077: "Spa:Pod:Object:Param:Props:gamma"
        value-131078: "Spa:Pod:Object:Param:Props:exposure"
        value-131079: "Spa:Pod:Object:Param:Props:gain"
        value-131080: "Spa:Pod:Object:Param:Props:sharpness"
        value-524289: "Spa:Pod:Object:Param:Props:params"
    */
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
    /// Spa:Pod:Object:Param:PropInfo:labels
    fn labels(&self) -> Option<PodStructDeserializer> {
        self.get(4u32)?.as_struct().ok()
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
struct Props<'a>(PodObjectDeserializer<'a>);
impl<'a> Props<'a> {
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
    /** Spa:Pod:Object:Param:Props:bluetoothAudioCodec
        value-1: "Spa:Enum:BluetoothAudioCodec:sbc"
        value-2: "Spa:Enum:BluetoothAudioCodec:sbc_xq"
        value-3: "Spa:Enum:BluetoothAudioCodec:mpeg"
        value-4: "Spa:Enum:BluetoothAudioCodec:aac"
        value-5: "Spa:Enum:BluetoothAudioCodec:aac_eld"
        value-6: "Spa:Enum:BluetoothAudioCodec:aptx"
        value-7: "Spa:Enum:BluetoothAudioCodec:aptx_hd"
        value-8: "Spa:Enum:BluetoothAudioCodec:ldac"
        value-9: "Spa:Enum:BluetoothAudioCodec:aptx_ll"
        value-10: "Spa:Enum:BluetoothAudioCodec:aptx_ll_duplex"
        value-11: "Spa:Enum:BluetoothAudioCodec:faststream"
        value-12: "Spa:Enum:BluetoothAudioCodec:faststream_duplex"
        value-13: "Spa:Enum:BluetoothAudioCodec:lc3plus_hr"
        value-14: "Spa:Enum:BluetoothAudioCodec:opus_05"
        value-15: "Spa:Enum:BluetoothAudioCodec:opus_05_51"
        value-16: "Spa:Enum:BluetoothAudioCodec:opus_05_71"
        value-17: "Spa:Enum:BluetoothAudioCodec:opus_05_duplex"
        value-18: "Spa:Enum:BluetoothAudioCodec:opus_05_pro"
        value-19: "Spa:Enum:BluetoothAudioCodec:opus_g"
        value-256: "Spa:Enum:BluetoothAudioCodec:cvsd"
        value-257: "Spa:Enum:BluetoothAudioCodec:msbc"
        value-258: "Spa:Enum:BluetoothAudioCodec:lc3_swb"
        value-512: "Spa:Enum:BluetoothAudioCodec:lc3"
    */
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
        parent: Array<Spa:floatArray>
    */
    fn channel_volumes(&self) -> Option<PodArrayDeserializer> {
        self.get(65544u32)?.as_array().ok()
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
        parent: Array<Spa:channelMap>
    */
    fn channel_map(&self) -> Option<PodArrayDeserializer> {
        self.get(65547u32)?.as_array().ok()
    }
    /// Spa:Pod:Object:Param:Props:monitorMute
    fn monitor_mute(&self) -> Option<bool> {
        self.get(65548u32)?.as_bool().ok()
    }
    /** Spa:Pod:Object:Param:Props:monitorVolumes
        parent: Array<Spa:floatArray>
    */
    fn monitor_volumes(&self) -> Option<PodArrayDeserializer> {
        self.get(65549u32)?.as_array().ok()
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
        parent: Array<Spa:floatArray>
    */
    fn soft_volumes(&self) -> Option<PodArrayDeserializer> {
        self.get(65552u32)?.as_array().ok()
    }
    /** Spa:Pod:Object:Param:Props:iec958Codecs
        parent: Array<Spa:iec958Codec>
    */
    fn iec958_codecs(&self) -> Option<PodArrayDeserializer> {
        self.get(65553u32)?.as_array().ok()
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
    /// Spa:Pod:Object:Param:Props:params
    fn params(&self) -> Option<PodStructDeserializer> {
        self.get(524289u32)?.as_struct().ok()
    }
}
/// Spa:Pod:Object:Param:Format
struct Format<'a>(PodObjectDeserializer<'a>);
impl<'a> Format<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /** Spa:Pod:Object:Param:Format:mediaType
        value-0: "Spa:Enum:MediaType:unknown"
        value-1: "Spa:Enum:MediaType:audio"
        value-2: "Spa:Enum:MediaType:video"
        value-3: "Spa:Enum:MediaType:image"
        value-4: "Spa:Enum:MediaType:binary"
        value-5: "Spa:Enum:MediaType:stream"
        value-6: "Spa:Enum:MediaType:application"
    */
    fn media_type(&self) -> Option<u32> {
        self.get(1u32)?.as_id().ok()
    }
    /** Spa:Pod:Object:Param:Format:mediaSubtype
        value-0: "Spa:Enum:MediaSubtype:unknown"
        value-1: "Spa:Enum:MediaSubtype:raw"
        value-2: "Spa:Enum:MediaSubtype:dsp"
        value-3: "Spa:Enum:MediaSubtype:iec958"
        value-4: "Spa:Enum:MediaSubtype:dsd"
        value-65537: "Spa:Enum:MediaSubtype:mp3"
        value-65538: "Spa:Enum:MediaSubtype:aac"
        value-65539: "Spa:Enum:MediaSubtype:vorbis"
        value-65540: "Spa:Enum:MediaSubtype:wma"
        value-65541: "Spa:Enum:MediaSubtype:ra"
        value-65542: "Spa:Enum:MediaSubtype:sbc"
        value-65543: "Spa:Enum:MediaSubtype:adpcm"
        value-65544: "Spa:Enum:MediaSubtype:g723"
        value-65545: "Spa:Enum:MediaSubtype:g726"
        value-65546: "Spa:Enum:MediaSubtype:g729"
        value-65547: "Spa:Enum:MediaSubtype:amr"
        value-65548: "Spa:Enum:MediaSubtype:gsm"
        value-65549: "Spa:Enum:MediaSubtype:alac"
        value-65550: "Spa:Enum:MediaSubtype:flac"
        value-65551: "Spa:Enum:MediaSubtype:ape"
        value-65552: "Spa:Enum:MediaSubtype:opus"
        value-131073: "Spa:Enum:MediaSubtype:h264"
        value-131074: "Spa:Enum:MediaSubtype:mjpg"
        value-131075: "Spa:Enum:MediaSubtype:dv"
        value-131076: "Spa:Enum:MediaSubtype:mpegts"
        value-131077: "Spa:Enum:MediaSubtype:h263"
        value-131078: "Spa:Enum:MediaSubtype:mpeg1"
        value-131079: "Spa:Enum:MediaSubtype:mpeg2"
        value-131080: "Spa:Enum:MediaSubtype:mpeg4"
        value-131081: "Spa:Enum:MediaSubtype:xvid"
        value-131082: "Spa:Enum:MediaSubtype:vc1"
        value-131083: "Spa:Enum:MediaSubtype:vp8"
        value-131084: "Spa:Enum:MediaSubtype:vp9"
        value-131085: "Spa:Enum:MediaSubtype:bayer"
        value-196609: "Spa:Enum:MediaSubtype:jpeg"
        value-327681: "Spa:Enum:MediaSubtype:midi"
        value-393217: "Spa:Enum:MediaSubtype:control"
    */
    fn media_subtype(&self) -> Option<u32> {
        self.get(2u32)?.as_id().ok()
    }
    /** Spa:Pod:Object:Param:Format:Audio:format
        value-0: "Spa:Enum:AudioFormat:UNKNOWN"
        value-1: "Spa:Enum:AudioFormat:ENCODED"
        value-257: "Spa:Enum:AudioFormat:S8"
        value-258: "Spa:Enum:AudioFormat:U8"
        value-259: "Spa:Enum:AudioFormat:S16LE"
        value-260: "Spa:Enum:AudioFormat:S16BE"
        value-261: "Spa:Enum:AudioFormat:U16LE"
        value-262: "Spa:Enum:AudioFormat:U16BE"
        value-263: "Spa:Enum:AudioFormat:S24_32LE"
        value-264: "Spa:Enum:AudioFormat:S24_32BE"
        value-265: "Spa:Enum:AudioFormat:U24_32LE"
        value-266: "Spa:Enum:AudioFormat:U24_32BE"
        value-267: "Spa:Enum:AudioFormat:S32LE"
        value-268: "Spa:Enum:AudioFormat:S32BE"
        value-269: "Spa:Enum:AudioFormat:U32LE"
        value-270: "Spa:Enum:AudioFormat:U32BE"
        value-271: "Spa:Enum:AudioFormat:S24LE"
        value-272: "Spa:Enum:AudioFormat:S24BE"
        value-273: "Spa:Enum:AudioFormat:U24LE"
        value-274: "Spa:Enum:AudioFormat:U24BE"
        value-275: "Spa:Enum:AudioFormat:S20LE"
        value-276: "Spa:Enum:AudioFormat:S20BE"
        value-277: "Spa:Enum:AudioFormat:U20LE"
        value-278: "Spa:Enum:AudioFormat:U20BE"
        value-279: "Spa:Enum:AudioFormat:S18LE"
        value-280: "Spa:Enum:AudioFormat:S18BE"
        value-281: "Spa:Enum:AudioFormat:U18LE"
        value-282: "Spa:Enum:AudioFormat:U18BE"
        value-283: "Spa:Enum:AudioFormat:F32LE"
        value-284: "Spa:Enum:AudioFormat:F32BE"
        value-285: "Spa:Enum:AudioFormat:F64LE"
        value-286: "Spa:Enum:AudioFormat:F64BE"
        value-287: "Spa:Enum:AudioFormat:ULAW"
        value-288: "Spa:Enum:AudioFormat:ALAW"
        value-513: "Spa:Enum:AudioFormat:U8P"
        value-514: "Spa:Enum:AudioFormat:S16P"
        value-515: "Spa:Enum:AudioFormat:S24_32P"
        value-516: "Spa:Enum:AudioFormat:S32P"
        value-517: "Spa:Enum:AudioFormat:S24P"
        value-518: "Spa:Enum:AudioFormat:F32P"
        value-519: "Spa:Enum:AudioFormat:F64P"
        value-520: "Spa:Enum:AudioFormat:S8P"
        value-259: "Spa:Enum:AudioFormat:S16"
        value-260: "Spa:Enum:AudioFormat:S16OE"
        value-261: "Spa:Enum:AudioFormat:U16"
        value-262: "Spa:Enum:AudioFormat:U16OE"
        value-263: "Spa:Enum:AudioFormat:S24_32"
        value-264: "Spa:Enum:AudioFormat:S24_32OE"
        value-265: "Spa:Enum:AudioFormat:U24_32"
        value-266: "Spa:Enum:AudioFormat:U24_32OE"
        value-267: "Spa:Enum:AudioFormat:S32"
        value-268: "Spa:Enum:AudioFormat:S32OE"
        value-269: "Spa:Enum:AudioFormat:U32"
        value-270: "Spa:Enum:AudioFormat:U32OE"
        value-271: "Spa:Enum:AudioFormat:S24"
        value-272: "Spa:Enum:AudioFormat:S24OE"
        value-273: "Spa:Enum:AudioFormat:U24"
        value-274: "Spa:Enum:AudioFormat:U24OE"
        value-275: "Spa:Enum:AudioFormat:S20"
        value-276: "Spa:Enum:AudioFormat:S20OE"
        value-277: "Spa:Enum:AudioFormat:U20"
        value-278: "Spa:Enum:AudioFormat:U20OE"
        value-279: "Spa:Enum:AudioFormat:S18"
        value-280: "Spa:Enum:AudioFormat:S18OE"
        value-281: "Spa:Enum:AudioFormat:U18"
        value-282: "Spa:Enum:AudioFormat:U18OE"
        value-283: "Spa:Enum:AudioFormat:F32"
        value-284: "Spa:Enum:AudioFormat:F32OE"
        value-285: "Spa:Enum:AudioFormat:F64"
        value-286: "Spa:Enum:AudioFormat:F64OE"
    */
    fn audio_format(&self) -> Option<u32> {
        self.get(65537u32)?.as_id().ok()
    }
    /** Spa:Pod:Object:Param:Format:Audio:flags
        value-0: "Spa:Flags:AudioFlags:none"
        value-1: "Spa:Flags:AudioFlags:unpositioned"
    */
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
        parent: Array<Spa:channelMap>
    */
    fn audio_position(&self) -> Option<PodArrayDeserializer> {
        self.get(65541u32)?.as_array().ok()
    }
    /** Spa:Pod:Object:Param:Format:Audio:iec958Codec
        value-0: "Spa:Enum:AudioIEC958Codec:UNKNOWN"
        value-1: "Spa:Enum:AudioIEC958Codec:PCM"
        value-2: "Spa:Enum:AudioIEC958Codec:DTS"
        value-3: "Spa:Enum:AudioIEC958Codec:AC3"
        value-4: "Spa:Enum:AudioIEC958Codec:MPEG"
        value-5: "Spa:Enum:AudioIEC958Codec:MPEG2-AAC"
        value-6: "Spa:Enum:AudioIEC958Codec:EAC3"
        value-7: "Spa:Enum:AudioIEC958Codec:TrueHD"
        value-8: "Spa:Enum:AudioIEC958Codec:DTS-HD"
    */
    fn audio_iec958_codec(&self) -> Option<u32> {
        self.get(65542u32)?.as_id().ok()
    }
    /** Spa:Pod:Object:Param:Format:Audio:bitorder
        value-0: "Spa:Enum:ParamBitorder:unknown"
        value-1: "Spa:Enum:ParamBitorder:msb"
        value-2: "Spa:Enum:ParamBitorder:lsb"
    */
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
    /** Spa:Pod:Object:Param:Format:Audio:AAC:streamFormat
        value-0: "Spa:Enum:AudioAACStreamFormat:UNKNOWN"
        value-1: "Spa:Enum:AudioAACStreamFormat:RAW"
        value-2: "Spa:Enum:AudioAACStreamFormat:MP2ADTS"
        value-3: "Spa:Enum:AudioAACStreamFormat:MP4ADTS"
        value-4: "Spa:Enum:AudioAACStreamFormat:MP4LOAS"
        value-5: "Spa:Enum:AudioAACStreamFormat:MP4LATM"
        value-6: "Spa:Enum:AudioAACStreamFormat:ADIF"
        value-7: "Spa:Enum:AudioAACStreamFormat:MP4FF"
    */
    fn audio_aac_stream_format(&self) -> Option<u32> {
        self.get(65547u32)?.as_id().ok()
    }
    /** Spa:Pod:Object:Param:Format:Audio:WMA:profile
        value-0: "Spa:Enum:AudioWMAProfile:UNKNOWN"
        value-1: "Spa:Enum:AudioWMAProfile:WMA7"
        value-2: "Spa:Enum:AudioWMAProfile:WMA8"
        value-3: "Spa:Enum:AudioWMAProfile:WMA9"
        value-4: "Spa:Enum:AudioWMAProfile:WMA10"
        value-5: "Spa:Enum:AudioWMAProfile:WMA9-Pro"
        value-6: "Spa:Enum:AudioWMAProfile:WMA9-Lossless"
        value-7: "Spa:Enum:AudioWMAProfile:WMA10-Lossless"
    */
    fn audio_wma_profile(&self) -> Option<u32> {
        self.get(65548u32)?.as_id().ok()
    }
    /** Spa:Pod:Object:Param:Format:Audio:AMR:bandMode
        value-0: "Spa:Enum:AudioAMRBandMode:UNKNOWN"
        value-1: "Spa:Enum:AudioAMRBandMode:NB"
        value-2: "Spa:Enum:AudioAMRBandMode:WB"
    */
    fn audio_amr_band_mode(&self) -> Option<u32> {
        self.get(65549u32)?.as_id().ok()
    }
    /** Spa:Pod:Object:Param:Format:Video:format
        value-1: "Spa:Enum:VideoFormat:encoded"
        value-2: "Spa:Enum:VideoFormat:I420"
        value-3: "Spa:Enum:VideoFormat:YV12"
        value-4: "Spa:Enum:VideoFormat:YUY2"
        value-5: "Spa:Enum:VideoFormat:UYVY"
        value-6: "Spa:Enum:VideoFormat:AYUV"
        value-7: "Spa:Enum:VideoFormat:RGBx"
        value-8: "Spa:Enum:VideoFormat:BGRx"
        value-9: "Spa:Enum:VideoFormat:xRGB"
        value-10: "Spa:Enum:VideoFormat:xBGR"
        value-11: "Spa:Enum:VideoFormat:RGBA"
        value-12: "Spa:Enum:VideoFormat:BGRA"
        value-13: "Spa:Enum:VideoFormat:ARGB"
        value-14: "Spa:Enum:VideoFormat:ABGR"
        value-15: "Spa:Enum:VideoFormat:RGB"
        value-16: "Spa:Enum:VideoFormat:BGR"
        value-17: "Spa:Enum:VideoFormat:Y41B"
        value-18: "Spa:Enum:VideoFormat:Y42B"
        value-19: "Spa:Enum:VideoFormat:YVYU"
        value-20: "Spa:Enum:VideoFormat:Y444"
        value-21: "Spa:Enum:VideoFormat:v210"
        value-22: "Spa:Enum:VideoFormat:v216"
        value-23: "Spa:Enum:VideoFormat:NV12"
        value-24: "Spa:Enum:VideoFormat:NV21"
        value-25: "Spa:Enum:VideoFormat:GRAY8"
        value-26: "Spa:Enum:VideoFormat:GRAY16_BE"
        value-27: "Spa:Enum:VideoFormat:GRAY16_LE"
        value-28: "Spa:Enum:VideoFormat:v308"
        value-29: "Spa:Enum:VideoFormat:RGB16"
        value-30: "Spa:Enum:VideoFormat:BGR16"
        value-31: "Spa:Enum:VideoFormat:RGB15"
        value-32: "Spa:Enum:VideoFormat:BGR15"
        value-33: "Spa:Enum:VideoFormat:UYVP"
        value-34: "Spa:Enum:VideoFormat:A420"
        value-35: "Spa:Enum:VideoFormat:RGB8P"
        value-36: "Spa:Enum:VideoFormat:YUV9"
        value-37: "Spa:Enum:VideoFormat:YVU9"
        value-38: "Spa:Enum:VideoFormat:IYU1"
        value-39: "Spa:Enum:VideoFormat:ARGB64"
        value-40: "Spa:Enum:VideoFormat:AYUV64"
        value-41: "Spa:Enum:VideoFormat:r210"
        value-42: "Spa:Enum:VideoFormat:I420_10BE"
        value-43: "Spa:Enum:VideoFormat:I420_10LE"
        value-44: "Spa:Enum:VideoFormat:I422_10BE"
        value-45: "Spa:Enum:VideoFormat:I422_10LE"
        value-46: "Spa:Enum:VideoFormat:Y444_10BE"
        value-47: "Spa:Enum:VideoFormat:Y444_10LE"
        value-48: "Spa:Enum:VideoFormat:GBR"
        value-49: "Spa:Enum:VideoFormat:GBR_10BE"
        value-50: "Spa:Enum:VideoFormat:GBR_10LE"
        value-51: "Spa:Enum:VideoFormat:NV16"
        value-52: "Spa:Enum:VideoFormat:NV24"
        value-53: "Spa:Enum:VideoFormat:NV12_64Z32"
        value-54: "Spa:Enum:VideoFormat:A420_10BE"
        value-55: "Spa:Enum:VideoFormat:A420_10LE"
        value-56: "Spa:Enum:VideoFormat:A422_10BE"
        value-57: "Spa:Enum:VideoFormat:A422_10LE"
        value-58: "Spa:Enum:VideoFormat:A444_10BE"
        value-59: "Spa:Enum:VideoFormat:A444_10LE"
        value-60: "Spa:Enum:VideoFormat:NV61"
        value-61: "Spa:Enum:VideoFormat:P010_10BE"
        value-62: "Spa:Enum:VideoFormat:P010_10LE"
        value-63: "Spa:Enum:VideoFormat:IYU2"
        value-64: "Spa:Enum:VideoFormat:VYUY"
        value-65: "Spa:Enum:VideoFormat:GBRA"
        value-66: "Spa:Enum:VideoFormat:GBRA_10BE"
        value-67: "Spa:Enum:VideoFormat:GBRA_10LE"
        value-68: "Spa:Enum:VideoFormat:GBR_12BE"
        value-69: "Spa:Enum:VideoFormat:GBR_12LE"
        value-70: "Spa:Enum:VideoFormat:GBRA_12BE"
        value-71: "Spa:Enum:VideoFormat:GBRA_12LE"
        value-72: "Spa:Enum:VideoFormat:I420_12BE"
        value-73: "Spa:Enum:VideoFormat:I420_12LE"
        value-74: "Spa:Enum:VideoFormat:I422_12BE"
        value-75: "Spa:Enum:VideoFormat:I422_12LE"
        value-76: "Spa:Enum:VideoFormat:Y444_12BE"
        value-77: "Spa:Enum:VideoFormat:Y444_12LE"
        value-78: "Spa:Enum:VideoFormat:RGBA_F16"
        value-79: "Spa:Enum:VideoFormat:RGBA_F32"
        value-80: "Spa:Enum:VideoFormat:xRGB_210LE"
        value-81: "Spa:Enum:VideoFormat:xBGR_210LE"
        value-82: "Spa:Enum:VideoFormat:RGBx_102LE"
        value-83: "Spa:Enum:VideoFormat:BGRx_102LE"
        value-84: "Spa:Enum:VideoFormat:ARGB_210LE"
        value-85: "Spa:Enum:VideoFormat:ABGR_210LE"
        value-86: "Spa:Enum:VideoFormat:RGBA_102LE"
        value-87: "Spa:Enum:VideoFormat:BGRA_102LE"
    */
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
    /** Spa:Pod:Object:Param:Format:Video:interlaceMode
        value-0: "Spa:Enum:VideoInterlaceMode:progressive"
        value-1: "Spa:Enum:VideoInterlaceMode:interleaved"
        value-2: "Spa:Enum:VideoInterlaceMode:mixed"
        value-3: "Spa:Enum:VideoInterlaceMode:fields"
    */
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
struct Buffers<'a>(PodObjectDeserializer<'a>);
impl<'a> Buffers<'a> {
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
struct Meta<'a>(PodObjectDeserializer<'a>);
impl<'a> Meta<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /** Spa:Pod:Object:Param:Meta:type
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
        self.get(1u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Meta:size
    fn size(&self) -> Option<i32> {
        self.get(2u32)?.as_i32().ok()
    }
}
/// Spa:Pod:Object:Param:IO
struct Io<'a>(PodObjectDeserializer<'a>);
impl<'a> Io<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /** Spa:Pod:Object:Param:IO:id
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
        self.get(1u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:IO:size
    fn size(&self) -> Option<i32> {
        self.get(2u32)?.as_i32().ok()
    }
}
/// Spa:Pod:Object:Param:Profile
struct Profile<'a>(PodObjectDeserializer<'a>);
impl<'a> Profile<'a> {
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
    /** Spa:Pod:Object:Param:Profile:available
        value-0: "Spa:Enum:ParamAvailability:unknown"
        value-1: "Spa:Enum:ParamAvailability:no"
        value-2: "Spa:Enum:ParamAvailability:yes"
    */
    fn available(&self) -> Option<u32> {
        self.get(5u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Profile:info
    fn info(&self) -> Option<PodStructDeserializer> {
        self.get(6u32)?.as_struct().ok()
    }
    /// Spa:Pod:Object:Param:Profile:classes
    fn classes(&self) -> Option<PodStructDeserializer> {
        self.get(7u32)?.as_struct().ok()
    }
    /// Spa:Pod:Object:Param:Profile:save
    fn save(&self) -> Option<bool> {
        self.get(8u32)?.as_bool().ok()
    }
}
/// Spa:Pod:Object:Param:PortConfig
struct PortConfig<'a>(PodObjectDeserializer<'a>);
impl<'a> PortConfig<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /** Spa:Pod:Object:Param:PortConfig:direction
        value-0: "Spa:Enum:Direction:Input"
        value-1: "Spa:Enum:Direction:Output"
    */
    fn direction(&self) -> Option<u32> {
        self.get(1u32)?.as_id().ok()
    }
    /** Spa:Pod:Object:Param:PortConfig:mode
        value-0: "Spa:Enum:ParamPortConfigMode:none"
        value-1: "Spa:Enum:ParamPortConfigMode:passthrough"
        value-2: "Spa:Enum:ParamPortConfigMode:convert"
        value-3: "Spa:Enum:ParamPortConfigMode:dsp"
    */
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
    /// Spa:Pod:Object:Param:PortConfig:format
    fn format(&self) -> Option<Format> {
        self.get(5u32)?.as_object().map(Format).ok()
    }
}
/// Spa:Pod:Object:Param:Route
struct Route<'a>(PodObjectDeserializer<'a>);
impl<'a> Route<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /// Spa:Pod:Object:Param:Route:index
    fn index(&self) -> Option<i32> {
        self.get(1u32)?.as_i32().ok()
    }
    /** Spa:Pod:Object:Param:Route:direction
        value-0: "Spa:Enum:Direction:Input"
        value-1: "Spa:Enum:Direction:Output"
    */
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
    /** Spa:Pod:Object:Param:Route:available
        value-0: "Spa:Enum:ParamAvailability:unknown"
        value-1: "Spa:Enum:ParamAvailability:no"
        value-2: "Spa:Enum:ParamAvailability:yes"
    */
    fn available(&self) -> Option<u32> {
        self.get(7u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Route:info
    fn info(&self) -> Option<PodStructDeserializer> {
        self.get(8u32)?.as_struct().ok()
    }
    /// Spa:Pod:Object:Param:Route:profiles
    fn profiles(&self) -> Option<i32> {
        self.get(9u32)?.as_i32().ok()
    }
    /// Spa:Pod:Object:Param:Route:props
    fn props(&self) -> Option<Props> {
        self.get(10u32)?.as_object().map(Props).ok()
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
struct Profiler<'a>(PodObjectDeserializer<'a>);
impl<'a> Profiler<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /// Spa:Pod:Object:Profiler:info
    fn info(&self) -> Option<PodStructDeserializer> {
        self.get(65537u32)?.as_struct().ok()
    }
    /// Spa:Pod:Object:Profiler:clock
    fn clock(&self) -> Option<PodStructDeserializer> {
        self.get(65538u32)?.as_struct().ok()
    }
    /// Spa:Pod:Object:Profiler:driverBlock
    fn driver_block(&self) -> Option<PodStructDeserializer> {
        self.get(65539u32)?.as_struct().ok()
    }
    /// Spa:Pod:Object:Profiler:followerBlock
    fn follower_block(&self) -> Option<PodStructDeserializer> {
        self.get(131073u32)?.as_struct().ok()
    }
}
/// Spa:Pod:Object:Param:Latency
struct Latency<'a>(PodObjectDeserializer<'a>);
impl<'a> Latency<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /** Spa:Pod:Object:Param:Latency:direction
        value-0: "Spa:Enum:Direction:Input"
        value-1: "Spa:Enum:Direction:Output"
    */
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
struct ProcessLatency<'a>(PodObjectDeserializer<'a>);
impl<'a> ProcessLatency<'a> {
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
struct Tag<'a>(PodObjectDeserializer<'a>);
impl<'a> Tag<'a> {
    fn get(&self, id: u32) -> Option<PodDeserializer> {
        todo!("{id}")
    }
    /** Spa:Pod:Object:Param:Tag:direction
        value-0: "Spa:Enum:Direction:Input"
        value-1: "Spa:Enum:Direction:Output"
    */
    fn direction(&self) -> Option<u32> {
        self.get(1u32)?.as_id().ok()
    }
    /// Spa:Pod:Object:Param:Tag:info
    fn info(&self) -> Option<PodStructDeserializer> {
        self.get(2u32)?.as_struct().ok()
    }
}
