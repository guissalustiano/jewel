/// Advertising phycial channel PDU
///
///    LSB                                                          MSB
///   ┌──────────┬──────────┬─────────┬──────────┬──────────┬──────────┬────────────────────────────────────────┐
///   │ PDU Type │ RFU      │ ChSel   │ TxAdd    │ RxAdd    │ Length   │                                        │
///   │ (4 bits) │ (1 bit)  │ (1 bit) │ (1 bit)  │ (1 bit)  │ (8 bits) │                                        │
///   ├──────────┴──────────┴─────────┴──────────┴──────────┴──────────┼────────────────────────────────────────┤
///   │                        HEADER (16 bits)                        │           Payload (1-255 bytes)        │
///   └────────────────────────────────────────────────────────────────┴────────────────────────────────────────┘
///
/// The ChSel, TxAdd and RxAdd fields of the advertising physical
/// channel PDU that are contained in the header contain information
/// specific to the PDU type defined for each advertising physical
/// channel PDU separately (represented by X).
///
/// If the ChSel, TxAdd or RxAdd fields are not defined as used in a
/// given PDU then they shall be considered reserved for future use.
/// (represented by -)
///
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-970b9251-9089-5ea4-1634-41defd816278
use crate::{
    address::{Address, AddressType},
    radio::Packet,
};

pub fn parse(bytes: &[u8]) -> Result<NonConnectableUndirected, ParseError> {
    let package = NonConnectableUndirected::reception_parse(bytes[0..39].try_into().unwrap());
    if package.is_ok() {
        return package;
    }

    Err(ParseError::InvalidType)
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

/// Advertising Non-Connectable Indication
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
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-3544231c-d808-9b6f-8f5a-d45c1c467d4e
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonConnectableUndirected<'a> {
    pub address: Address,
    pub data: &'a [u8],
}

impl<'a> NonConnectableUndirected<'a> {
    pub const TYPE: u8 = 0b0010;
}

impl<'a> Packet<'a, 39> for NonConnectableUndirected<'a> {
    type Error = ParseError;
    fn transmission_bytes(&self) -> [u8; 39] {
        let mut bytes = [0u8; 39];

        // AdvAddr.len() + AdvAddr.len()
        let payload_len = 6 + self.data.len();
        assert!(self.data.len() <= 37);

        // The TxAdd in the advertising physical channel PDU header indicates whether the
        // advertiser’s address in the AdvA field is public (TxAdd = 0) or random (TxAdd = 1).
        let tx_add = match self.address.address_type {
            AddressType::Public => false,
            AddressType::Random => true,
        };

        let header = Header {
            flags: Flags {
                rx_add: false,
                tx_add,
                ch_sel: false,
                rfu: false,
                pdu_type: Self::TYPE,
            },
            length: payload_len as u8,
        };

        // write flags and length
        bytes[..2].copy_from_slice(&header.transmission_bytes());

        // write address
        bytes[2..8].copy_from_slice(&self.address.transmission_bytes());

        // write data
        bytes[8..(8 + self.data.len())].copy_from_slice(self.data);

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

        let address = Address::new_le(
            pdu[0..6].try_into().unwrap(),
            if header.flags.tx_add {
                AddressType::Random
            } else {
                AddressType::Public
            },
        );

        let data = &pdu[6..(header.length as usize)];

        Ok(Self { address, data })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum ParseError {
    InvalidType,
    InvalidLength,
    InvalidHeader,
}

#[cfg(test)]
mod test {
    use super::*;

    // Exemple generate from
    #[test]
    fn hello_word_adv_pdu() {
        let actual = NonConnectableUndirected {
            address: Address::new_be([0xff, 0xe1, 0xe8, 0xd0, 0xdc, 0x27], AddressType::Random),
            data: &[
                0x02, 0x01, 0x06, // Flags
                0x03, 0x03, 0x09, 0x18, // Complete list of 16-bit UUIDs available
                0x0A, 0x09, // Length, Type: Device name
                b'H', b'e', b'l', b'l', b'o', b'R', b'u', b's', b't',
            ],
        };

        let expected = [
            0x42u8, // ADV_NONCONN_IND, Random address,
            0x18,   // Length of payload
            0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff, // Adress
            0x02, 0x01, 0x06, // Flags
            0x03, 0x03, 0x09, 0x18, // Complete list of 16-bit UUIDs available
            0x0A, 0x09, // Length, Type: Device name
            b'H', b'e', b'l', b'l', b'o', b'R', b'u', b's', b't',
        ];

        assert_eq!(actual.transmission_bytes()[..(expected.len())], expected);
    }

    #[test]
    fn non_connectable_bytes_parse_is_complementary() {
        let packet = NonConnectableUndirected {
            address: Address::new_be([0xff, 0xe1, 0xe8, 0xd0, 0xdc, 0x27], AddressType::Random),
            data: &[0x01, 0x02, 0x03],
        };

        let bytes = packet.transmission_bytes();
        let actual = NonConnectableUndirected::reception_parse(&bytes).unwrap();

        dbg!(&bytes);

        assert_eq!(packet, actual);
    }
}
