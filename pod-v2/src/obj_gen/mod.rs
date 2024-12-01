use crate::{
    deserialize::{
        OwnedPod, PodArrayDeserializer, PodObjectDeserializer, PodSequenceDeserializer,
        PodStructDeserializer,
    },
    PodDeserializer,
};
use bstr::BStr;
use libspa_consts::{SpaFraction, SpaRectangle};

mod gen;
pub use gen::*;
