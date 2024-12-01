use crate::{
    deserialize::{
        OwnedPod, PodArrayDeserializer, PodObjectDeserializer, PodSequenceDeserializer,
        PodStructDeserializer,
    },
    PodDeserializer,
};
use bstr::BStr;
use libspa_consts::{
    SpaAudioFormat, SpaAudioIec958Codec, SpaBluetoothAudioCodec, SpaDirection, SpaEnum,
    SpaFraction, SpaMediaSubtype, SpaMediaType, SpaMetaType, SpaParamPortConfigMode, SpaProp,
    SpaRectangle, SpaVideoFormat, SpaVideoInterlaceMode, SpaVideoMultiviewFlags,
    SpaVideoMultiviewMode,
};

mod gen;
pub use gen::*;
