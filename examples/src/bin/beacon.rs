#![no_std]
#![no_main]

use core::mem;
use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::{bind_interrupts, peripherals, radio};
use embassy_time::Duration;
use jewel::{phy::MAX_PDU_LENGTH, Address, AdvData, Broadcaster, Flags};
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

    let ficr: embassy_nrf::pac::FICR = unsafe { mem::transmute(()) };
    let device_address_public = ficr.deviceaddrtype.read().deviceaddrtype().is_public();
    let device_address = u64::from(ficr.deviceaddr[0].read().bits()) | u64::from(ficr.deviceaddr[1].read().bits());
    let device_address = if device_address_public { Address::new_public(device_address) } else { Address::new_random(device_address) };

    let mut buffer = [0u8; MAX_PDU_LENGTH];

    info!("Starting BLE radio");
    let mut radio: RadioImpl<'_, _> = radio::ble::Radio::new(p.RADIO, Irqs).into();
    let mut broadcaster = Broadcaster::new(
        &mut radio,
        Duration::from_millis(300),
        device_address,
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
