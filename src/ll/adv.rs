//! Advertising physical channel PDU
//!
//!    LSB                                                          MSB
//!   ┌──────────┬──────────┬─────────┬──────────┬──────────┬──────────┬────────────────────────────────────────┐
//!   │ PDU Type │ RFU      │ ChSel   │ TxAdd    │ RxAdd    │ Length   │                                        │
//!   │ (4 bits) │ (1 bit)  │ (1 bit) │ (1 bit)  │ (1 bit)  │ (8 bits) │                                        │
//!   ├──────────┴──────────┴─────────┴──────────┴──────────┴──────────┼────────────────────────────────────────┤
//!   │                        HEADER (16 bits)                        │           Payload (1-255 bytes)        │
//!   └────────────────────────────────────────────────────────────────┴────────────────────────────────────────┘
//!
//! The ChSel, TxAdd and RxAdd fields of the advertising physical
//! channel PDU that are contained in the header contain information
//! specific to the PDU type defined for each advertising physical
//! channel PDU separately (represented by X).
//!
//! If the ChSel, TxAdd or RxAdd fields are not defined as used in a
//! given PDU then they shall be considered reserved for future use.
//! (represented by -)
//!
//! Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-970b9251-9089-5ea4-1634-41defd816278

use crate::ll::{Address, AddressType};
use crate::phy::Packet;
use defmt::Format;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Format)]
pub enum ParseError {
    InvalidType,
    InvalidLength,
    InvalidHeader,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Format)]
pub enum AdvPdu<'a> {
    AdvInd(AdvInd<'a>) = AdvInd::TYPE,
    AdvDirectInd(AdvDirectInd) = AdvDirectInd::TYPE,
    AdvNonconnInd(AdvNonconnInd<'a>) = AdvNonconnInd::TYPE,
    AdvScanInd(AdvScanInd<'a>) = AdvScanInd::TYPE,
    ScanReq(ScanReq) = ScanReq::TYPE,
    ScanRsp(ScanRsp<'a>) = ScanRsp::TYPE,
}

impl<'a> From<AdvInd<'a>> for AdvPdu<'a> {
    fn from(adv_ind: AdvInd<'a>) -> Self {
        Self::AdvInd(adv_ind)
    }
}

impl<'a> From<AdvDirectInd> for AdvPdu<'a> {
    fn from(adv_direct_ind: AdvDirectInd) -> Self {
        Self::AdvDirectInd(adv_direct_ind)
    }
}

impl<'a> From<AdvNonconnInd<'a>> for AdvPdu<'a> {
    fn from(adv_nonconn_ind: AdvNonconnInd<'a>) -> Self {
        Self::AdvNonconnInd(adv_nonconn_ind)
    }
}

impl<'a> From<AdvScanInd<'a>> for AdvPdu<'a> {
    fn from(adv_scan_ind: AdvScanInd<'a>) -> Self {
        Self::AdvScanInd(adv_scan_ind)
    }
}

impl<'a> From<ScanReq> for AdvPdu<'a> {
    fn from(scan_req: ScanReq) -> Self {
        Self::ScanReq(scan_req)
    }
}

impl<'a> From<ScanRsp<'a>> for AdvPdu<'a> {
    fn from(scan_rsp: ScanRsp<'a>) -> Self {
        Self::ScanRsp(scan_rsp)
    }
}

pub fn parse<'a>(bytes: &'a [u8]) -> Result<AdvPdu<'a>, ParseError> {
    if let Ok(packet) = bytes[0..14]
        .try_into()
        .map_err(|_| ParseError::InvalidType)
        .and_then(AdvDirectInd::reception_parse)
    {
        Ok(packet.into())
    } else if let Ok(packet) = bytes[0..39]
        .try_into()
        .map_err(|_| ParseError::InvalidType)
        .and_then(AdvInd::reception_parse)
    {
        Ok(packet.into())
    } else if let Ok(packet) = bytes[0..39]
        .try_into()
        .map_err(|_| ParseError::InvalidType)
        .and_then(AdvNonconnInd::reception_parse)
    {
        Ok(packet.into())
    } else if let Ok(packet) = bytes[0..39]
        .try_into()
        .map_err(|_| ParseError::InvalidType)
        .and_then(AdvScanInd::reception_parse)
    {
        Ok(packet.into())
    } else if let Ok(packet) = bytes[0..14]
        .try_into()
        .map_err(|_| ParseError::InvalidType)
        .and_then(ScanReq::reception_parse)
    {
        Ok(packet.into())
    } else if let Ok(packet) = bytes[0..39]
        .try_into()
        .map_err(|_| ParseError::InvalidType)
        .and_then(ScanRsp::reception_parse)
    {
        Ok(packet.into())
    } else {
        Err(ParseError::InvalidType)
    }
}

