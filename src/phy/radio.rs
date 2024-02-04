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
//    │Preamble   │Access-Address│(1 byte)│(1 byte)│(1 byte)│ Data PDU      │CRC      │Costant tone extension l
//    │(1-2 bytes)│(4 bytes)     │        │        │        │ (1-255 bytes) │(3 bytes)│(16 to 160 us)         l
//    │           │              ├────────┴────────┴--------┤               │         │                       l
//    │           │              │ Data Header (2-3 bytes)  │               │         │                       l
//    │           │              ├──────────────────────────┴───────────────┤         │                       l
//    │           │              │                                          │         │                       l
//    │           │              │  PDU (2-258 bytes)                       │         │                       l
//    │           │              │                                          │         │                       l
//    │           │              │                                          │         │                       l
//    └───────────┴──────────────┴──────────────────────────────────────────┴─────────┴-----------------------┘
//
//The CREInfo and Constant Tone Extension are optional.
//
//The Length field is the length of the PDU, not including the header.
//On the nrf52 is used to define how mutch bytes the radio will read from the pointer and send.
//
//Each fild is send with the least significant bit first.

use crate::phy::channel::Channel;

pub const MAX_PDU_LENGTH: usize = 258;

// For BLE the CRC polynomial is
// `x^24 + x^10 + x^9 + x^6 + x^4 + x^3 + x + 1`
pub const CRC_POLY: u32 = 0b00000001_00000000_00000110_01011011;

/// BLE PHY
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "1 Mbit/s BLE with 1 byte preamble"]
    Ble1mbit,
    #[doc = "2 Mbit/s BLE with 2 byte preamble"]
    Ble2mbit,
}

/// Data Physical Channel PDU header could require and extra byte to the CTEInfo.
/// This byte cannot be used in the payload because change their size.
pub enum HeaderSize {
    TwoBytes,
    ThreeBytes,
}

/// I only know enough about nrf52, so this is a interface specific for it for now, but must be generalized later.
pub trait BleRadio<'b> {
    type Error;
    /// Set the radio mode and respective preamble length
    ///
    /// The radio must be disabled before calling this function
    fn set_mode(&mut self, mode: Mode);

    /// Set the radio tx power
    /// round to the nearest supported value and clamp the value outside the support range
    ///
    /// The radio must be disabled before calling this function
    fn set_tx_power(&mut self, power_db: i8);

    /// Set the header size, 2 or 3 bytes
    ///
    /// The radio must be disabled before calling this function
    fn set_header_size(&mut self, header_size: HeaderSize);

    /// Set the acess address, the 4 bytes after the preamble
    ///
    /// The radio must be disabled before calling this function
    fn set_access_address(&mut self, access_address: u32);

    // Set channel
    ///
    /// The radio must be disabled before calling this function
    fn set_channel(&mut self, channel: Channel);

    /// Set the CRC init value
    ///
    /// The radio must be disabled before calling this function
    fn set_crc_init(&mut self, crc_init: u32);

    /// Set buffer to read/write
    /// The buffer should exist for the life time of the transmission
    ///
    /// The buffer should live for the life time of the transmission/reception
    fn set_buffer(&mut self, buffer: &'b [u8]) -> Result<(), Self::Error>;

    // Set buffer mut
    fn set_buffer_mut(&mut self, buffer: &'b mut [u8]) -> Result<(), Self::Error> {
        self.set_buffer(buffer)
    }

    /// Transmit the packaget in the  buffer
    #[allow(async_fn_in_trait)]
    async fn transmit(&mut self);

    /// Receive the packaget to the buffer
    #[allow(async_fn_in_trait)]
    async fn receive(&mut self);
}
