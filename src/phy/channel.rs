//! RF channels
//!
//! Ref: [Core 6.B.1.4](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-3abb4023-1a31-b4db-cb9d-a70064cb40a0)

/// Utility trait for RF channels
pub trait ChannelTrait {
    fn channel_index(&self) -> u8;
    fn physical_index(&self) -> u8;

    /// RF channel center frequency in MHz
    /// Ref: [Core 6.B.1.4](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-3abb4023-1a31-b4db-cb9d-a70064cb40a0)
    /// ```
    /// use jewel::phy::{AdvertisingChannel, ChannelTrait};
    ///
    /// assert_eq!(AdvertisingChannel::Ch37.central_frequency(), 2402);
    /// assert_eq!(AdvertisingChannel::Ch39.central_frequency(), 2480);
    /// ```
    fn central_frequency(&self) -> u16 {
        2402u16 + (self.physical_index() as u16) * 2u16
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
    ///  (0b101_0111)
    ///
    /// Ref: [Core 6.B.3.2](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-c4266341-6cc3-b2d0-3b02-17c240cf2fa4)
    /// ```
    /// use jewel::phy::{DataChannel, ChannelTrait};
    ///
    /// assert_eq!(DataChannel::Ch23.whitening_init(), 0b0101_0111);
    /// ```
    fn whitening_init(&self) -> u8 {
        0b0100_0000 | self.channel_index()
    }
}

/// 39 RF channels, separated by Advertising channels (Primary Advertising)
/// and Data channels (General Purpose)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel {
    Advertising(AdvertisingChannel),
    Data(DataChannel),
}

/// 3 of the 39 RF channels used for initial advertising and all legacy advertising activitie.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdvertisingChannel {
    #[doc = "Channel 37"]
    Ch37 = 37,

    #[doc = "Channel 38"]
    Ch38 = 38,

    #[doc = "Channel 39"]
    Ch39 = 39,
}

/// 36 of the 39 RF channels used for initial advertising and all legacy advertising activitie.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataChannel {
    #[doc = "Channel 0"]
    Ch0 = 0,

    #[doc = "Channel 1"]
    Ch1 = 1,

    #[doc = "Channel 2"]
    Ch2 = 2,

    #[doc = "Channel 3"]
    Ch3 = 3,

    #[doc = "Channel 4"]
    Ch4 = 4,

    #[doc = "Channel 5"]
    Ch5 = 5,

    #[doc = "Channel 6"]
    Ch6 = 6,

    #[doc = "Channel 7"]
    Ch7 = 7,

    #[doc = "Channel 8"]
    Ch8 = 8,

    #[doc = "Channel 9"]
    Ch9 = 9,

    #[doc = "Channel 10"]
    Ch10 = 10,

    #[doc = "Channel 11"]
    Ch11 = 11,

    #[doc = "Channel 12"]
    Ch12 = 12,

    #[doc = "Channel 13"]
    Ch13 = 13,

    #[doc = "Channel 14"]
    Ch14 = 14,

    #[doc = "Channel 15"]
    Ch15 = 15,

    #[doc = "Channel 16"]
    Ch16 = 16,

    #[doc = "Channel 17"]
    Ch17 = 17,

    #[doc = "Channel 18"]
    Ch18 = 18,

    #[doc = "Channel 19"]
    Ch19 = 19,

    #[doc = "Channel 20"]
    Ch20 = 20,

    #[doc = "Channel 21"]
    Ch21 = 21,

    #[doc = "Channel 22"]
    Ch22 = 22,

    #[doc = "Channel 23"]
    Ch23 = 23,

    #[doc = "Channel 24"]
    Ch24 = 24,

    #[doc = "Channel 25"]
    Ch25 = 25,

    #[doc = "Channel 26"]
    Ch26 = 26,

    #[doc = "Channel 27"]
    Ch27 = 27,

    #[doc = "Channel 28"]
    Ch28 = 28,

    #[doc = "Channel 29"]
    Ch29 = 29,

    #[doc = "Channel 30"]
    Ch30 = 30,

    #[doc = "Channel 31"]
    Ch31 = 31,

    #[doc = "Channel 32"]
    Ch32 = 32,

    #[doc = "Channel 33"]
    Ch33 = 33,

    #[doc = "Channel 34"]
    Ch34 = 34,

    #[doc = "Channel 35"]
    Ch35 = 35,

    #[doc = "Channel 36"]
    Ch36 = 36,
}

