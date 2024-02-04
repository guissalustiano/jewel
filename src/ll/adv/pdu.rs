//! Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-8909a735-7143-2804-ce68-c535a4fc011d

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
    AdvInd(AdvInd<'a>) = AdvInd::PDU_TYPE,
    AdvDirectInd(AdvDirectInd) = AdvDirectInd::PDU_TYPE,
    AdvNonconnInd(AdvNonconnInd<'a>) = AdvNonconnInd::PDU_TYPE,
    AdvScanInd(AdvScanInd<'a>) = AdvScanInd::PDU_TYPE,
    ScanReq(ScanReq) = ScanReq::PDU_TYPE,
    ScanRsp(ScanRsp<'a>) = ScanRsp::PDU_TYPE,
}

pub fn parse(bytes: &[u8]) -> Result<AdvPdu<'_>, ParseError> {
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

/// Generalize ADV_DIRECT_IND and SCAN_REQ
/// Used to connectable directed advertising events
///
///   |LSB                                               MSB|LSB    MSB|LSB      MSB|LSB                     MSB|
///   ┌──────────┬──────────┬─────────┬──────────┬──────────┬──────────┬──────────────────┬────────────────────┐
///   │ PDU Type:│ RFU:     │ ChSel:  │ TxAdd:   │ RxAdd:   │ Length   │ first()          │second()            │
///   │ 0b0001   │ -        │ -       │ (1 bit)  │ (1 bit)  │ (8 bits) │ (6 bytes)        │(6 bytes)           │
///   ├──────────┴──────────┴─────────┴──────────┴──────────┴──────────┼──────────────────┴────────────────────┤
///   │                        HEADER (2 bytes)                        │           Payload (12 bytes)          │
///   └────────────────────────────────────────────────────────────────┴───────────────────────────────────────┘
/// The TxAdd and RxAdd indicate the type of the advertiser address, either public or random.
pub trait TwoAddress: Sized {
    const PDU_TYPE: u8;

    fn first(&self) -> &Address;
    fn second(&self) -> &Address;
    fn from_addresses(first: Address, second: Address) -> Self;

    fn bytes(&self, dest: &mut [u8]) -> usize {
        let payload_len = 12;
        let total_len = 2 + payload_len;
        assert!(dest.len() >= total_len);

        let header = Header::with_rxtx(
            self.first().r#type,
            self.second().r#type,
            Self::PDU_TYPE,
            payload_len as u8,
        );

        dest[..2].copy_from_slice(&header.bytes()); // write header
        dest[2..8].copy_from_slice(&self.first().bytes()); // write address 1
        dest[8..14].copy_from_slice(&self.second().bytes()); // write address 2

        total_len
    }

    fn parse(bytes: &[u8]) -> Result<Self, ParseError> {
        if bytes.len() < 8 {
            return Err(ParseError::InvalidLength);
        }

        let (header, pdu) = bytes.split_at(2);

        let header = Header::parse(header.try_into().unwrap());
        if header.flags.pdu_type != Self::PDU_TYPE {
            return Err(ParseError::InvalidType);
        }
        if header.length as usize > 37 {
            return Err(ParseError::InvalidLength);
        }

        let (first, second) = pdu.split_at(6);

        let first = Address::new_le(
            first.try_into().unwrap(),
            Header::bit_to_address_type(header.flags.tx_add),
        );

        let second = Address::new_le(
            second.try_into().unwrap(),
            Header::bit_to_address_type(header.flags.rx_add),
        );

        Ok(Self::from_addresses(first, second))
    }
}

/// Generalize ADV_IND, ADV_NONCONN_IND, ADV_SCAN_IND and SCAN_RSP
///
///   |LSB                                               MSB|LSB    MSB|LSB      MSB|LSB                     MSB|
///   ┌──────────┬──────────┬─────────┬──────────┬──────────┬──────────┬───────────┬────────────────────────────┐
///   │ PDU Type:│ RFU:     │ ChSel:  │ TxAdd:   │ RxAdd:   │ Length   │ address() │        data()              │
///   │ 0b0000   │ -        │ -       │ (1 bit)  │ -        │ (8 bits) │ (6 bytes) │       (0-31 bytes)         │
///   ├──────────┴──────────┴─────────┴──────────┴──────────┴──────────┼───────────┴────────────────────────────┤
///   │                        HEADER (2 bytes)                        │           Payload (6-37 bytes)         │
///   └────────────────────────────────────────────────────────────────┴────────────────────────────────────────┘
/// The TxAdd indicate the type of the advertiser address, either public or random.
///
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-3544231c-d808-9b6f-8f5a-d45c1c467d4e
pub trait AddressAndData<'a>: Sized {
    const PDU_TYPE: u8;

    fn address(&self) -> &Address;
    fn data(&self) -> &[u8];
    fn from_address_and_data(address: Address, data: &'a [u8]) -> Self;

    fn bytes(&self, dest: &mut [u8]) -> usize {
        let payload_len = 6 + self.data().len();
        let total_len = 2 + payload_len;
        assert!(dest.len() >= total_len);

        let header = Header::with_tx(self.address().r#type, Self::PDU_TYPE, payload_len as u8);

        dest[..2].copy_from_slice(&header.bytes()); // write header
        dest[2..8].copy_from_slice(&self.address().bytes()); // write address
        dest[8..total_len].copy_from_slice(self.data()); // write data

        total_len
    }

    fn parse(bytes: &'a [u8]) -> Result<Self, ParseError> {
        if bytes.len() < 8 {
            return Err(ParseError::InvalidLength);
        }

        let (header, pdu) = bytes.split_at(2);

        let header = Header::parse(header.try_into().unwrap());
        if header.flags.pdu_type != Self::PDU_TYPE {
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

        Ok(Self::from_address_and_data(address, data))
    }
}

mod ind {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Format)]
    pub struct AdvInd<'a> {
        // It contains the advertiser address.
        adv_address: Address,

        // It contains Advertising Data from the advertiser’s Host.
        // It can be empty.
        adv_data: &'a [u8],
    }

    impl<'a> AdvInd<'a> {
        pub fn new(adv_address: Address, adv_data: &'a [u8]) -> Self {
            Self {
                adv_address,
                adv_data,
            }
        }
    }

    impl<'a> AddressAndData<'a> for AdvInd<'a> {
        const PDU_TYPE: u8 = 0b0000;

        fn address(&self) -> &Address {
            &self.adv_address
        }

        fn data(&self) -> &[u8] {
            self.adv_data
        }

        fn from_address_and_data(address: Address, data: &'a [u8]) -> Self {
            Self {
                adv_address: address,
                adv_data: data,
            }
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

            let mut bytes = [0u8; 39];
            let len = actual.bytes(&mut bytes);
            assert_eq!(bytes[..len], expected);
        }

        #[test]
        fn adv_complementary() {
            let bytes = [
                0x40u8, // ADV_IND, Random address,
                9,      // Length of payload
                0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
                0x01, 0x02, 0x03, // Data
            ];

            let expected = AdvInd {
                adv_address: Address::new_random(0xffe1e8d0dc27),
                adv_data: &[0x01, 0x02, 0x03],
            };

            assert_eq!(AdvInd::parse(&bytes).unwrap(), expected);
        }
    }
}

mod direct_ind {
    use super::*;

    // TODO: implement support_le_channel_selection for BLE 5.1
    #[derive(Debug, Clone, PartialEq, Eq, Format)]
    pub struct AdvDirectInd {
        /// It contains the advertiser address.
        adv_address: Address,

        /// It contains the target address, the address of the device to which this PDU is addressed.
        target_address: Address,
    }

    impl AdvDirectInd {
        pub fn new(adv_address: Address, target_address: Address) -> Self {
            Self {
                adv_address,
                target_address,
            }
        }
    }

    impl TwoAddress for AdvDirectInd {
        const PDU_TYPE: u8 = 0b0001;

        fn first(&self) -> &Address {
            &self.adv_address
        }

        fn second(&self) -> &Address {
            &self.target_address
        }

        fn from_addresses(first: Address, second: Address) -> Self {
            Self {
                adv_address: first,
                target_address: second,
            }
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
                adv_address: Address::new_random(0xffe1e8d0dc27),
                target_address: Address::new_random(0xffe1e8d0dc27),
            };

            let expected = [
                0xC1u8, // ADV_DIRECT_IND, Random address, Random address
                12,     // Length of payload
                0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
                0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
            ];

            let mut bytes = [0u8; 14];
            let len = actual.bytes(&mut bytes);
            assert_eq!(bytes[..len], expected);
        }

        #[test]
        fn direct_complementary() {
            let expected = AdvDirectInd {
                adv_address: Address::new_random(0xffe1e8d0dc27),
                target_address: Address::new_random(0xffe1e8d0dc27),
            };
            let actual = [
                0xC1u8, // ADV_DIRECT_IND, Random address, Random address
                12,     // Length of payload
                0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
                0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
            ];

            assert_eq!(AdvDirectInd::parse(&actual).unwrap(), expected);
        }
    }
}

