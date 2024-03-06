#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::{bind_interrupts, peripherals, radio};
use embassy_time::Duration;
use jewel::{phy::MAX_PDU_LENGTH, AdvData, Broadcaster, Flags};
use jewel_nrf::RadioImpl;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    RADIO => radio::InterruptHandler<peripherals::RADIO>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_nrf::config::Config::default();
    config.hfclk_source = embassy_nrf::config::HfclkSource::ExternalXtal;
    let p = embassy_nrf::init(config);

    let mut buffer = [0u8; MAX_PDU_LENGTH];

    info!("Starting BLE radio");
    let mut radio: RadioImpl<'_, _> = radio::ble::Radio::new(p.RADIO, Irqs).into();

    let mut broadcaster = Broadcaster::new(
        &mut radio,
        Duration::from_millis(300),
        AdvData::empty()
            .set_flags(Flags::discoverable())
            .set_uuids16(&[0x0918])
            .set_complete_local_name("HelloRust"),
        &mut buffer,
    )
    .unwrap();

    loop {
        info!("Sending packet");
        broadcaster.transmit().await.unwrap();
    }
}
