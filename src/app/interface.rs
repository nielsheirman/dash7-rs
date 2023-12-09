use deku::prelude::*;

use crate::{network::Addressee, session::QoS, types::VarInt};

/// Section 9.2.1
///
/// Parameters to handle the sending of a request.
// ALP SPEC: Add link to D7a section
#[derive(DekuRead, DekuWrite, Default, Debug, Clone, PartialEq)]
pub struct Dash7InterfaceConfiguration {
    pub qos: QoS,

    /// Flush Start Timeout in Compressed Format, unit is in seconds
    ///
    /// Maximum time to send the packet. This means that the modem will wait for a "good opportunity"
    /// to send the packet until the timeout, after which it will just send the packet over the
    /// air.
    ///
    /// A good opportunity is, for example, if we are sending another packet to the same target,
    /// then we can aggregate the requests, to avoid advertising twice. Another example would be if
    /// the target sends us a packet, the modem can aggregate our request to the response of the
    /// request of the target.
    pub dormant_session_timeout: VarInt,

    /// Response Execution Delay in Compressed Format, unit is in milliseconds.
    ///
    /// Time given to the target to process the request.
    #[cfg(not(feature = "subiot_v0"))]
    pub te: VarInt,

    /// Address of the target.
    pub addressee: Addressee,
}

#[derive(DekuRead, DekuWrite, Default, Debug, Clone, PartialEq)]
pub struct LoRaWANInterfaceConfiguration {
    #[deku(pad_bits_before = "5", bits = 1)]
    pub adr_enabled: bool,
    #[deku(bits = 1)]
    pub request_ack: bool,

    #[deku(pad_bits_before = "1")]
    pub application_port: u8,
    pub data_rate: u8,
}

#[derive(DekuRead, DekuWrite, Default, Debug, Clone, PartialEq)]
pub struct LoRaWANOTAAInterfaceConfiguration {
    pub base: LoRaWANInterfaceConfiguration,

    #[deku(count = "8")]
    pub device_eui: Vec<u8>,

    #[deku(count = "8")]
    pub app_eui: Vec<u8>,

    #[deku(count = "16")]
    pub app_key: Vec<u8>,
}

#[derive(DekuRead, DekuWrite, Default, Debug, Clone, PartialEq)]
pub struct LoRaWANABPInterfaceConfiguration {
    pub base: LoRaWANInterfaceConfiguration,

    #[deku(count = "16")]
    pub network_session_key: Vec<u8>,

    #[deku(count = "16")]
    pub app_session_key: Vec<u8>,

    pub device_address: u32,

    pub network_id: u32,
}

#[derive(DekuRead, DekuWrite, Debug, Clone, Copy, PartialEq)]
#[deku(bits = 8, type = "u8")]
pub enum InterfaceType {
    #[deku(id = "0x00")]
    Host,

    #[deku(id = "0x01")]
    Serial,

    #[deku(id = "0x02")]
    LoRaWanABP,

    #[deku(id = "0x03")]
    LoRaWanOTAA,

    #[deku(id = "0xD7")]
    Dash7,

    #[deku(id_pat = "_")]
    Unknown,
}

#[derive(DekuRead, DekuWrite, Debug, Clone, PartialEq)]
#[deku(ctx = "interface_type: InterfaceType", id = "interface_type")]
pub enum InterfaceConfiguration {
    #[deku(id = "InterfaceType::Host")]
    Host,

    #[deku(id = "InterfaceType::Serial")]
    Serial,

    #[deku(id = "InterfaceType::LoRaWanABP")]
    LoRaWanABP(LoRaWANABPInterfaceConfiguration),

    #[deku(id = "InterfaceType::LoRaWanOTAA")]
    LoRaWanOTAA(LoRaWANOTAAInterfaceConfiguration),

    #[deku(id = "InterfaceType::Dash7")]
    Dash7(Dash7InterfaceConfiguration),

    #[deku(id_pat = "_")]
    Unknown,
}
