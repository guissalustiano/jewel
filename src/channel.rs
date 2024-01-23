/// 3 of the 39 RF channels used for initial advertising and all legacy advertising activitie.
///
/// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-3abb4023-1a31-b4db-cb9d-a70064cb40a0
pub enum PrimaryAdvertisingChannel {
    #[doc = "Channel 37"]
    Ch37,

    #[doc = "Channel 38"]
    Ch38,

    #[doc = "Channel 39"]
    Ch39,
}

impl PrimaryAdvertisingChannel {
    // https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-8d4b6daf-4142-e928-81d1-520529d8277f
    fn physical_index(&self) -> u8 {
        match self {
            PrimaryAdvertisingChannel::Ch37 => 37,
            PrimaryAdvertisingChannel::Ch38 => 38,
            PrimaryAdvertisingChannel::Ch39 => 39,
        }
    }

    fn channel_index(&self) -> u8 {
        match self {
            PrimaryAdvertisingChannel::Ch37 => 0,
            PrimaryAdvertisingChannel::Ch38 => 12,
            PrimaryAdvertisingChannel::Ch39 => 39,
        }
    }

    /// RF channel center frequency in MHz
    /// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-8d4b6daf-4142-e928-81d1-520529d8277f
    /// ```
    /// use embassy_nrf::radio::ble::PrimaryAdvertisingChannel;
    ///
    /// assert_eq!(PrimaryAdvertisingChannel::Channel37.central_frequency(), 2402);
    /// assert_eq!(PrimaryAdvertisingChannel::Channel38.central_frequency(), 2426);
    /// assert_eq!(PrimaryAdvertisingChannel::Channel39.central_frequency(), 2480);
    /// ```
    pub fn central_frequency(&self) -> u16 {
        2402u16 + (self.channel_index() as u16) * 2u16
    }

    /// The whitener and de-whitener are defined the same way, using a 7-bit
    /// linear feedback shift register with the polynomial x7 + x4 + 1.
    /// Before whitening or de-whitening, the shift register is initialized
    /// with a sequence that is derived from the physical channel index in
    /// which the packet is transmitted in the following manner:
    /// - Position 0 is set to one.
    /// - Positions 1 to 6 are set to the channel index of the channel used
    ///  when transmitting or receiving, from the most significant bit in
    /// position 1 to the least significant bit in position 6.
    ///
    /// For example, if the channel index = 23 (0x17), the positions would be set as follows:
    /// Position 0 = 1
    /// Position 1 = 0
    /// Position 2 = 1
    /// Position 3 = 0
    /// Position 4 = 1
    /// Position 5 = 1
    /// Position 6 = 1
    ///
    /// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-c4266341-6cc3-b2d0-3b02-17c240cf2fa4
    ///
    /// ```
    /// use embassy_nrf::radio::ble::PrimaryAdvertisingChannel;
    ///
    /// assert_eq!(PrimaryAdvertisingChannel::Channel37.whitening_init(), 0x70);
    /// assert_eq!(PrimaryAdvertisingChannel::Channel38.whitening_init(), 0x7C);
    /// assert_eq!(PrimaryAdvertisingChannel::Channel39.whitening_init(), 0x77);
    /// ```
    pub fn whitening_init(&self) -> u8 {
        0b01000000 | self.physical_index()
    }
}