#[derive(Debug, Clone, Format)]
struct Header {
    flags: Flags,
    length: u8,
}

impl<'a> Packet<'_, 2> for Header {
    type Error = ();

    fn transmission_bytes(&self) -> [u8; 2] {
        [self.flags.transmission_bytes()[0], self.length]
    }

    fn reception_parse(bytes: &[u8; 2]) -> Result<Self, Self::Error> {
        Ok(Self {
            flags: Flags::reception_parse(&bytes[0..1].try_into().unwrap())?,
            length: bytes[1],
        })
    }
}

impl Header {
    // The TxAdd in the advertising physical channel PDU header indicates whether the
    // advertiser’s address in the AdvA field is public (TxAdd = 0) or random (TxAdd = 1).
    // Also works for the RxAdd
    fn address_type_to_header_bit(address_type: AddressType) -> bool {
        match address_type {
            AddressType::Public => false,
            AddressType::Random => true,
        }
    }

    fn header_bit_to_address_type(bit: bool) -> AddressType {
        match bit {
            false => AddressType::Public,
            true => AddressType::Random,
        }
    }

    fn new(pdu_type: u8, length: u8) -> Self {
        Self {
            flags: Flags {
                rx_add: false,
                tx_add: false,
                ch_sel: false,
                rfu: false,
                pdu_type: pdu_type & 0b1111,
            },
            length,
        }
    }

    fn with_tx(adv_address_type: AddressType, pdu_type: u8, length: u8) -> Self {
        let mut header = Self::new(pdu_type, length);
        header.flags.tx_add = Self::address_type_to_header_bit(adv_address_type);
        header
    }

    fn with_rxtx(
        adv_address_type: AddressType,
        target_address_type: AddressType,
        pdu_type: u8,
        length: u8,
    ) -> Self {
        let mut header = Self::with_tx(adv_address_type, pdu_type, length);
        header.flags.rx_add = Self::address_type_to_header_bit(target_address_type);
        header
    }
}

#[derive(Debug, Clone, Format)]
struct Flags {
    rx_add: bool,
    tx_add: bool,
    ch_sel: bool,
    rfu: bool,
    pdu_type: u8,
}

impl<'a> Packet<'_, 1> for Flags {
    type Error = ();

    /// RxAdd | TxAdd | ChSel | RFU | PDU Type
    fn transmission_bytes(&self) -> [u8; 1] {
        let bool2bit = |b: bool| -> u8 {
            if b {
                1
            } else {
                0
            }
        };
        let byte = (bool2bit(self.rx_add)) << 7
            | (bool2bit(self.tx_add)) << 6
            | (bool2bit(self.ch_sel)) << 5
            | (bool2bit(self.rfu)) << 4
            | (self.pdu_type & 0b1111);

        [byte]
    }

    fn reception_parse(bytes: &[u8; 1]) -> Result<Self, Self::Error> {
        let byte = bytes[0];
        Ok(Self {
            rx_add: (byte & 0b1000_0000) != 0,
            tx_add: (byte & 0b0100_0000) != 0,
            ch_sel: (byte & 0b0010_0000) != 0,
            rfu: (byte & 0b0001_0000) != 0,
            pdu_type: byte & 0b0000_1111,
        })
    }
}

