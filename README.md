# Jewel
BLE for bare-metal, real-time system.

Work in progress.

## Roadmap
- [X] [Implement RADIO driver on embassy nrf hal](https://github.com/embassy-rs/embassy/pull/2351) 
- [ ] Beacon (Advertizor peripheral)
    - [X] ADV_NONCONN_IND package
    - [ ] adv structure
    - [ ] state machine with time contraints
- [ ] Scanabble peripheral
  - [ ] ADV_SCAN_IND, SCAN_REQ, SCAN_RSP (AUX_SCAN_­REQ, AUX_SCAN_­RSP)
  - [ ] state machine with time constraints
- [ ] Connectable periperal
  - [ ] ADV_IND, ADV_DIRECT_IND, CONNECT_IND, AUX_CONNECT_REQ, AUX_CONNECT_RSP
  - [ ] plain data PDU
  - [ ] GATT
  - [ ] state machine
- [ ] Central scanner
- [ ] Central connector
- [ ] Data contoller
- [ ] LL multiple state machine suport
