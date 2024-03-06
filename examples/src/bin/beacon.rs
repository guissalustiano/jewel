#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::{bind_interrupts, peripherals, radio};
use embassy_time::Duration;
use jewel::{ll::LinkLayer, phy::MAX_PDU_LENGTH, AdvData, Flags};
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

    info!("Starting BLE radio");
    let mut radio: RadioImpl<'_, _> = radio::ble::Radio::new(p.RADIO, Irqs).into();

    let adv_data = AdvData::empty()
        .set_flags(Flags::discoverable())
        .set_uuids16(&[0x0918])
        .set_shortened_local_name("Hello");

    let mut adv_data_buffer = [0u8; MAX_PDU_LENGTH];
    let len = adv_data.bytes(&mut adv_data_buffer);
    let adv_data_buffer = &adv_data_buffer[..len];

    let scan_data = AdvData::empty().set_complete_local_name("HelloRust");

    let mut scan_data_buffer = [0u8; MAX_PDU_LENGTH];
    let len = scan_data.bytes(&mut scan_data_buffer);
    let scan_data_buffer = &scan_data_buffer[..len];

    let ll = LinkLayer::new(&mut radio);

    ll.adv_nonconnectable_scannable_undirected(
        Duration::from_millis(300),
        adv_data_buffer,
        scan_data_buffer,
    )
    .await
    .unwrap();
}