/// Advertising Connectable and Scannable Undirected event
/// Used in connectable and scannable undirected advertising events
///
///   |LSB                                               MSB|LSB    MSB|LSB      MSB|LSB                     MSB|
///   ┌──────────┬──────────┬─────────┬──────────┬──────────┬──────────┬───────────┬────────────────────────────┐
///   │ PDU Type:│ RFU:     │ ChSel:  │ TxAdd:   │ RxAdd:   │ Length   │ AdvAddr   │        AdvData             │
///   │ 0b0000   │ -        │ -       │ (1 bit)  │ -        │ (8 bits) │ (6 bytes) │       (0-31 bytes)         │
///   ├──────────┴──────────┴─────────┴──────────┴──────────┴──────────┼───────────┴────────────────────────────┤
///   │                        HEADER (2 bytes)                        │           Payload (6-37 bytes)         │
///   └────────────────────────────────────────────────────────────────┴────────────────────────────────────────┘
/// The TxAdd indicate the type of the advertiser address, either public or random.
///
/// The AdvA field contains the advertiser’s public or random device address as indicated by TxAdd.
/// The AdvData field, if not empty, contains Advertising Data from the advertiser’s Host.
///
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-3544231c-d808-9b6f-8f5a-d45c1c467d4e
#[derive(Debug, Clone, PartialEq, Eq, Format)]
pub struct AdvInd<'a> {
    adv_address: Address,
    adv_data: &'a [u8],
}

impl<'a> AdvInd<'a> {
    const TYPE: u8 = 0b0000;

    pub fn new(adv_address: Address, adv_data: &'a [u8]) -> Self {
        Self {
            adv_address,
            adv_data,
        }
    }
}

impl<'a> Packet<'a, 39> for AdvInd<'a> {
    type Error = ParseError;
    fn transmission_bytes(&self) -> [u8; 39] {
        let mut bytes = [0u8; 39];

        let header = Header::with_tx(
            self.adv_address.r#type,
            Self::TYPE,
            6 + self.adv_data.len() as u8, // total pdu length
        );

        bytes[..2].copy_from_slice(&header.transmission_bytes()); // write header
        bytes[2..8].copy_from_slice(&self.adv_address.transmission_bytes()); // write address
        bytes[8..(8 + self.adv_data.len())].copy_from_slice(self.adv_data); // write data

        bytes
    }

    fn reception_parse(bytes: &'a [u8; 39]) -> Result<Self, Self::Error> {
        if bytes.len() < 8 {
            return Err(ParseError::InvalidLength);
        }

        let (header, pdu) = bytes.split_at(2);

        let header = Header::reception_parse(header.try_into().unwrap()).unwrap();
        if header.flags.pdu_type != Self::TYPE {
            return Err(ParseError::InvalidType);
        }
        if header.length as usize > 37 {
            return Err(ParseError::InvalidLength);
        }

        let adv_address = Address::new_le(
            pdu[0..6].try_into().unwrap(),
            Header::header_bit_to_address_type(header.flags.tx_add),
        );

        let adv_data = &pdu[6..(header.length as usize)];

        Ok(Self {
            adv_address,
            adv_data,
        })
    }
}

/// Advertising Connectable Directed event (ADV_DIRECT_IND)
/// Used to connectable directed advertising events
///
///   |LSB                                               MSB|LSB    MSB|LSB      MSB|LSB                     MSB|
///   ┌──────────┬──────────┬─────────┬──────────┬──────────┬──────────┬──────────────────┬────────────────────┐
///   │ PDU Type:│ RFU:     │ ChSel:  │ TxAdd:   │ RxAdd:   │ Length   │ AdvAddr          │TargetAddr          │
///   │ 0b0001   │ -        │ (1 bit) │ (1 bit)  │ (1 bit)  │ (8 bits) │ (6 bytes)        │(6 bytes)           │
///   ├──────────┴──────────┴─────────┴──────────┴──────────┴──────────┼──────────────────┴────────────────────┤
///   │                        HEADER (2 bytes)                        │           Payload (12 bytes)          │
///   └────────────────────────────────────────────────────────────────┴───────────────────────────────────────┘
/// The TxAdd indicate the type of the advertiser address, either public or random.
/// The RxAdd in the advertising physical channel PDU header indicates whether the
/// target’s address in the TargetA field is public or random
/// The The ChSel field in is set if the advertiser supports the LE Channel Selection Algorithm #2 feature
///
/// The AdvA field contains the advertiser’s public or random device address
/// as indicated by TxAdd.
/// The TargetA field is the address of the device to which this PDU is addressed.
/// The TargetA field contains the target’s public or random device address as indicated by RxAdd.
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-3544231c-d808-9b6f-8f5a-d45c1c467d4e
#[derive(Debug, Clone, PartialEq, Eq, Format)]
pub struct AdvDirectInd {
    support_le_channel_selection: bool,
    adv_address: Address,
    target_address: Address,
}

