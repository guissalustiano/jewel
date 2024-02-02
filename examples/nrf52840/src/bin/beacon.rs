#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_nrf::{bind_interrupts, peripherals, radio};
use embassy_time::Timer;
use jewel::{
    ll::{Address, AdvNonconnInd},
    phy::{BleRadio, Packet},
};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    RADIO => radio::InterruptHandler<peripherals::RADIO>;
});

// Same payload as the embassy/nrf-softdevice ble_advertising example,
// but just in channel 39.
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_nrf::config::Config::default();
    config.hfclk_source = embassy_nrf::config::HfclkSource::ExternalXtal;
    let p = embassy_nrf::init(config);

    info!("Starting BLE radio");
    let mut radio = radio::ble::Radio::new(p.RADIO, Irqs);

    let body = [
        0x02, 0x01, 0x06, // Flags
        0x03, 0x03, 0x09, 0x18, // Complete list of 16-bit UUIDs available
        0x0A, 0x09, // Length, Type: Device name
        b'H', b'e', b'l', b'l', b'o', b'R', b'u', b's', b't',
    ];

    let pdu = AdvNonconnInd::new(Address::new_random(0xffe1e8d0dc27), &body);
    let data = pdu.transmission_bytes();

    info!("{:?}", data);
    unwrap!(radio.set_buffer(data.as_ref()));

    loop {
        info!("Sending packet");
        radio.transmit().await;
        Timer::after_millis(500).await;
    }
}
