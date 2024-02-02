use crate::ll::Address;
use defmt::Format;

use super::Header;

pub use self::direct_ind::*;
pub use self::ind::*;
pub use self::nonconn_ind::*;
pub use self::scan_ind::*;
pub use self::scan_req::*;
pub use self::scan_rsp::*;

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

pub fn parse<'a>(bytes: &'a [u8]) -> Result<AdvPdu<'a>, ParseError> {
    if let Ok(packet) = bytes[0..14]
        .try_into()
        .map_err(|_| ParseError::InvalidType)
        .and_then(AdvDirectInd::parse)
    {
        Ok(packet.into())
    } else if let Ok(packet) = bytes[0..39]
        .try_into()
        .map_err(|_| ParseError::InvalidType)
        .and_then(AdvInd::parse)
    {
        Ok(packet.into())
    } else if let Ok(packet) = bytes[0..39]
        .try_into()
        .map_err(|_| ParseError::InvalidType)
        .and_then(AdvNonconnInd::parse)
    {
        Ok(packet.into())
    } else if let Ok(packet) = bytes[0..39]
        .try_into()
        .map_err(|_| ParseError::InvalidType)
        .and_then(AdvScanInd::parse)
    {
        Ok(packet.into())
    } else if let Ok(packet) = bytes[0..14]
        .try_into()
        .map_err(|_| ParseError::InvalidType)
        .and_then(ScanReq::parse)
    {
        Ok(packet.into())
    } else if let Ok(packet) = bytes[0..39]
        .try_into()
        .map_err(|_| ParseError::InvalidType)
        .and_then(ScanRsp::parse)
    {
        Ok(packet.into())
    } else {
        Err(ParseError::InvalidType)
    }
}

mod ind {
    use super::*;

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
        pub const TYPE: u8 = 0b0000;

        pub fn new(adv_address: Address, adv_data: &'a [u8]) -> Self {
            Self {
                adv_address,
                adv_data,
            }
        }

        pub fn bytes(&self) -> [u8; 39] {
            let mut bytes = [0u8; 39];

            let header = Header::with_tx(
                self.adv_address.r#type,
                Self::TYPE,
                6 + self.adv_data.len() as u8, // total pdu length
            );

            bytes[..2].copy_from_slice(&header.bytes()); // write header
            bytes[2..8].copy_from_slice(&self.adv_address.bytes()); // write address
            bytes[8..(8 + self.adv_data.len())].copy_from_slice(self.adv_data); // write data

            bytes
        }

        pub fn parse(bytes: &'a [u8; 39]) -> Result<Self, ParseError> {
            if bytes.len() < 8 {
                return Err(ParseError::InvalidLength);
            }

            let (header, pdu) = bytes.split_at(2);

            let header = Header::parse(header.try_into().unwrap());
            if header.flags.pdu_type != Self::TYPE {
                return Err(ParseError::InvalidType);
            }
            if header.length as usize > 37 {
                return Err(ParseError::InvalidLength);
            }

            let adv_address = Address::new_le(
                pdu[0..6].try_into().unwrap(),
                Header::bit_to_address_type(header.flags.tx_add),
            );

            let adv_data = &pdu[6..(header.length as usize)];

            Ok(Self {
                adv_address,
                adv_data,
            })
        }
    }

    impl<'a> From<AdvInd<'a>> for AdvPdu<'a> {
        fn from(adv_ind: AdvInd<'a>) -> Self {
            Self::AdvInd(adv_ind)
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

            assert_eq!(actual.bytes()[..(expected.len())], expected);
        }

        #[test]
        fn adv_complementary() {
            let packet = AdvInd {
                adv_address: Address::new_random(0xffe1e8d0dc27),
                adv_data: &[0x01, 0x02, 0x03],
            };

            let bytes = packet.bytes();
            let actual = AdvInd::parse(&bytes).unwrap();

            assert_eq!(packet, actual);
        }
    }
}