impl AdvDirectInd {
    const TYPE: u8 = 0b0001;

    pub fn new(adv_address: Address, target_address: Address) -> Self {
        Self {
            support_le_channel_selection: false,
            adv_address,
            target_address,
        }
    }
}

impl<'a> Packet<'a, 14> for AdvDirectInd {
    type Error = ParseError;
    fn transmission_bytes(&self) -> [u8; 14] {
        let mut bytes = [0u8; 14];
        let header = Header::with_rxtx(
            self.adv_address.r#type,
            self.target_address.r#type,
            Self::TYPE,
            12, // length of pdu
        );

        bytes[..2].copy_from_slice(&header.transmission_bytes()); // write header
        bytes[2..8].copy_from_slice(&self.adv_address.transmission_bytes()); // write address
        bytes[8..14].copy_from_slice(&self.target_address.transmission_bytes()); // write address

        bytes
    }

    fn reception_parse(bytes: &'a [u8; 14]) -> Result<Self, Self::Error> {
        if bytes.len() < 8 {
            return Err(ParseError::InvalidLength);
        }

        let (header, pdu) = bytes.split_at(2);

        let header = Header::reception_parse(header.try_into().unwrap()).unwrap();
        if header.flags.pdu_type != Self::TYPE {
            return Err(ParseError::InvalidType);
        }
        if header.length as usize > 37 {
            return Err(ParseError::InvalidLength);
        }

        let (adv_address, target_address) = pdu.split_at(6);

        let adv_address = Address::new_le(
            adv_address.try_into().unwrap(),
            Header::header_bit_to_address_type(header.flags.tx_add),
        );

        let target_address = Address::new_le(
            target_address.try_into().unwrap(),
            Header::header_bit_to_address_type(header.flags.rx_add),
        );

        Ok(Self {
            support_le_channel_selection: header.flags.ch_sel,
            adv_address,
            target_address,
        })
    }
}

/// Advertising Non-Connectable and Non-Scannable Undirected event (ADV_NONCONN_IND)
/// Used to non-connectable and non-scannable undirected advertising events
///
///   |LSB                                               MSB|LSB    MSB|LSB      MSB|LSB                     MSB|
///   ┌──────────┬──────────┬─────────┬──────────┬──────────┬──────────┬────────────────────────────────────────┐
///   │ PDU Type:│ RFU:     │ ChSel:  │ TxAdd:   │ RxAdd:   │ Length   │ AdvAddr   │        AdvAddr             │
///   │ 0b0010   │ -        │ -       │ (1 bit)  │ -        │ (8 bits) │ (6 bytes) │       (0-31 bytes)         │
///   ├──────────┴──────────┴─────────┴──────────┴──────────┴──────────┼────────────────────────────────────────┤
///   │                        HEADER (2 bytes)                        │           Payload (6-37 bytes)         │
///   └────────────────────────────────────────────────────────────────┴────────────────────────────────────────┘
/// The TxAdd indicate the type of the advertiser address, either public or random.
///
/// The AdvAddr field contains the advertiser’s public or random device address as indicated by TxAdd.
/// The AdvData field, if not empty, contains Advertising Data from the advertiser’s Host.
///
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-3544231c-d808-9b6f-8f5a-d45c1c467d4e
#[derive(Debug, Clone, PartialEq, Eq, Format)]
pub struct AdvNonconnInd<'a> {
    adv_address: Address,
    adv_data: &'a [u8],
}

impl<'a> AdvNonconnInd<'a> {
    const TYPE: u8 = 0b0010;

    pub fn new(address: Address, data: &'a [u8]) -> Self {
        Self {
            adv_address: address,
            adv_data: data,
        }
    }
}

