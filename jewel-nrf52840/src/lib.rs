#![no_std]

use embassy_nrf::radio::{
    ble::{Error, Mode, Radio as NrfRadio, TxPower},
    Instance,
};
use jewel::phy::{ChannelTrait, Radio, CRC_POLY};

pub struct RadioImpl<'d, T: Instance> {
    radio: NrfRadio<'d, T>,
}

// From 5.4.1 of the nRF52840 Product Specification:
// > The HFXO must be running to use the RADIO or  the calibration mechanism associated with the 32.768 kHz RC oscillator.
// Currently the jewel crate don't implement the calibration mechanism, so we need to ensure that the HFXO is running
impl<'d, T: Instance> From<NrfRadio<'d, T>> for RadioImpl<'d, T> {
    fn from(radio: NrfRadio<'d, T>) -> Self {
        RadioImpl { radio }
    }
}

impl<'d, T: Instance> Radio for RadioImpl<'d, T> {
    type Error = Error;

    fn set_mode(&mut self, mode: jewel::phy::Mode) {
        let embassy_mode = match mode {
            jewel::phy::Mode::Ble1mbit => Mode::BLE_1MBIT,
        };

        self.radio.set_mode(embassy_mode);
    }

    fn set_tx_power(&mut self, power_db: i8) {
        let tx_power: TxPower = match power_db {
            8..=i8::MAX => TxPower::POS8D_BM,
            7 => TxPower::POS7D_BM,
            6 => TxPower::POS6D_BM,
            5 => TxPower::POS5D_BM,
            4 => TxPower::POS4D_BM,
            3 => TxPower::POS3D_BM,
            1..=2 => TxPower::POS2D_BM,
            -3..=0 => TxPower::_0D_BM,
            -7..=-4 => TxPower::NEG4D_BM,
            -11..=-8 => TxPower::NEG8D_BM,
            -15..=-12 => TxPower::NEG12D_BM,
            -19..=-16 => TxPower::NEG16D_BM,
            -29..=-20 => TxPower::NEG20D_BM,
            -39..=-30 => TxPower::NEG30D_BM,
            i8::MIN..=-40 => TxPower::NEG40D_BM,
        };

        self.radio.set_tx_power(tx_power)
    }

    fn set_header_size(&mut self, header_size: jewel::phy::HeaderSize) {
        let use_s1 = match header_size {
            jewel::phy::HeaderSize::TwoBytes => false,
            jewel::phy::HeaderSize::ThreeBytes => true,
        };
        self.radio.set_header_expansion(use_s1)
    }

    fn set_access_address(&mut self, access_address: u32) {
        self.radio.set_access_address(access_address)
    }

    fn set_channel(&mut self, channel: jewel::phy::Channel) {
        self.radio.set_frequency(channel.central_frequency().into());
        self.radio.set_whitening_init(channel.whitening_init());
    }

    fn set_crc_init(&mut self, crc_init: u32) {
        self.radio.set_crc_poly(CRC_POLY);
        self.radio.set_crc_init(crc_init)
    }

    fn set_buffer(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        self.radio.set_buffer(buffer)
    }

    async fn transmit(&mut self) {
        self.radio.transmit().await
    }

    async fn receive(&mut self) {
        self.radio.receive().await
    }
}
