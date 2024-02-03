//! Advertising and Scan Response data format
//!
//! Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/generic-access-profile.html#UUID-c2a0b759-8ef4-7055-c13b-17c083691361
//!
//! The AD type identifier values are defined in [Assigned Numbers](https://www.bluetooth.com/specifications/assigned-numbers/).
//! The AD type data formats and meanings are defined in Section 1 of the Part A of the [Core Specification Supplementi](https://www.bluetooth.com/specifications/specs/core-specification-supplement-10/)
pub use flags::*;
pub use local_name::*;
pub use uuid::*;
///
/// Cap 1.1 in Section 1.3 of the Core Specification Supplement
mod uuid {
    use defmt::Format;

    type Uuid16 = u16;
    type Uuid32 = u32;
    pub use uuid::Uuid as Uuid128;

    #[derive(Debug, Clone, Format, Copy, PartialEq, Eq)]
    pub struct IncompleteListOf16BitServiceUuids(Uuid16);

    impl IncompleteListOf16BitServiceUuids {
        pub const AD_TYPE: u8 = 0x02;

        pub fn bytes(&self) -> [u8; 2] {
            self.0.to_be_bytes()
        }

        pub fn parse(bytes: &[u8; 2]) -> Self {
            Self(Uuid16::from_be_bytes(*bytes))
        }
    }

    #[derive(Debug, Clone, Format, Copy, PartialEq, Eq)]
    pub struct CompleteListOf16BitServiceUuids(Uuid16);

    impl CompleteListOf16BitServiceUuids {
        pub const AD_TYPE: u8 = 0x03;

        pub fn bytes(&self) -> [u8; 2] {
            self.0.to_be_bytes()
        }

        pub fn parse(bytes: &[u8; 2]) -> Self {
            Self(Uuid16::from_be_bytes(*bytes))
        }
    }

    #[derive(Debug, Clone, Format, Copy, PartialEq, Eq)]
    pub struct IncompleteListOf32BitServiceUuids(Uuid32);

    impl IncompleteListOf32BitServiceUuids {
        pub const AD_TYPE: u8 = 0x04;

        pub fn bytes(&self) -> [u8; 4] {
            self.0.to_be_bytes()
        }

        pub fn parse(bytes: &[u8; 4]) -> Self {
            Self(Uuid32::from_be_bytes(*bytes))
        }
    }

    #[derive(Debug, Clone, Format, Copy, PartialEq, Eq)]
    pub struct CompleteListOf32BitServiceUuids(Uuid32);

    impl CompleteListOf32BitServiceUuids {
        pub const AD_TYPE: u8 = 0x05;

        pub fn bytes(&self) -> [u8; 4] {
            self.0.to_be_bytes()
        }

        pub fn parse(bytes: &[u8; 4]) -> Self {
            Self(Uuid32::from_be_bytes(*bytes))
        }
    }

    #[derive(Debug, Clone, Format, Copy, PartialEq, Eq)]
    pub struct IncompleteListOf128BitServiceUuids(Uuid128);

    impl IncompleteListOf128BitServiceUuids {
        pub const AD_TYPE: u8 = 0x06;

        pub fn bytes(&self) -> [u8; 16] {
            *self.0.as_bytes()
        }

        pub fn parse(bytes: &[u8; 16]) -> Self {
            Self(Uuid128::from_bytes(*bytes))
        }
    }

    #[derive(Debug, Clone, Format, Copy, PartialEq, Eq)]
    pub struct CompleteListOf128BitServiceUuids(Uuid128);

    impl CompleteListOf128BitServiceUuids {
        pub const AD_TYPE: u8 = 0x07;

        pub fn bytes(&self) -> [u8; 16] {
            *self.0.as_bytes()
        }

