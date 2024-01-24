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

pub fn parse(bytes: &[u8]) -> Result<NonConnectableUndirected, ParseError> {
    let package = NonConnectableUndirected::parse(bytes);
    if package.is_ok() {
        return package;
    }

    Err(ParseError::InvalidType)
}

#[derive(Debug, Clone)]
pub struct Address {
    // Little endian address
    address_le: [u8; 6],

    address_type: AddressType,
}

#[derive(Debug, Clone, Copy)]
pub enum AddressType {
    Public,
    Random,
}

impl Address {
    fn new_le(address_le: [u8; 6], address_type: AddressType) -> Self {
        Self {
            address_le,
            address_type,
        }
    }

    // from a big endian address
    pub fn new_be(address_be: [u8; 6], address_type: AddressType) -> Self {
        let address_le = [
            address_be[5],
            address_be[4],
            address_be[3],
            address_be[2],
            address_be[1],
            address_be[0],
        ];

        Self::new_le(address_le, address_type)
    }

    // Little endian address
    // ```
    // let address = Address::new_be([0xff, 0xe1, 0xe8, 0xd0, 0xdc, 0x27], AddressType::Random).bytes()
    // assert_eq!(address.bytes(), [0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff]);
    // ```
    pub fn bytes(&self) -> [u8; 6] {
        self.address_le
    }
}

#[derive(Debug, Clone)]
struct Header {
    flags: Flags,
    length: u8,
}

impl Header {
    fn parse(bytes: [u8; 2]) -> Self {
        Self {
            flags: Flags::parse(bytes[0]),
            length: bytes[1],
        }
    }

    fn bytes(&self) -> [u8; 2] {
        [self.flags.bytes(), self.length]
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

impl Flags {
    fn parse(byte: u8) -> Self {
        Self {
            rx_add: (byte & 0b1000_0000) != 0,
            tx_add: (byte & 0b0100_0000) != 0,
            ch_sel: (byte & 0b0010_0000) != 0,
            rfu: (byte & 0b0001_0000) != 0,
            pdu_type: byte & 0b0000_1111,
        }
    }

    /// RxAdd | TxAdd | ChSel | RFU | PDU Type
    fn bytes(&self) -> u8 {
        let bool2bit = |b: bool| -> u8 {
            if b {
                1
            } else {
                0
            }
        };

        (bool2bit(self.rx_add)) << 7
            | (bool2bit(self.tx_add)) << 6
            | (bool2bit(self.ch_sel)) << 5
            | (bool2bit(self.rfu)) << 4
            | (self.pdu_type & 0b1111)
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
pub struct NonConnectableUndirected<'a> {
    pub address: Address,
    pub data: &'a [u8],
}

impl<'a> NonConnectableUndirected<'a> {
    pub const TYPE: u8 = 0b0010;

    pub fn bytes(&self) -> [u8; 39] {
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
        bytes[..2].copy_from_slice(&header.bytes());

        // write address
        bytes[2..8].copy_from_slice(&self.address.bytes());

        // write data
        bytes[8..(8 + self.data.len())].copy_from_slice(self.data);

        bytes
    }

    pub fn parse(bytes: &'a [u8]) -> Result<Self, ParseError> {
        if bytes.len() < 8 {
            return Err(ParseError::InvalidLength);
        }

        let header = Header::parse(bytes[0..2].try_into().unwrap());

        if header.flags.pdu_type != Self::TYPE {
            return Err(ParseError::InvalidLength);
        }

        let address = Address::new_le(
            bytes[2..8].try_into().unwrap(),
            if header.flags.tx_add {
                AddressType::Random
            } else {
                AddressType::Public
            },
        );

        let data = &bytes[8..(8 + header.length as usize)];

        Ok(Self { address, data })
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidType,
    InvalidLength,
    InvalidHeader,
}
