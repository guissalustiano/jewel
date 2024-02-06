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
    // it's only used to keep the pdu alive
    #[allow(dead_code)]
    _buffer: [u8; MAX_PDU_LENGTH],
}

impl<'r, 'a, R: Radio> Broadcaster<'r, 'a, R> {
    pub fn new(
        radio: &'r mut R,
        interval: Duration,
        address: Address,
        data: AdvData<'a>,
    ) -> Result<Broadcaster<'r, 'a, R>, R::Error> {
        let mut body_buffer = [0u8; MAX_PDU_LENGTH];
        let len = data.bytes(&mut body_buffer);

        let pdu = AdvNonconnInd::new(address, &body_buffer[..len]);

        let mut buffer = [0u8; MAX_PDU_LENGTH];
        pdu.bytes(&mut buffer);

        let ll = LinkLayer::new(radio);

        // FIXME: it should be a better way to do this
        let ll = ll.advertise(interval, unsafe {
            &*(&buffer as *const [u8; MAX_PDU_LENGTH])
        })?;

        Ok(Broadcaster {
            ll,
            _buffer: buffer,
        })
    }

    pub async fn transmit(&mut self) {
        self.ll.transmit().await;
    }
}
