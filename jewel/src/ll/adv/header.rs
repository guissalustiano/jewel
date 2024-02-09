use defmt::Format;

use crate::ll::AddressType;

#[derive(Debug, Clone, Format)]
pub struct Header {
    pub flags: Flags,
    pub length: u8,
}

impl Header {
    pub fn bytes(&self) -> [u8; 2] {
        [self.flags.bytes()[0], self.length]
    }

    pub fn parse(bytes: &[u8; 2]) -> Self {
        let (flags, length) = bytes.split_at(1);
        Self {
            flags: Flags::parse(flags.try_into().unwrap()),
            length: length[0],
        }
    }

    // The TxAdd in the advertising physical channel PDU header indicates whether the
    // advertiser’s address in the AdvA field is public (TxAdd = 0) or random (TxAdd = 1).
    // Also works for the RxAdd
    fn address_type_to_header_bit(address_type: AddressType) -> bool {
        match address_type {
            AddressType::Public => false,
            AddressType::Random => true,
        }
    }

    pub fn bit_to_address_type(bit: bool) -> AddressType {
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

    pub(crate) fn with_tx(adv_address_type: AddressType, pdu_type: u8, length: u8) -> Self {
        let mut header = Self::new(pdu_type, length);
        header.flags.tx_add = Self::address_type_to_header_bit(adv_address_type);
        header
    }

    pub(crate) fn with_rxtx(
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
pub struct Flags {
    pub rx_add: bool,
    pub tx_add: bool,
    pub ch_sel: bool,
    pub rfu: bool,
    pub pdu_type: u8,
}

fn bool_to_bit(b: bool) -> u8 {
    match b {
        true => 1,
        false => 0,
    }
}

impl Flags {
    /// RxAdd | TxAdd | ChSel | RFU | PDU Type
    pub fn bytes(&self) -> [u8; 1] {
        let byte = (bool_to_bit(self.rx_add)) << 7
            | (bool_to_bit(self.tx_add)) << 6
            | (bool_to_bit(self.ch_sel)) << 5
            | (bool_to_bit(self.rfu)) << 4
            | (self.pdu_type & 0b1111);

        [byte]
    }

    pub fn parse(bytes: &[u8; 1]) -> Self {
        let byte = bytes[0];
        Self {
            rx_add: (byte & 0b1000_0000) != 0,
            tx_add: (byte & 0b0100_0000) != 0,
            ch_sel: (byte & 0b0010_0000) != 0,
            rfu: (byte & 0b0001_0000) != 0,
            pdu_type: byte & 0b0000_1111,
        }
    }
}
