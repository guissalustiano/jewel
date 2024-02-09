//! Advertising physical channel PDU
//!
//!    LSB                                                          MSB
//!   ┌──────────┬──────────┬─────────┬──────────┬──────────┬──────────┬────────────────────────────────────────┐
//!   │ PDU Type │ RFU      │ ChSel   │ TxAdd    │ RxAdd    │ Length   │                                        │
//!   │ (4 bits) │ (1 bit)  │ (1 bit) │ (1 bit)  │ (1 bit)  │ (8 bits) │                                        │
//!   ├──────────┴──────────┴─────────┴──────────┴──────────┴──────────┼────────────────────────────────────────┤
//!   │                        HEADER (16 bits)                        │           Payload (1-255 bytes)        │
//!   └────────────────────────────────────────────────────────────────┴────────────────────────────────────────┘
//!
//! The ChSel, TxAdd and RxAdd fields of the advertising physical
//! channel PDU that are contained in the header contain information
//! specific to the PDU type defined for each advertising physical
//! channel PDU separately (represented by X).
//!
//! If the ChSel, TxAdd or RxAdd fields are not defined as used in a
//! given PDU then they shall be considered reserved for future use.
//! (represented by -)
//!
//! Ref: [Core 6.B.2.3](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-970b9251-9089-5ea4-1634-41defd816278)

mod header;
mod pdu;

pub use header::*;
pub use pdu::*;
