# Jewel
BLE for bare-metal, real-time system.

Work in progress.

## Roadmap
First implement the Bluetooth 4.2, then 5.X, Mesh

- [X] Physical
    - [X] [Implement RADIO driver on embassy nrf hal](https://github.com/embassy-rs/embassy/pull/2351) 
- [ ] Air interface
    - [X] Address
    - [ ] Advertising PDUs
        - [X] ADV_IND
        - [X] ADV_DIRECT_IND
        - [X] ADV_NONCONN_IND
        - [X] ADV_SCAN_IND
        - [ ] ADV_EXT_IND
        - [ ] AUX_ADV_IND
        - [ ] AUX_SYNC_IND
        - [ ] AUX_CHAIN_IND
        - [ ] AUX_SYNC_SUBEVENT_IND
        - [ ] AUX_SYNC_SUBEVENT_RSP
    - [ ] Scan PDUs
        - [X] SCAN_REQ
        - [ ] SCAN_RSP
        - [ ] AUX_SCAN_REQ
        - [ ] AUX_SCAN_RSP
    - [ ] Initiating PDUs
        - [ ] CONNECT_IND
        - [ ] AUX_CONNECT_REQ
        - [ ] AUX_CONNECT_RSP
- [ ] Air Interface protocol
    - [ ] Timing
    - [ ] Device filtering
- [ ] L2CP
- [ ] GATT/ATT
    - [ ] Adv structure
    - [ ] GATT server
    - [ ] GATT client
- [ ] Multiple state machine suport
- [ ] Mesh
