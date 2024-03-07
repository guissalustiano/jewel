/// Devices are identified using a device address and an address type;
/// the address type indicates either a public device address or a random
/// device address. A public device address and a random device address are
/// both 48 bits in length.
/// A device shall use at least one type of device address and may use both.
/// The device may be addressed by any device address that it uses.
/// A device's Identity Address is a Public Device Address or
/// Random Static Device Address that it uses in packets it transmits.
/// If a device is using Resolvable Private Addresses, it shall also have an
/// Identity Address.
/// The comparision of two device addresses includes the device address type
///
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-3815b05a-b69c-4e3c-5897-c8d3baa4fc30
use defmt::Format;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Address {
    // Little endian address
    address_le: [u8; 6],
    pub r#type: AddressType,
}

impl Format for Address {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Address({:#08X}, {:?})", self.u64_address(), self.r#type);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Format)]
pub enum AddressType {
    Public,
    Random,
}

impl Address {
    /// from a little endian address
    pub(crate) fn new_le(address_le: [u8; 6], address_type: AddressType) -> Self {
        Self {
            address_le,
            r#type: address_type,
        }
    }

    fn u64_to_le(address: u64) -> [u8; 6] {
        [
            (address & 0xff) as u8,
            ((address >> 8) & 0xff) as u8,
            ((address >> 16) & 0xff) as u8,
            ((address >> 24) & 0xff) as u8,
            ((address >> 32) & 0xff) as u8,
            ((address >> 40) & 0xff) as u8,
        ]
    }

    /// Create a new public address from a big endian address
    /// ```
    /// use jewel::ll::{Address, AddressType};
    /// let address = Address::new_public(0xffe1e8d0dc27);
    /// assert_eq!(address.r#type, AddressType::Public);
    /// ```
    pub fn new_public(address: u64) -> Self {
        Self::new_le(Self::u64_to_le(address), AddressType::Public)
    }

    /// Create a new random address from a big endian address
    /// ```
    /// use jewel::ll::{Address, AddressType};
    /// let address = Address::new_random(0xffe1e8d0dc27);
    /// assert_eq!(address.r#type, AddressType::Random);
    /// ```
    pub fn new_random(address: u64) -> Self {
        Self::new_le(Self::u64_to_le(address), AddressType::Random)
    }

    /// Serizalie the address in little endian address for transmission
    /// ```
    /// use jewel::ll::Address;
    /// let address = Address::new_random(0xffe1e8d0dc27);
    /// assert_eq!(address.bytes(), [0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff]);
    /// ```
    pub fn bytes(&self) -> [u8; 6] {
        self.address_le
    }

    fn u64_address(&self) -> u64 {
        u64::from_le_bytes([
            self.address_le[0],
            self.address_le[1],
            self.address_le[2],
            self.address_le[3],
            self.address_le[4],
            self.address_le[5],
            0,
            0,
        ])
    }
}