mod direct_ind {
    use super::*;

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
        pub const TYPE: u8 = 0b0001;

        pub fn new(adv_address: Address, target_address: Address) -> Self {
            Self {
                support_le_channel_selection: false,
                adv_address,
                target_address,
            }
        }

        pub fn bytes(&self) -> [u8; 14] {
            let mut bytes = [0u8; 14];
            let header = Header::with_rxtx(
                self.adv_address.r#type,
                self.target_address.r#type,
                Self::TYPE,
                12, // length of pdu
            );

            bytes[..2].copy_from_slice(&header.bytes()); // write header
            bytes[2..8].copy_from_slice(&self.adv_address.bytes()); // write address
            bytes[8..14].copy_from_slice(&self.target_address.bytes()); // write address

            bytes
        }

        pub fn parse(bytes: &[u8; 14]) -> Result<Self, ParseError> {
            if bytes.len() < 8 {
                return Err(ParseError::InvalidLength);
            }

            let (header, pdu) = bytes.split_at(2);

            let header = Header::parse(header.try_into().unwrap());
            if header.flags.pdu_type != Self::TYPE {
                return Err(ParseError::InvalidType);
            }
            if header.length as usize > 37 {
                return Err(ParseError::InvalidLength);
            }

            let (adv_address, target_address) = pdu.split_at(6);

            let adv_address = Address::new_le(
                adv_address.try_into().unwrap(),
                Header::bit_to_address_type(header.flags.tx_add),
            );

            let target_address = Address::new_le(
                target_address.try_into().unwrap(),
                Header::bit_to_address_type(header.flags.rx_add),
            );

            Ok(Self {
                support_le_channel_selection: header.flags.ch_sel,
                adv_address,
                target_address,
            })
        }
    }

    impl<'a> From<AdvDirectInd> for AdvPdu<'a> {
        fn from(adv_direct_ind: AdvDirectInd) -> Self {
            Self::AdvDirectInd(adv_direct_ind)
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

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

            assert_eq!(actual.bytes()[..(expected.len())], expected);
        }

        #[test]
        fn direct_complementary() {
            let packet = AdvDirectInd {
                support_le_channel_selection: false,
                adv_address: Address::new_random(0xffe1e8d0dc27),
                target_address: Address::new_random(0xffe1e8d0dc27),
            };

            let bytes = packet.bytes();
            let actual = AdvDirectInd::parse(&bytes).unwrap();

            assert_eq!(packet, actual);
        }
    }
}

mod nonconn_ind {
    use super::*;
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
        pub const TYPE: u8 = 0b0010;

        pub fn new(address: Address, data: &'a [u8]) -> Self {
            Self {
                adv_address: address,
                adv_data: data,
            }
        }

        pub fn bytes(&self) -> [u8; 39] {
            let mut bytes = [0u8; 39];

            assert!(self.adv_data.len() <= 37);

            let header = Header::with_tx(
                self.adv_address.r#type,
                Self::TYPE,
                6 + self.adv_data.len() as u8, // total pdu length
            );

            bytes[..2].copy_from_slice(&header.bytes()); // write header
            bytes[2..8].copy_from_slice(&self.adv_address.bytes()); // write address
            bytes[8..(8 + self.adv_data.len())].copy_from_slice(self.adv_data); // write data

            bytes
        }

