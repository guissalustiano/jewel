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
}

impl<'r, 'a, R: Radio> Broadcaster<'r, 'a, R> {
    pub fn new(
        radio: &'r mut R,
        interval: Duration,
        address: Address,
        data: AdvData<'a>,
        buffer: &'a mut [u8; MAX_PDU_LENGTH], // TODO: remove this
    ) -> Result<Broadcaster<'r, 'a, R>, R::Error> {
        let mut body_buffer = [0u8; MAX_PDU_LENGTH];
        let len = data.bytes(&mut body_buffer);

        let pdu = AdvNonconnInd::new(address, &body_buffer[..len]);

        pdu.bytes(buffer);

        let ll = LinkLayer::new(radio);
        let ll = ll.advertise(interval, buffer)?;

        Ok(Broadcaster { ll })
    }

    pub async fn transmit(&mut self) {
        self.ll.transmit().await;
    }
}
