//! BLE packet format for the LE Uncoded PHYs
//
//    ┌───────────┬──────────────┬────────┬────────┬────────────────────────┬─────────┬-----------------------┐
//    │           │              │        │        │                        │         │                       l
//    │           │              │Flags   │Length  │                        │         │                       l
//    │           │              │(1 byte)│(1 byte)│  Adv PD                │         │                       l
//    │           │              │        │        │  (1-255 bytes)         │         │                       l
//    │           │              ├────────┴────────┤                        │         │                       l
//    │           │              │ Adv Header (2B) │                        │         │                       l
//    │           │              ├────────┬────────┼--------┬───────────────┤         │                       l
//    │           │              │        │        │        │               │         │                       l
//    │           │              │Flags   │Length  │CREInfo │               │         │                       l
//    │Preamble   │Access-Address│(1 byte)│(1 byte)│[1 byte]│ Data PDU      │CRC      │Costant tone extension l
//    │(1-2 bytes)│(4 bytes)     │        │        │        │ (1-255 bytes) │(3 bytes)│[16 to 160 us]         l
//    │           │              ├────────┴────────┴--------┤               │         │                       l
//    │           │              │ Data Header (2-3 bytes)  │               │         │                       l
//    │           │              ├──────────────────────────┴───────────────┤         │                       l
//    │           │              │                                          │         │                       l
//    │           │              │  PDU (2-258 bytes)                       │         │                       l
//    │           │              │                                          │         │                       l
//    │           │              │                                          │         │                       l
//    └───────────┴──────────────┴──────────────────────────────────────────┴─────────┴-----------------------┘
//
// The CREInfo and Constant Tone Extension are optional.
//
// The Length field is the length of the PDU, not including the header.
// On the nrf52 is used to define how mutch bytes the radio will read from the pointer and send.
//
// Each fild is send with the least significant bit first.

use crate::{phy::channel::Channel, Address};

/// Maximum PDU length
pub const MAX_PDU_LENGTH: usize = 258;

/// For BLE the CRC polynomial is `x^24 + x^10 + x^9 + x^6 + x^4 + x^3 + x + 1`
pub const CRC_POLY: u32 = 0b00000001_00000000_00000110_01011011;

/// BLE PHY
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "1 Mbit/s BLE with 1 byte preamble"]
    Ble1mbit,
    // TODO: Add support for other PHYs
    //#[doc = "2 Mbit/s BLE with 2 byte preamble"]
    //Ble2mbit,
}

/// Represents the size of the header
// Data Physical Channel PDU header could require and extra byte to the CTEInfo.
// This byte cannot be used in the payload because change their size.
pub enum HeaderSize {
    TwoBytes,
    ThreeBytes,
}

/// Radio trait
// It is specific for it for now, but must be generalized later.
pub trait Radio {
    type Error;
    /// Set the radio mode and respective preamble length
    fn set_mode(&mut self, mode: Mode);

    /// Set the radio tx power
    /// round to the nearest supported value and clamp the value outside the support range
    fn set_tx_power(&mut self, power_db: i8);

    /// Set the header size, 2 or 3 bytes
    fn set_header_size(&mut self, header_size: HeaderSize);

    /// Set the acess address, the 4 bytes after the preamble
    fn set_access_address(&mut self, access_address: u32);

    // Set channel
    fn set_channel(&mut self, channel: Channel);

    /// Set the CRC polynomial
    fn set_crc_poly(&mut self, crc_poly: u32);

    /// Set the CRC init value
    fn set_crc_init(&mut self, crc_init: u32);

    /// Transmit the packaget in the  buffer
    ///
    /// If the lengh field in the buffer is greather them the buffer size,
    /// the radio will transmit data out of the buffer memory
    #[allow(async_fn_in_trait)]
    async fn transmit(&mut self, buffer: &[u8]) -> Result<(), Self::Error>;

    /// Receive the packaget to the buffer
    ///
    /// If the lengh of receive package is smaller them the buffer
    /// the radio will write out of the bounderies of the radio
    #[allow(async_fn_in_trait)]
    async fn receive(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error>;

    fn device_address(&self) -> Address;

    // TODO: Hardware Link Layer device filtering (6.20.10 Device address match)
}