        pub fn parse(bytes: &[u8; 16]) -> Self {
            Self(Uuid128::from_bytes(*bytes))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn serialize_incomplete_list_of_16_bit_service_uuids() {
            let uuid = IncompleteListOf16BitServiceUuids(0x1234);
            assert_eq!(uuid.bytes(), [0x12, 0x34]);
        }

        #[test]
        fn deserialize_incomplete_list_of_16_bit_service_uuids() {
            let uuid = IncompleteListOf16BitServiceUuids::parse(&[0x12, 0x34]);
            assert_eq!(uuid.0, 0x1234);
        }

        #[test]
        fn serialize_complete_list_of_16_bit_service_uuids() {
            let uuid = CompleteListOf16BitServiceUuids(0x1234);
            assert_eq!(uuid.bytes(), [0x12, 0x34]);
        }

        #[test]
        fn deserialize_complete_list_of_16_bit_service_uuids() {
            let uuid = CompleteListOf16BitServiceUuids::parse(&[0x12, 0x34]);
            assert_eq!(uuid.0, 0x1234);
        }

        #[test]
        fn serialize_incomplete_list_of_32_bit_service_uuids() {
            let uuid = IncompleteListOf32BitServiceUuids(0x1234_5678);
            assert_eq!(uuid.bytes(), [0x12, 0x34, 0x56, 0x78]);
        }

        #[test]
        fn deserialize_incomplete_list_of_32_bit_service_uuids() {
            let uuid = IncompleteListOf32BitServiceUuids::parse(&[0x12, 0x34, 0x56, 0x78]);
            assert_eq!(uuid.0, 0x1234_5678);
        }

        #[test]
        fn serialize_complete_list_of_32_bit_service_uuids() {
            let uuid = CompleteListOf32BitServiceUuids(0x1234_5678);
            assert_eq!(uuid.bytes(), [0x12, 0x34, 0x56, 0x78]);
        }

        #[test]
        fn deserialize_complete_list_of_32_bit_service_uuids() {
            let uuid = CompleteListOf32BitServiceUuids::parse(&[0x12, 0x34, 0x56, 0x78]);
            assert_eq!(uuid.0, 0x1234_5678);
        }

        #[test]
        fn serialize_incomplete_list_of_128_bit_service_uuids() {
            let uuid = IncompleteListOf128BitServiceUuids(Uuid128::from_u128(
                0x1234_5678_9abc_def0_1234_5678_9abc_def0,
            ));
            assert_eq!(
                uuid.bytes(),
                [
                    0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
                    0xbc, 0xde, 0xf0
                ]
            );
        }

        #[test]
        fn deserialize_incomplete_list_of_128_bit_service_uuids() {
            let uuid = IncompleteListOf128BitServiceUuids::parse(&[
                0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc,
                0xde, 0xf0,
            ]);
            assert_eq!(
                uuid.0,
                Uuid128::from_u128(0x1234_5678_9abc_def0_1234_5678_9abc_def0)
            );
        }

        #[test]
        fn serialize_complete_list_of_128_bit_service_uuids() {
            let uuid = CompleteListOf128BitServiceUuids(Uuid128::from_u128(
                0x1234_5678_9abc_def0_1234_5678_9abc_def0,
            ));
            assert_eq!(
                uuid.bytes(),
                [
                    0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
                    0xbc, 0xde, 0xf0
                ]
            );
        }

        #[test]
        fn deserialize_complete_list_of_128_bit_service_uuids() {
            let uuid = CompleteListOf128BitServiceUuids::parse(&[
                0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc,
                0xde, 0xf0,
            ]);
            assert_eq!(
                uuid.0,
                Uuid128::from_u128(0x1234_5678_9abc_def0_1234_5678_9abc_def0)
            );
        }
    }
}

/// Cap 1.2 in Section 1.3 of the Core Specification Supplement
mod local_name {
    use defmt::Format;

    #[derive(Debug, Clone, Format, Copy, PartialEq, Eq)]
    pub struct ShortenedLocalName<'a>(&'a str);

    impl<'a> ShortenedLocalName<'a> {
        pub const AD_TYPE: u8 = 0x08;

        pub fn bytes(&self) -> &[u8] {
            self.0.as_bytes()
        }

        pub fn parse(bytes: &'a [u8]) -> Self {
            Self(core::str::from_utf8(bytes).unwrap())
        }
    }