        pub fn parse(bytes: &'a [u8; 39]) -> Result<Self, ParseError> {
            if bytes.len() < 8 {
                return Err(ParseError::InvalidLength);
            }

            let (header, pdu) = bytes.split_at(2);

            let header = Header::parse(header.try_into().unwrap());

            if header.flags.pdu_type != Self::TYPE {
                return Err(ParseError::InvalidType);
            }
            if header.length as usize > 37 {
                return Err(ParseError::InvalidLength);
            }

            let address = Address::new_le(
                pdu[0..6].try_into().unwrap(),
                Header::bit_to_address_type(header.flags.tx_add),
            );

            let data = &pdu[6..(header.length as usize)];

            Ok(Self {
                adv_address: address,
                adv_data: data,
            })
        }
    }

    impl<'a> From<AdvNonconnInd<'a>> for AdvPdu<'a> {
        fn from(adv_nonconn_ind: AdvNonconnInd<'a>) -> Self {
            Self::AdvNonconnInd(adv_nonconn_ind)
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
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

            assert_eq!(actual.bytes()[..(expected.len())], expected);
        }

        #[test]
        fn non_connectable_complementary() {
            let packet = AdvNonconnInd {
                adv_address: Address::new_random(0xffe1e8d0dc27),
                adv_data: &[0x01, 0x02, 0x03],
            };

            let bytes = packet.bytes();
            let actual = AdvNonconnInd::parse(&bytes).unwrap();

            assert_eq!(packet, actual);
        }
    }
}

mod scan_ind {
    use super::*;
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
        pub const TYPE: u8 = 0b0110;
        pub fn new(address: Address, data: &'a [u8]) -> Self {
            Self {
                adv_address: address,
                adv_data: data,
            }
        }

        pub fn bytes(&self) -> [u8; 39] {
            let mut bytes = [0u8; 39];

            assert!(self.adv_data.len() <= 37);

            let header = Header::with_tx(
                self.adv_address.r#type,
                Self::TYPE,
                6 + self.adv_data.len() as u8, // total pdu length
            );

            bytes[..2].copy_from_slice(&header.bytes()); // write header
            bytes[2..8].copy_from_slice(&self.adv_address.bytes()); // write address
            bytes[8..(8 + self.adv_data.len())].copy_from_slice(self.adv_data); // write data

            bytes
        }

        pub fn parse(bytes: &'a [u8; 39]) -> Result<Self, ParseError> {
            if bytes.len() < 8 {
                return Err(ParseError::InvalidLength);
            }

            let (header, pdu) = bytes.split_at(2);

            let header = Header::parse(header.try_into().unwrap());

            if header.flags.pdu_type != Self::TYPE {
                return Err(ParseError::InvalidType);
            }
            if header.length as usize > 37 {
                return Err(ParseError::InvalidLength);
            }

            let address = Address::new_le(
                pdu[0..6].try_into().unwrap(),
                Header::bit_to_address_type(header.flags.tx_add),
            );

            let data = &pdu[6..(header.length as usize)];

            Ok(Self {
                adv_address: address,
                adv_data: data,
            })
        }
    }

    impl<'a> From<AdvScanInd<'a>> for AdvPdu<'a> {
        fn from(adv_scan_ind: AdvScanInd<'a>) -> Self {
            Self::AdvScanInd(adv_scan_ind)
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

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

            assert_eq!(actual.bytes()[..(expected.len())], expected);
        }

        #[test]
        fn scan_complementary() {
            let packet = AdvScanInd {
                adv_address: Address::new_random(0xffe1e8d0dc27),
                adv_data: &[0x01, 0x02, 0x03],
            };

            let bytes = packet.bytes();
            let actual = AdvScanInd::parse(&bytes).unwrap();

            assert_eq!(packet, actual);
        }
    }
}

mod scan_req {
    use super::*;
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
        pub const TYPE: u8 = 0b0011;
        pub fn new(scan_address: Address, adv_address: Address) -> Self {
            Self {
                scan_address,
                adv_address,
            }
        }

        pub fn bytes(&self) -> [u8; 14] {
            let mut bytes = [0u8; 14];
            let header = Header::with_rxtx(
                self.scan_address.r#type,
                self.adv_address.r#type,
                Self::TYPE,
                12, // length of pdu
            );

            bytes[..2].copy_from_slice(&header.bytes()); // write header
            bytes[2..8].copy_from_slice(&self.scan_address.bytes()); // write scan address
            bytes[8..14].copy_from_slice(&self.adv_address.bytes()); // write adv address

            bytes
        }

