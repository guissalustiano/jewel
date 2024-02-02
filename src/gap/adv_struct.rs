//! Advertising and Scan Response data format
//!
//! Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/generic-access-profile.html#UUID-c2a0b759-8ef4-7055-c13b-17c083691361
//!
//! The AD type identifier values are defined in [Assigned Numbers](https://www.bluetooth.com/specifications/assigned-numbers/).
//! The AD type data formats and meanings are defined in Section 1 of the Part A of the [Core Specification Supplementi](https://www.bluetooth.com/specifications/specs/core-specification-supplement-10/)
pub use flags::*;

mod flags {
    use core::fmt;

    use defmt::Format;
    use serde::{
        de::{self, Visitor},
        Serialize, Serializer,
    };

    /// Ref: Cap 1.3
    #[derive(Debug, Clone, Format)]
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
        pub fn broadcast() -> Flags {
            Self {
                le_limited_disc: false,
                le_general_disc: false,
                br_edr_not_supported: true,
                simultaneous_le_bredr_capable_controller: false,
            }
        }
    }

    impl Serialize for Flags {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_u8(
                (self.le_limited_disc as u8)
                    | (self.le_general_disc as u8) << 1
                    | (self.br_edr_not_supported as u8) << 2
                    | (self.simultaneous_le_bredr_capable_controller as u8) << 3,
            )
        }
    }

    impl<'de> Visitor<'de> for Flags {
        type Value = Flags;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a u8")
        }

        fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Flags {
                le_limited_disc: (value & 0b0000_0001) != 0,
                le_general_disc: (value & 0b0000_0010) != 0,
                br_edr_not_supported: (value & 0b0000_0100) != 0,
                simultaneous_le_bredr_capable_controller: (value & 0b0000_1000) != 0,
            })
        }
    }
}