    #[derive(Debug, Clone, Format, Copy, PartialEq, Eq)]
    pub struct CompleteLocalName<'a>(&'a str);

    impl<'a> CompleteLocalName<'a> {
        pub const AD_TYPE: u8 = 0x09;

        pub fn bytes(&self) -> &[u8] {
            self.0.as_bytes()
        }

        pub fn parse(bytes: &'a [u8]) -> Self {
            Self(core::str::from_utf8(bytes).unwrap())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn serialize_shortened_local_name() {
            let name = ShortenedLocalName("test");
            assert_eq!(name.bytes(), b"test");
        }

        #[test]
        fn deserialize_shortened_local_name() {
            let name = ShortenedLocalName::parse(b"test");
            assert_eq!(name.0, "test");
        }

        #[test]
        fn serialize_complete_local_name() {
            let name = CompleteLocalName("test");
            assert_eq!(name.bytes(), b"test");
        }

        #[test]
        fn deserialize_complete_local_name() {
            let name = CompleteLocalName::parse(b"test");
            assert_eq!(name.0, "test");
        }
    }
}

/// Cap 1.3 in Section 1.3 of the Core Specification Supplement
mod flags {
    use defmt::Format;

    #[derive(Debug, Clone, Format, Copy, PartialEq, Eq)]
    pub struct Flags {
        /// Device operating in LE Limited Discoverable mode.
        ///
        /// Either this or `le_general_disc` must be set for the device to be discoverable.
        /// Note that "Broadcast Mode" still works with undiscoverable devices, since it doesn't need
        /// discovery or connections.
        pub le_limited_disc: bool,

        /// Device operating in LE General Discoverable mode.
        ///
        /// Either this or `le_limited_disc` must be set for the device to be discoverable.
        /// Note that "Broadcast Mode" still works with undiscoverable devices, since it doesn't need
        /// discovery or connections.
        pub le_general_disc: bool,

        /// Indicate if the device that sent this `Flags` value supports BR/EDR (aka "Classic Bluetooth").
        pub br_edr_not_supported: bool,

        pub simultaneous_le_bredr_capable_controller: bool, // not used in BLE
    }

    // Based in rubble implementation https://github.com/jonas-schievink/rubble/blob/f475c20e213fcd0a957521951bea3c3892699640/rubble/src/link/ad_structure.rs#L264-L309
    impl Flags {
        pub const AD_TYPE: u8 = 0x01;

        /// Returns flags suitable for discoverable devices that want to establish a connection.
        ///
        /// The created `Flags` value specifies that this device is not BR/EDR (classic Bluetooth)
        /// capable and is in General Discoverable mode.
        pub fn discoverable() -> Self {
            Self {
                le_limited_disc: false,
                le_general_disc: true,
                br_edr_not_supported: true,
                simultaneous_le_bredr_capable_controller: false,
            }
        }

        /// Returns flags suitable for non-connectable devices that just broadcast advertising packets.
        ///
        /// Creates a `Flags` value that specifies that BR/EDR (classic Bluetooth) is not supported and
        /// that this device is not discoverable.
        pub fn broadcast() -> Self {
            Self {
                le_limited_disc: false,
                le_general_disc: false,
                br_edr_not_supported: true,
                simultaneous_le_bredr_capable_controller: false,
            }
        }

        pub fn byte(&self) -> u8 {
            (self.le_limited_disc as u8)
                | (self.le_general_disc as u8) << 1
                | (self.br_edr_not_supported as u8) << 2
                | (self.simultaneous_le_bredr_capable_controller as u8) << 3
        }

        pub fn parse(byte: u8) -> Self {
            Self {
                le_limited_disc: byte & 0b0000_0001 != 0,
                le_general_disc: byte & 0b0000_0010 != 0,
                br_edr_not_supported: byte & 0b0000_0100 != 0,
                simultaneous_le_bredr_capable_controller: byte & 0b0000_1000 != 0,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn serialize_flags() {
            let flags = Flags {
                le_limited_disc: true,
                le_general_disc: false,
                br_edr_not_supported: true,
                simultaneous_le_bredr_capable_controller: false,
            };

            assert_eq!(flags.byte(), 0b0000_0101);
        }

        #[test]
        fn deserialize_flags() {
            assert_eq!(
                Flags::parse(0b0000_0101),
                Flags {
                    le_limited_disc: true,
                    le_general_disc: false,
                    br_edr_not_supported: true,
                    simultaneous_le_bredr_capable_controller: false,
                }
            );
        }
    }
}