impl ChannelTrait for Channel {
    fn channel_index(&self) -> u8 {
        match self {
            Channel::Advertising(channel) => channel.channel_index(),
            Channel::Data(channel) => channel.channel_index(),
        }
    }

    fn physical_index(&self) -> u8 {
        match self {
            Channel::Advertising(channel) => channel.physical_index(),
            Channel::Data(channel) => channel.physical_index(),
        }
    }
}

impl AdvertisingChannel {
    pub fn channels() -> impl Iterator<Item = Self> {
        [Self::Ch37, Self::Ch38, Self::Ch39].iter().copied()
    }
}

impl ChannelTrait for AdvertisingChannel {
    // https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-8d4b6daf-4142-e928-81d1-520529d8277f
    fn channel_index(&self) -> u8 {
        *self as u8
    }

    fn physical_index(&self) -> u8 {
        match self {
            AdvertisingChannel::Ch37 => 0,
            AdvertisingChannel::Ch38 => 12,
            AdvertisingChannel::Ch39 => 39,
        }
    }
}

impl ChannelTrait for DataChannel {
    fn channel_index(&self) -> u8 {
        *self as u8
    }

    fn physical_index(&self) -> u8 {
        match self {
            // AdvertisingChannel::Ch37 => 0,
            DataChannel::Ch0 => 1,
            DataChannel::Ch1 => 2,
            DataChannel::Ch2 => 3,
            DataChannel::Ch3 => 4,
            DataChannel::Ch4 => 5,
            DataChannel::Ch5 => 6,
            DataChannel::Ch6 => 7,
            DataChannel::Ch7 => 8,
            DataChannel::Ch8 => 9,
            DataChannel::Ch9 => 10,
            DataChannel::Ch10 => 11,
            // AdvertisingChannel::Ch37 => 12,
            DataChannel::Ch11 => 13,
            DataChannel::Ch12 => 14,
            DataChannel::Ch13 => 15,
            DataChannel::Ch14 => 16,
            DataChannel::Ch15 => 17,
            DataChannel::Ch16 => 18,
            DataChannel::Ch17 => 19,
            DataChannel::Ch18 => 20,
            DataChannel::Ch19 => 21,
            DataChannel::Ch20 => 22,
            DataChannel::Ch21 => 23,
            DataChannel::Ch22 => 24,
            DataChannel::Ch23 => 25,
            DataChannel::Ch24 => 26,
            DataChannel::Ch25 => 27,
            DataChannel::Ch26 => 28,
            DataChannel::Ch27 => 29,
            DataChannel::Ch28 => 30,
            DataChannel::Ch29 => 31,
            DataChannel::Ch30 => 32,
            DataChannel::Ch31 => 33,
            DataChannel::Ch32 => 34,
            DataChannel::Ch33 => 35,
            DataChannel::Ch34 => 36,
            DataChannel::Ch35 => 37,
            DataChannel::Ch36 => 38,
            // AdvertisingChannel::Ch39 => 39,
        }
    }
}

impl TryFrom<u8> for Channel {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if let Ok(channel) = AdvertisingChannel::try_from(value) {
            Ok(Self::Advertising(channel))
        } else if let Ok(channel) = DataChannel::try_from(value) {
            Ok(Self::Data(channel))
        } else {
            Err(())
        }
    }
}

impl From<AdvertisingChannel> for Channel {
    fn from(channel: AdvertisingChannel) -> Self {
        Self::Advertising(channel)
    }
}

impl From<DataChannel> for Channel {
    fn from(channel: DataChannel) -> Self {
        Self::Data(channel)
    }
}

impl TryFrom<u8> for AdvertisingChannel {
    type Error = ();

