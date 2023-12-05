use deku::prelude::*;

use crate::alp::{
    physical::{ChannelHeader, SubBand},
    varint::VarInt,
};

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Default, Debug, Clone, PartialEq)]
#[deku(bits=6, type="u8")]
pub enum CsmaCaMode {
    #[default]
    #[deku(id="0")] Unc,
    #[deku(id="1")] Aind,
    #[deku(id="2")] Raind,
    #[deku(id="3")] Rigd,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubProfile {
    pub subband_bitmap: u8,
    pub scan_automation_period: VarInt,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AccessProfile {
    pub channel_header: ChannelHeader,
    #[deku(count = "4")]
    pub sub_profiles: Vec<SubProfile>,
    #[deku(count = "8")]
    pub sub_bands: Vec<SubBand>,
}