impl<'a> Packet<'a, 39> for AdvNonconnInd<'a> {
    type Error = ParseError;
    fn transmission_bytes(&self) -> [u8; 39] {
        let mut bytes = [0u8; 39];

        assert!(self.adv_data.len() <= 37);

        let header = Header::with_tx(
            self.adv_address.r#type,
            Self::TYPE,
            6 + self.adv_data.len() as u8, // total pdu length
        );

        bytes[..2].copy_from_slice(&header.transmission_bytes()); // write header
        bytes[2..8].copy_from_slice(&self.adv_address.transmission_bytes()); // write address
        bytes[8..(8 + self.adv_data.len())].copy_from_slice(self.adv_data); // write data

        bytes
    }

    fn reception_parse(bytes: &'a [u8; 39]) -> Result<Self, Self::Error> {
        if bytes.len() < 8 {
            return Err(ParseError::InvalidLength);
        }

        let (header, pdu) = bytes.split_at(2);

        let header = Header::reception_parse(header.try_into().unwrap()).unwrap();

        if header.flags.pdu_type != Self::TYPE {
            return Err(ParseError::InvalidType);
        }
        if header.length as usize > 37 {
            return Err(ParseError::InvalidLength);
        }

        let address = Address::new_le(
            pdu[0..6].try_into().unwrap(),
            Header::header_bit_to_address_type(header.flags.tx_add),
        );

        let data = &pdu[6..(header.length as usize)];

        Ok(Self {
            adv_address: address,
            adv_data: data,
        })
    }
}

/// Scannable Undirected event (ADV_SCAN_IND)
/// Used to non-connectable and non-scannable undirected advertising events
///
///   |LSB                                               MSB|LSB    MSB|LSB      MSB|LSB                     MSB|
///   ┌──────────┬──────────┬─────────┬──────────┬──────────┬──────────┬────────────────────────────────────────┐
///   │ PDU Type:│ RFU:     │ ChSel:  │ TxAdd:   │ RxAdd:   │ Length   │ AdvAddr   │        AdvAddr             │
///   │ 0b0110   │ -        │ -       │ (1 bit)  │ -        │ (8 bits) │ (6 bytes) │       (0-31 bytes)         │
///   ├──────────┴──────────┴─────────┴──────────┴──────────┴──────────┼────────────────────────────────────────┤
///   │                        HEADER (2 bytes)                        │           Payload (6-37 bytes)         │
///   └────────────────────────────────────────────────────────────────┴────────────────────────────────────────┘
/// The TxAdd indicate the type of the advertiser address, either public or random.
///
/// The AdvAddr field contains the advertiser’s public or random device address as indicated by TxAdd.
/// The AdvData field, if not empty, contains Advertising Data from the advertiser’s Host.
///
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-3544231c-d808-9b6f-8f5a-d45c1c467d4e
#[derive(Debug, Clone, PartialEq, Eq, Format)]
pub struct AdvScanInd<'a> {
    adv_address: Address,
    adv_data: &'a [u8],
}

impl<'a> AdvScanInd<'a> {
    const TYPE: u8 = 0b0110;
    pub fn new(address: Address, data: &'a [u8]) -> Self {
        Self {
            adv_address: address,
            adv_data: data,
        }
    }
}

impl<'a> Packet<'a, 39> for AdvScanInd<'a> {
    type Error = ParseError;
    fn transmission_bytes(&self) -> [u8; 39] {
        let mut bytes = [0u8; 39];

        assert!(self.adv_data.len() <= 37);

        let header = Header::with_tx(
            self.adv_address.r#type,
            Self::TYPE,
            6 + self.adv_data.len() as u8, // total pdu length
        );

        bytes[..2].copy_from_slice(&header.transmission_bytes()); // write header
        bytes[2..8].copy_from_slice(&self.adv_address.transmission_bytes()); // write address
        bytes[8..(8 + self.adv_data.len())].copy_from_slice(self.adv_data); // write data

        bytes
    }

    fn reception_parse(bytes: &'a [u8; 39]) -> Result<Self, Self::Error> {
        if bytes.len() < 8 {
            return Err(ParseError::InvalidLength);
        }

        let (header, pdu) = bytes.split_at(2);

        let header = Header::reception_parse(header.try_into().unwrap()).unwrap();

        if header.flags.pdu_type != Self::TYPE {
            return Err(ParseError::InvalidType);
        }
        if header.length as usize > 37 {
            return Err(ParseError::InvalidLength);
        }

        let address = Address::new_le(
            pdu[0..6].try_into().unwrap(),
            Header::header_bit_to_address_type(header.flags.tx_add),
        );

        let data = &pdu[6..(header.length as usize)];

        Ok(Self {
            adv_address: address,
            adv_data: data,
        })
    }
}

