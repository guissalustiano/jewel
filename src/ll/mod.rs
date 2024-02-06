mod address;
mod adv;

use core::time::Duration;

pub use address::*;
pub use adv::*;

///  Inter Frame Space
///  The time interval between two consecutive packets on the same channel index
///  It is defined as the time from the end of the last bit of the previous packet to the start of the first bit of the subsequent packet.
///
///  Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-adf2f32c-5470-6d89-daf1-0a42b657da75
const T_IFS: Duration = Duration::from_micros(150);

/// Minimum AUX Frame Space
/// The minimum time interval between a packet containing an AuxPtr and the auxiliary packet it indicates.
/// It is defined as the minimum time from the end of the last bit of the packet containing the AuxPtr to the start of the auxiliary packet.
///
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-76fbe828-b8f7-12e4-8de2-223c867e4a2a
const T_MAFS: Duration = Duration::from_micros(300);

/// Minimum Subevent Space
/// The minimum time interval between the end of the last bit of the last packet in one subevent
/// and the start of the first bit of the first packet in the next subevent.
///
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-ea6717b6-1fb3-c5ec-9153-04e4b5ee20fb
const T_MSS: Duration = Duration::from_micros(150);

// TODO: Implement clock accuracy based on the receiver's clock accuracy and jitter.
// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-1cdb9b08-1996-f9bd-9dd5-9587794799b1

/// Active clock accuracy
/// The average timing of packet transmission during a connection, BIG, or CIG event, during active scanning, during a periodic advertising with responses subevent, and when requesting a connection
///
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-1cdb9b08-1996-f9bd-9dd5-9587794799b1
const T_ACA: Duration = Duration::from_micros(2); // less than or equal to ±50 ppm

/// Sleep clock accuracy
/// The max worst-case drift and instantaneos deviataion timing for all other activities
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-4a9f77e1-d1e1-dfe1-1181-032ae1feb03e
const T_SCA: Duration = Duration::from_micros(20); // less than or equal to ±500 ppm

// Guessing a reasonable propagation distance
const PROPAGATION_DISTANCE: u64 = 10; // meters

/// Range delay
/// Where two devices are more than a few meters apart the time taken for a signal to propagate between them will be significant compared with the Active Clock Accuracy
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-e16c5296-3b60-01b4-3251-a8f289f1cdb2
const RANGE_DELAY: Duration = Duration::from_nanos(2 * PROPAGATION_DISTANCE * 4);

// TODO: Implement Window widening
// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-fed93539-5fa3-b4de-4789-1b8a1b48fa13

// pattern from https://hoverbear.org/blog/rust-state-machine-pattern/
pub struct LinkLayer<S = Standby> {
    state: S,
}

impl LinkLayer<Standby> {
    pub fn new() -> Self {
        LinkLayer { state: Standby {} }
    }
}

pub struct Standby {}
pub struct Advertising {
    // Pseudo-random value used to generate the advDelay between each advertising event
    seed: u64,
}