        pub fn parse(bytes: &[u8; 14]) -> Result<Self, ParseError> {
            if bytes.len() < 8 {
                return Err(ParseError::InvalidLength);
            }

            let (header, pdu) = bytes.split_at(2);

            let header = Header::parse(header.try_into().unwrap());
            if header.flags.pdu_type != Self::TYPE {
                return Err(ParseError::InvalidType);
            }
            if header.length as usize > 37 {
                return Err(ParseError::InvalidLength);
            }

            let (scan_address, adv_address) = pdu.split_at(6);

            let scan_address = Address::new_le(
                scan_address.try_into().unwrap(),
                Header::bit_to_address_type(header.flags.tx_add),
            );

            let adv_address = Address::new_le(
                adv_address.try_into().unwrap(),
                Header::bit_to_address_type(header.flags.tx_add),
            );

            Ok(Self {
                scan_address,
                adv_address,
            })
        }
    }

    impl<'a> From<ScanReq> for AdvPdu<'a> {
        fn from(scan_req: ScanReq) -> Self {
            Self::ScanReq(scan_req)
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
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

            assert_eq!(actual.bytes()[..(expected.len())], expected);
        }

        #[test]
        fn scan_request_complementary() {
            let packet = ScanReq {
                scan_address: Address::new_random(0xffe1e8d0dc27),
                adv_address: Address::new_random(0xffe1e8d0dc27),
            };

            let bytes = packet.bytes();
            let actual = ScanReq::parse(&bytes).unwrap();

            assert_eq!(packet, actual);
        }
    }
}

mod scan_rsp {
    use super::*;
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
        pub const TYPE: u8 = 0b0100;
        pub fn new(address: Address, data: &'a [u8]) -> Self {
            Self {
                adv_address: address,
                adv_data: data,
            }
        }

        pub fn bytes(&self) -> [u8; 39] {
            let mut bytes = [0u8; 39];

            assert!(self.adv_data.len() <= 37);

            let header = Header::with_tx(
                self.adv_address.r#type,
                Self::TYPE,
                6 + self.adv_data.len() as u8, // total pdu length
            );

            bytes[..2].copy_from_slice(&header.bytes()); // write header
            bytes[2..8].copy_from_slice(&self.adv_address.bytes()); // write address
            bytes[8..(8 + self.adv_data.len())].copy_from_slice(self.adv_data); // write data

            bytes
        }

        pub fn parse(bytes: &'a [u8; 39]) -> Result<Self, ParseError> {
            if bytes.len() < 8 {
                return Err(ParseError::InvalidLength);
            }

            let (header, pdu) = bytes.split_at(2);

            let header = Header::parse(header.try_into().unwrap());

            if header.flags.pdu_type != Self::TYPE {
                return Err(ParseError::InvalidType);
            }
            if header.length as usize > 37 {
                return Err(ParseError::InvalidLength);
            }

            let address = Address::new_le(
                pdu[0..6].try_into().unwrap(),
                Header::bit_to_address_type(header.flags.tx_add),
            );

            let data = &pdu[6..(header.length as usize)];

            Ok(Self {
                adv_address: address,
                adv_data: data,
            })
        }
    }

    impl<'a> From<ScanRsp<'a>> for AdvPdu<'a> {
        fn from(scan_rsp: ScanRsp<'a>) -> Self {
            Self::ScanRsp(scan_rsp)
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

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

            assert_eq!(actual.bytes()[..(expected.len())], expected);
        }

        #[test]
        fn scan_response_complementary() {
            let packet = ScanRsp {
                adv_address: Address::new_random(0xffe1e8d0dc27),
                adv_data: &[0x01, 0x02, 0x03],
            };

            let bytes = packet.bytes();
            let actual = ScanRsp::parse(&bytes).unwrap();

            assert_eq!(packet, actual);
        }
    }
}
