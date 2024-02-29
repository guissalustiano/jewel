//! Generic acess profile

mod adv_struct;

pub use adv_struct::*;
use embassy_time::Duration;
use rand::rngs::SmallRng;

use crate::{
    ll::{Address, AddressAndData, AdvNonconnInd, Advertising, LinkLayer},
    phy::{Radio, MAX_PDU_LENGTH},
};

pub struct Broadcaster<'r, 'a, R: Radio> {
    ll: LinkLayer<'r, R, Advertising<'a, SmallRng>>,

    // you should never access this buffer directly
    // it's only used to keep borrow of the buffer
    #[allow(dead_code)]
    _buffer: &'a [u8],
}

/// Brodcast provile. Advertise legacy packages on the 3 primary advertising channels.
/// ```no_run
/// #![no_std]
/// #![no_main]
///
/// use defmt::info;
/// use embassy_executor::Spawner;
/// use embassy_nrf::{bind_interrupts, peripherals, radio};
/// use embassy_time::Duration;
/// use jewel::{Address, AdvData, Broadcaster, Flags};
/// use {defmt_rtt as _, panic_probe as _};
///
/// bind_interrupts!(struct Irqs {
///     RADIO => radio::InterruptHandler<peripherals::RADIO>;
/// });
///
/// #[embassy_executor::main]
/// async fn main(_spawner: Spawner) {
///     let mut config = embassy_nrf::config::Config::default();
///     config.hfclk_source = embassy_nrf::config::HfclkSource::ExternalXtal;
///     let p = embassy_nrf::init(config);
///
///     info!("Starting BLE radio");
///     let mut radio = radio::ble::Radio::new(p.RADIO, Irqs);
///     let mut broadcaster = Broadcaster::new(
///         &mut radio,
///         Duration::from_millis(300),
///         Address::new_random(0xffe1e8d0dc27),
///         AdvData::empty()
///             .set_flags(Flags::discoverable())
///             .set_uuids16(&[0x0918])
///             .set_complete_local_name("HelloRust"),
///     )
///     .unwrap();
///
///     loop {
///         info!("Sending packet");
///         broadcaster.transmit().await;
///     }
/// }
/// ```
impl<'r, 'a, R: Radio> Broadcaster<'r, 'a, R> {
    /// Create a new broadcaster
    pub fn new(
        radio: &'r mut R,
        interval: Duration,
        address: Address,
        data: AdvData<'a>,
        buffer: &'a mut [u8; MAX_PDU_LENGTH],
    ) -> Result<Broadcaster<'r, 'a, R>, R::Error> {
        let mut body_buffer = [0u8; MAX_PDU_LENGTH];
        let len = data.bytes(&mut body_buffer);

        let pdu = AdvNonconnInd::new(address, &body_buffer[..len]);

        pdu.bytes(buffer);

        let ll = LinkLayer::new(radio);

        // FIXME: it should be a better way to do this
        let ll = ll.advertise(interval, buffer);

        Ok(Broadcaster {
            ll,
            _buffer: buffer,
        })
    }

    /// Transmit the packet respecting the given interval
    /// You should call this method before the interval time
    pub async fn transmit(&mut self) -> Result<(), R::Error> {
        self.ll.transmit().await
    }
}
