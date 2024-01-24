#![no_std]

pub mod adv_pdu;
pub mod channel;

/// BLE advertising address for 4.* advertising packets
///
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-ddca5d1e-d894-5b28-4322-94b1c69bef07
pub const ADV_ADDRESS: u32 = 0x8E_89_BE_D6;

/// BLE advertising CRC initial value, 24 bits
/// For all other Advertising Physical Ch PDUs, the shift register shall be preset with 0x555555
///
/// Ref: https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-ef2b6d91-cee4-fb69-3b93-c1a5948aadae
pub const ADV_CRC_INIT: u32 = 0x555555;