mod nonconn_ind {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Format)]
    pub struct AdvNonconnInd<'a> {
        /// It contains the advertiser address.
        adv_address: Address,

        /// It contains Advertising Data from the advertiser’s Host.
        adv_data: &'a [u8],
    }

    impl<'a> AdvNonconnInd<'a> {
        pub const TYPE: u8 = 0b0010;
        pub const PACKET_MAX_SIZE: usize = 39;

        pub fn new(address: Address, data: &'a [u8]) -> Self {
            Self {
                adv_address: address,
                adv_data: data,
            }
        }
    }

    impl<'a> AddressAndData<'a> for AdvNonconnInd<'a> {
        const PDU_TYPE: u8 = Self::TYPE;

        fn address(&self) -> &Address {
            &self.adv_address
        }

        fn data(&self) -> &[u8] {
            self.adv_data
        }

        fn from_address_and_data(address: Address, data: &'a [u8]) -> Self {
            Self {
                adv_address: address,
                adv_data: data,
            }
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

            let mut bytes = [0u8; 39];
            let len = actual.bytes(&mut bytes);
            assert_eq!(bytes[..len], expected);
        }

        #[test]
        fn non_connectable_complementary() {
            let expected = AdvNonconnInd {
                adv_address: Address::new_random(0xffe1e8d0dc27),
                adv_data: &[0x01, 0x02, 0x03],
            };

            let actual = [
                0x42u8, // ADV_NONCONN_IND, Random address,
                9,      // Length of payload
                0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
                0x01, 0x02, 0x03, // Data
            ];

            assert_eq!(AdvNonconnInd::parse(&actual).unwrap(), expected);
        }
    }
}