/// Scan Request (SCAN_REQ)
/// Used to connectable directed advertising events
///
///   |LSB                                               MSB|LSB    MSB|LSB      MSB|LSB                     MSB|
///   ┌──────────┬──────────┬─────────┬──────────┬──────────┬──────────┬──────────────────┬────────────────────┐
///   │ PDU Type:│ RFU:     │ ChSel:  │ TxAdd:   │ RxAdd:   │ Length   │ ScanAddr         │AdvAddr             │
///   │ 0b0001   │ -        │ (1 bit) │ (1 bit)  │ (1 bit)  │ (8 bits) │ (6 bytes)        │(6 bytes)           │
///   ├──────────┴──────────┴─────────┴──────────┴──────────┴──────────┼──────────────────┴────────────────────┤
///   │                        HEADER (2 bytes)                        │           Payload (12 bytes)          │
///   └────────────────────────────────────────────────────────────────┴───────────────────────────────────────┘
/// The TxAdd indicate the type of the scanner’s address in the ScanA field, either public or random.
/// The RxAdd indicates whether the advertiser’s address in the AdvA field is public or random
/// The The ChSel field in is set if the advertiser supports the LE Channel Selection Algorithm #2 feature
///
/// The ScanA field contains the scanner’s public or random device address as indicated by TxAdd
/// The TargetA field is the address of the device to which this PDU is addressed.
/// The AdvA field is the address of the device to which this PDU is addressed
/// The AdvA field shall contain the advertiser’s public or random device address as indicated by RxAdd.
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-3544231c-d808-9b6f-8f5a-d45c1c467d4e
#[derive(Debug, Clone, PartialEq, Eq, Format)]
pub struct ScanReq {
    scan_address: Address,
    adv_address: Address,
}

impl ScanReq {
    const TYPE: u8 = 0b0011;
    pub fn new(scan_address: Address, adv_address: Address) -> Self {
        Self {
            scan_address,
            adv_address,
        }
    }
}

impl<'a> Packet<'a, 14> for ScanReq {
    type Error = ParseError;
    fn transmission_bytes(&self) -> [u8; 14] {
        let mut bytes = [0u8; 14];
        let header = Header::with_rxtx(
            self.scan_address.r#type,
            self.adv_address.r#type,
            Self::TYPE,
            12, // length of pdu
        );

        bytes[..2].copy_from_slice(&header.transmission_bytes()); // write header
        bytes[2..8].copy_from_slice(&self.scan_address.transmission_bytes()); // write scan address
        bytes[8..14].copy_from_slice(&self.adv_address.transmission_bytes()); // write adv address

        bytes
    }

    fn reception_parse(bytes: &'a [u8; 14]) -> Result<Self, Self::Error> {
        if bytes.len() < 8 {
            return Err(ParseError::InvalidLength);
        }

        let (header, pdu) = bytes.split_at(2);

        let header = Header::reception_parse(header.try_into().unwrap()).unwrap();
        if header.flags.pdu_type != Self::TYPE {
            return Err(ParseError::InvalidType);
        }
        if header.length as usize > 37 {
            return Err(ParseError::InvalidLength);
        }

        let (scan_address, adv_address) = pdu.split_at(6);

        let scan_address = Address::new_le(
            scan_address.try_into().unwrap(),
            Header::header_bit_to_address_type(header.flags.tx_add),
        );

        let adv_address = Address::new_le(
            adv_address.try_into().unwrap(),
            Header::header_bit_to_address_type(header.flags.tx_add),
        );

        Ok(Self {
            scan_address,
            adv_address,
        })
    }
}