    /// From physical index
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            37 => Ok(Self::Ch37),
            38 => Ok(Self::Ch38),
            39 => Ok(Self::Ch39),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for DataChannel {
    type Error = ();

    /// From channel index
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Ch0),
            1 => Ok(Self::Ch1),
            2 => Ok(Self::Ch2),
            3 => Ok(Self::Ch3),
            4 => Ok(Self::Ch4),
            5 => Ok(Self::Ch5),
            6 => Ok(Self::Ch6),
            7 => Ok(Self::Ch7),
            8 => Ok(Self::Ch8),
            9 => Ok(Self::Ch9),
            10 => Ok(Self::Ch10),
            11 => Ok(Self::Ch11),
            12 => Ok(Self::Ch12),
            13 => Ok(Self::Ch13),
            14 => Ok(Self::Ch14),
            15 => Ok(Self::Ch15),
            16 => Ok(Self::Ch16),
            17 => Ok(Self::Ch17),
            18 => Ok(Self::Ch18),
            19 => Ok(Self::Ch19),
            20 => Ok(Self::Ch20),
            21 => Ok(Self::Ch21),
            22 => Ok(Self::Ch22),
            23 => Ok(Self::Ch23),
            24 => Ok(Self::Ch24),
            25 => Ok(Self::Ch25),
            26 => Ok(Self::Ch26),
            27 => Ok(Self::Ch27),
            28 => Ok(Self::Ch28),
            29 => Ok(Self::Ch29),
            30 => Ok(Self::Ch30),
            31 => Ok(Self::Ch31),
            32 => Ok(Self::Ch32),
            33 => Ok(Self::Ch33),
            34 => Ok(Self::Ch34),
            35 => Ok(Self::Ch35),
            36 => Ok(Self::Ch36),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_central_frequency() {
        assert_eq!(AdvertisingChannel::Ch37.central_frequency(), 2402);
        assert_eq!(DataChannel::Ch0.central_frequency(), 2404);
        assert_eq!(DataChannel::Ch1.central_frequency(), 2406);
        assert_eq!(DataChannel::Ch2.central_frequency(), 2408);
        assert_eq!(DataChannel::Ch3.central_frequency(), 2410);
        assert_eq!(DataChannel::Ch4.central_frequency(), 2412);
        assert_eq!(DataChannel::Ch5.central_frequency(), 2414);
        assert_eq!(DataChannel::Ch6.central_frequency(), 2416);
        assert_eq!(DataChannel::Ch7.central_frequency(), 2418);
        assert_eq!(DataChannel::Ch8.central_frequency(), 2420);
        assert_eq!(DataChannel::Ch9.central_frequency(), 2422);
        assert_eq!(DataChannel::Ch10.central_frequency(), 2424);
        assert_eq!(AdvertisingChannel::Ch38.central_frequency(), 2426);
        assert_eq!(DataChannel::Ch11.central_frequency(), 2428);
        assert_eq!(DataChannel::Ch12.central_frequency(), 2430);
        assert_eq!(DataChannel::Ch13.central_frequency(), 2432);
        assert_eq!(DataChannel::Ch14.central_frequency(), 2434);
        assert_eq!(DataChannel::Ch15.central_frequency(), 2436);
        assert_eq!(DataChannel::Ch16.central_frequency(), 2438);
        assert_eq!(DataChannel::Ch17.central_frequency(), 2440);
        assert_eq!(DataChannel::Ch18.central_frequency(), 2442);
        assert_eq!(DataChannel::Ch19.central_frequency(), 2444);
        assert_eq!(DataChannel::Ch20.central_frequency(), 2446);
        assert_eq!(DataChannel::Ch21.central_frequency(), 2448);
        assert_eq!(DataChannel::Ch22.central_frequency(), 2450);
        assert_eq!(DataChannel::Ch23.central_frequency(), 2452);
        assert_eq!(DataChannel::Ch24.central_frequency(), 2454);
        assert_eq!(DataChannel::Ch25.central_frequency(), 2456);
        assert_eq!(DataChannel::Ch26.central_frequency(), 2458);
        assert_eq!(DataChannel::Ch27.central_frequency(), 2460);
        assert_eq!(DataChannel::Ch28.central_frequency(), 2462);
        assert_eq!(DataChannel::Ch29.central_frequency(), 2464);
        assert_eq!(DataChannel::Ch30.central_frequency(), 2466);
        assert_eq!(DataChannel::Ch31.central_frequency(), 2468);
        assert_eq!(DataChannel::Ch32.central_frequency(), 2470);
        assert_eq!(DataChannel::Ch33.central_frequency(), 2472);
        assert_eq!(DataChannel::Ch34.central_frequency(), 2474);
        assert_eq!(DataChannel::Ch35.central_frequency(), 2476);
        assert_eq!(DataChannel::Ch36.central_frequency(), 2478);
        assert_eq!(AdvertisingChannel::Ch39.central_frequency(), 2480);
    }

    #[test]
    fn test_whitening_init() {
        assert_eq!(DataChannel::Ch0.whitening_init(), 0b0100_0000);
        assert_eq!(AdvertisingChannel::Ch37.whitening_init(), 0b0110_0101);
        assert_eq!(AdvertisingChannel::Ch38.whitening_init(), 0b0110_0110);
        assert_eq!(AdvertisingChannel::Ch39.whitening_init(), 0b0110_0111);
    }
}