mod scan_ind {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Format)]
    pub struct AdvScanInd<'a> {
        adv_address: Address,
        adv_data: &'a [u8],
    }

    impl<'a> AdvScanInd<'a> {
        pub fn new(address: Address, data: &'a [u8]) -> Self {
            Self {
                adv_address: address,
                adv_data: data,
            }
        }
    }

    impl<'a> AddressAndData<'a> for AdvScanInd<'a> {
        const PDU_TYPE: u8 = 0b0110;

        fn address(&self) -> &Address {
            &self.adv_address
        }

        fn data(&self) -> &[u8] {
            self.adv_data
        }

        fn from_address_and_data(address: Address, data: &'a [u8]) -> Self {
            Self {
                adv_address: address,
                adv_data: data,
            }
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

            let mut bytes = [0u8; 39];
            let len = actual.bytes(&mut bytes);
            assert_eq!(bytes[..len], expected);
        }

        #[test]
        fn scan_complementary() {
            let expected = AdvScanInd {
                adv_address: Address::new_random(0xffe1e8d0dc27),
                adv_data: &[0x01, 0x02, 0x03],
            };

            let actual = [
                0x46u8, // ADV_SCAN_ID, Random address,
                9,      // Length of payload
                0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
                0x01, 0x02, 0x03, // Data
            ];

            assert_eq!(AdvScanInd::parse(&actual).unwrap(), expected);
        }
    }
}

mod scan_req {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Format)]
    pub struct ScanReq {
        scan_address: Address,
        adv_address: Address,
    }

    impl ScanReq {
        pub fn new(scan_address: Address, adv_address: Address) -> Self {
            Self {
                scan_address,
                adv_address,
            }
        }
    }

    impl TwoAddress for ScanReq {
        const PDU_TYPE: u8 = 0b0011;

        fn first(&self) -> &Address {
            &self.scan_address
        }

        fn second(&self) -> &Address {
            &self.adv_address
        }

        fn from_addresses(first: Address, second: Address) -> Self {
            Self {
                scan_address: first,
                adv_address: second,
            }
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

            let mut bytes = [0u8; 14];
            let len = actual.bytes(&mut bytes);
            assert_eq!(bytes[..len], expected);
        }

        #[test]
        fn scan_request_complementary() {
            let expected = ScanReq {
                scan_address: Address::new_random(0xffe1e8d0dc27),
                adv_address: Address::new_random(0xffe1e8d0dc27),
            };

            let actual = [
                0xC3u8, // ADV_DIRECT_IND, Random address,
                12,     // Length of payload
                0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
                0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
            ];

            assert_eq!(ScanReq::parse(&actual).unwrap(), expected);
        }
    }
}

mod scan_rsp {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Format)]
    pub struct ScanRsp<'a> {
        adv_address: Address,
        adv_data: &'a [u8],
    }

    impl<'a> ScanRsp<'a> {
        pub fn new(address: Address, data: &'a [u8]) -> Self {
            Self {
                adv_address: address,
                adv_data: data,
            }
        }
    }

    impl<'a> AddressAndData<'a> for ScanRsp<'a> {
        const PDU_TYPE: u8 = 0b0100;

        fn address(&self) -> &Address {
            &self.adv_address
        }

        fn data(&self) -> &[u8] {
            self.adv_data
        }

        fn from_address_and_data(address: Address, data: &'a [u8]) -> Self {
            Self {
                adv_address: address,
                adv_data: data,
            }
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

            let mut bytes = [0u8; 39];
            let len = actual.bytes(&mut bytes);
            assert_eq!(bytes[..len], expected);
        }

        #[test]
        fn scan_response_complementary() {
            let expected = ScanRsp {
                adv_address: Address::new_random(0xffe1e8d0dc27),
                adv_data: &[0x01, 0x02, 0x03],
            };

            let actual = [
                0x44u8, // ADV_NONCONN_IND, Random address,
                9,      // Length of payload
                0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
                0x01, 0x02, 0x03, // Data
            ];

            assert_eq!(ScanRsp::parse(&actual).unwrap(), expected);
        }
    }
}