/// Advertising Scan Indication (SCAN_RSP)
/// Used to non-connectable and non-scannable undirected advertising events
///
///   |LSB                                               MSB|LSB    MSB|LSB      MSB|LSB                     MSB|
///   ┌──────────┬──────────┬─────────┬──────────┬──────────┬──────────┬────────────────────────────────────────┐
///   │ PDU Type:│ RFU:     │ ChSel:  │ TxAdd:   │ RxAdd:   │ Length   │ AdvAddr   │        AdvAddr             │
///   │ 0b0110   │ -        │ -       │ (1 bit)  │ -        │ (8 bits) │ (6 bytes) │       (0-31 bytes)         │
///   ├──────────┴──────────┴─────────┴──────────┴──────────┴──────────┼────────────────────────────────────────┤
///   │                        HEADER (2 bytes)                        │           Payload (6-37 bytes)         │
///   └────────────────────────────────────────────────────────────────┴────────────────────────────────────────┘
/// The TxAdd indicate the type of the advertiser address, either public or random.
///
/// The AdvAddr field contains the advertiser’s public or random device address as indicated by TxAdd.
/// The AdvData field, if not empty, contains Advertising Data from the advertiser’s Host.
///
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-3544231c-d808-9b6f-8f5a-d45c1c467d4e
#[derive(Debug, Clone, PartialEq, Eq, Format)]
pub struct ScanRsp<'a> {
    adv_address: Address,
    adv_data: &'a [u8],
}

impl<'a> ScanRsp<'a> {
    const TYPE: u8 = 0b0100;
    pub fn new(address: Address, data: &'a [u8]) -> Self {
        Self {
            adv_address: address,
            adv_data: data,
        }
    }
}

