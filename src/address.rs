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
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Address {
    // Little endian address
    address_le: [u8; 6],

    pub(crate) address_type: AddressType,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AddressType {
    Public,
    Random,
}

impl Address {
    /// from a little endian address
    pub(crate) fn new_le(address_le: [u8; 6], address_type: AddressType) -> Self {
        Self {
            address_le,
            address_type,
        }
    }

    /// create an address in big endian array
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

    // Serizalie the address in little endian address for transmission
    // ```
    // let address = Address::new_be([0xff, 0xe1, 0xe8, 0xd0, 0xdc, 0x27], AddressType::Random).bytes()
    // assert_eq!(address.bytes(), [0x27, 0xdc, 0xd0, 0xe8, 0xe1, 0xff]);
    // ```
    pub fn transmission_bytes(&self) -> [u8; 6] {
        self.address_le
    }
}