impl<'a> Packet<'a, 39> for ScanRsp<'a> {
    type Error = ParseError;
    fn transmission_bytes(&self) -> [u8; 39] {
        let mut bytes = [0u8; 39];

        assert!(self.adv_data.len() <= 37);

        let header = Header::with_tx(
            self.adv_address.r#type,
            Self::TYPE,
            6 + self.adv_data.len() as u8, // total pdu length
        );

        bytes[..2].copy_from_slice(&header.transmission_bytes()); // write header
        bytes[2..8].copy_from_slice(&self.adv_address.transmission_bytes()); // write address
        bytes[8..(8 + self.adv_data.len())].copy_from_slice(self.adv_data); // write data

        bytes
    }

    fn reception_parse(bytes: &'a [u8; 39]) -> Result<Self, Self::Error> {
        if bytes.len() < 8 {
            return Err(ParseError::InvalidLength);
        }

        let (header, pdu) = bytes.split_at(2);

        let header = Header::reception_parse(header.try_into().unwrap()).unwrap();

        if header.flags.pdu_type != Self::TYPE {
            return Err(ParseError::InvalidType);
        }
        if header.length as usize > 37 {
            return Err(ParseError::InvalidLength);
        }

        let address = Address::new_le(
            pdu[0..6].try_into().unwrap(),
            Header::header_bit_to_address_type(header.flags.tx_add),
        );

        let data = &pdu[6..(header.length as usize)];

        Ok(Self {
            adv_address: address,
            adv_data: data,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // ADV_IND
    #[test]
    fn adv_serialize() {
        let actual = AdvInd {
            adv_address: Address::new_random(0xffe1e8d0dc27),
            adv_data: &[0x01, 0x02, 0x03],
        };

        let expected = [
            0x40u8, // ADV_IND, Random address,
            9,      // Length of payload
            0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
            0x01, 0x02, 0x03, // Data
        ];

        assert_eq!(actual.transmission_bytes()[..(expected.len())], expected);
    }

    #[test]
    fn adv_complementary() {
        let packet = AdvInd {
            adv_address: Address::new_random(0xffe1e8d0dc27),
            adv_data: &[0x01, 0x02, 0x03],
        };

        let bytes = packet.transmission_bytes();
        let actual = AdvInd::reception_parse(&bytes).unwrap();

        assert_eq!(packet, actual);
    }

    // ADV_DIRECT_IND
    #[test]
    fn direct_serialize() {
        let actual = AdvDirectInd {
            support_le_channel_selection: false,
            adv_address: Address::new_random(0xffe1e8d0dc27),
            target_address: Address::new_random(0xffe1e8d0dc27),
        };

        let expected = [
            0xC1u8, // ADV_DIRECT_IND, Random address, Random address
            12,     // Length of payload
            0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
            0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
        ];

        assert_eq!(actual.transmission_bytes()[..(expected.len())], expected);
    }

    #[test]
    fn direct_complementary() {
        let packet = AdvDirectInd {
            support_le_channel_selection: false,
            adv_address: Address::new_random(0xffe1e8d0dc27),
            target_address: Address::new_random(0xffe1e8d0dc27),
        };

        let bytes = packet.transmission_bytes();
        let actual = AdvDirectInd::reception_parse(&bytes).unwrap();

        assert_eq!(packet, actual);
    }

    // ADV_NONCONN_IND
    #[test]
    fn non_connectable_serialize() {
        let actual = AdvNonconnInd {
            adv_address: Address::new_random(0xffe1e8d0dc27),
            adv_data: &[0x01, 0x02, 0x03],
        };

        let expected = [
            0x42u8, // ADV_NONCONN_IND, Random address,
            9,      // Length of payload
            0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
            0x01, 0x02, 0x03, // Data
        ];

        assert_eq!(actual.transmission_bytes()[..(expected.len())], expected);
    }

    #[test]
    fn non_connectable_complementary() {
        let packet = AdvNonconnInd {
            adv_address: Address::new_random(0xffe1e8d0dc27),
            adv_data: &[0x01, 0x02, 0x03],
        };

        let bytes = packet.transmission_bytes();
        let actual = AdvNonconnInd::reception_parse(&bytes).unwrap();

        assert_eq!(packet, actual);
    }

    // ADV_SCAN_IND
    #[test]
    fn scan_serialize() {
        let actual = AdvScanInd {
            adv_address: Address::new_random(0xffe1e8d0dc27),
            adv_data: &[0x01, 0x02, 0x03],
        };

        let expected = [
            0x46u8, // ADV_SCAN_ID, Random address,
            9,      // Length of payload
            0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
            0x01, 0x02, 0x03, // Data
        ];

        assert_eq!(actual.transmission_bytes()[..(expected.len())], expected);
    }

    #[test]
    fn scan_complementary() {
        let packet = AdvScanInd {
            adv_address: Address::new_random(0xffe1e8d0dc27),
            adv_data: &[0x01, 0x02, 0x03],
        };

        let bytes = packet.transmission_bytes();
        let actual = AdvScanInd::reception_parse(&bytes).unwrap();

        assert_eq!(packet, actual);
    }

    // SCAN_REQ
    #[test]
    fn scan_request_serialize() {
        let actual = ScanReq {
            scan_address: Address::new_random(0xffe1e8d0dc27),
            adv_address: Address::new_random(0xffe1e8d0dc27),
        };

        let expected = [
            0xC3u8, // ADV_DIRECT_IND, Random address,
            12,     // Length of payload
            0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
            0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
        ];

        assert_eq!(actual.transmission_bytes()[..(expected.len())], expected);
    }

    #[test]
    fn scan_request_complementary() {
        let packet = ScanReq {
            scan_address: Address::new_random(0xffe1e8d0dc27),
            adv_address: Address::new_random(0xffe1e8d0dc27),
        };

        let bytes = packet.transmission_bytes();
        let actual = ScanReq::reception_parse(&bytes).unwrap();

        assert_eq!(packet, actual);
    }

    // SCAN_RSP
    #[test]
    fn scan_response_serialize() {
        let actual = ScanRsp {
            adv_address: Address::new_random(0xffe1e8d0dc27),
            adv_data: &[0x01, 0x02, 0x03],
        };

        let expected = [
            0x44u8, // ADV_NONCONN_IND, Random address,
            9,      // Length of payload
            0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
            0x01, 0x02, 0x03, // Data
        ];

        assert_eq!(actual.transmission_bytes()[..(expected.len())], expected);
    }

    #[test]
    fn scan_response_complementary() {
        let packet = ScanRsp {
            adv_address: Address::new_random(0xffe1e8d0dc27),
            adv_data: &[0x01, 0x02, 0x03],
        };

        let bytes = packet.transmission_bytes();
        let actual = ScanRsp::reception_parse(&bytes).unwrap();

        assert_eq!(packet, actual);
    }
}
