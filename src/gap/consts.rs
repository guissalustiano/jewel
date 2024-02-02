//! Assigned numbers
//!
//! Ref: https://www.bluetooth.com/specifications/assigned-numbers/

pub mod ad_types {
    /// Incomplete List of 16-bit Service Class UUIDs
    /// Core Specification Supplement, Part A, Section 1.1
    const INCOMPLETE_LIST_OF_16_BIT_SERVICE_CLASS_UUIDS: u8 = 0x02;

    /// Complete List of 16-bit Service Class UUIDs
    /// Core Specification Supplement, Part A, Section 1.1
    const COMPLETE_LIST_OF_16_BIT_SERVICE_CLASS_UUIDS: u8 = 0x03;

    /// Incomplete List of 32-bit Service Class UUIDs
    /// Core Specification Supplement, Part A, Section 1.1
    const INCOMPLETE_LIST_OF_32_BIT_SERVICE_CLASS_UUIDS: u8 = 0x04;

    /// Complete List of 32-bit Service Class UUIDs
    /// Core Specification Supplement, Part A, Section 1.1
    const COMPLETE_LIST_OF_32_BIT_SERVICE_CLASS_UUIDS: u8 = 0x05;

    /// Incomplete List of 128-bit Service Class UUIDs
    /// Core Specification Supplement, Part A, Section 1.1
    const INCOMPLETE_LIST_OF_128_BIT_SERVICE_CLASS_UUIDS: u8 = 0x06;

    /// Complete List of 128-bit Service Class UUIDs
    /// Core Specification Supplement, Part A, Section 1.1
    const COMPLETE_LIST_OF_128_BIT_SERVICE_CLASS_UUIDS: u8 = 0x07;

    /// Shortened Local Name
    /// Core Specification Supplement, Part A, Section 1.2
    const SHORTENED_LOCAL_NAME: u8 = 0x08;

    /// Complete Local Name
    /// Core Specification Supplement, Part A, Section 1.2
    const COMPLETE_LOCAL_NAME: u8 = 0x09;

    /// Tx Power Level
    /// Core Specification Supplement, Part A, Section 1.5
    const TX_POWER_LEVEL: u8 = 0x0A;

    /// Class of Device
    /// Core Specification Supplement, Part A, Section 1.6
    const CLASS_OF_DEVICE: u8 = 0x0D;

    /// Simple Pairing Hash C-192
    /// Core Specification Supplement, Part A, Section 1.6
    const SIMPLE_PAIRING_HASH_C_192: u8 = 0x0E;

    /// Simple Pairing Randomizer R-192
    /// Core Specification Supplement, Part A, Section 1.6
    const SIMPLE_PAIRING_RANDOMIZER_R_192: u8 = 0x0F;

    /// Device ID
    /// Device ID Profile
    const DEVICE_ID: u8 = 0x10;

    /// Security Manager TK Value
    /// Core Specification Supplement, Part A, Section 1.8
    const SECURITY_MANAGER_TK_VALUE: u8 = 0x10;

    /// Security Manager Out of Band Flags
    /// Core Specification Supplement, Part A, Section 1.7
    const SECURITY_MANAGER_OUT_OF_BAND_FLAGS: u8 = 0x11;

    /// Peripheral Connection Interval Range
    /// Core Specification Supplement, Part A, Section 1.9
    const PERIPHERAL_CONNECTION_INTERVAL_RANGE: u8 = 0x12;

    /// List of 16-bit Service Solicitation UUIDs
    /// Core Specification Supplement, Part A, Section 1.10
    const LIST_OF_16_BIT_SERVICE_SOLICITATION_UUIDS: u8 = 0x14;

    /// List of 128-bit Service Solicitation UUIDs
    /// Core Specification Supplement, Part A, Section 1.10
    const LIST_OF_128_BIT_SERVICE_SOLICITATION_UUIDS: u8 = 0x15;

    /// Service Data - 16-bit UUID
    /// Core Specification Supplement, Part A, Section 1.11
    const SERVICE_DATA__16_BIT_UUID: u8 = 0x16;

    /// Public Target Address
    /// Core Specification Supplement, Part A, Section 1.13
    const PUBLIC_TARGET_ADDRESS: u8 = 0x17;

    /// Random Target Address
    /// Core Specification Supplement, Part A, Section 1.14
    const RANDOM_TARGET_ADDRESS: u8 = 0x18;

    /// Appearance
    /// Core Specification Supplement, Part A, Section 1.12
    const APPEARANCE: u8 = 0x19;

    /// Advertising Interval
    /// Core Specification Supplement, Part A, Section 1.15
    const ADVERTISING_INTERVAL: u8 = 0x1A;

    /// LE Bluetooth Device Address
    /// Core Specification Supplement, Part A, Section 1.16
    const LE_BLUETOOTH_DEVICE_ADDRESS: u8 = 0x1B;

    /// LE Role
    /// Core Specification Supplement, Part A, Section 1.17
    const LE_ROLE: u8 = 0x1C;

    /// Simple Pairing Hash C-256
    /// Core Specification Supplement, Part A, Section 1.6
    const SIMPLE_PAIRING_HASH_C_256: u8 = 0x1D;

    /// Simple Pairing Randomizer R-256
    /// Core Specification Supplement, Part A, Section 1.6
    const SIMPLE_PAIRING_RANDOMIZER_R_256: u8 = 0x1E;

    /// List of 32-bit Service Solicitation UUIDs
    /// Core Specification Supplement, Part A, Section 1.10
    const LIST_OF_32_BIT_SERVICE_SOLICITATION_UUIDS: u8 = 0x1F;

    /// Service Data - 32-bit UUID
    /// Core Specification Supplement, Part A, Section 1.11
    const SERVICE_DATA__32_BIT_UUID: u8 = 0x20;

    /// Service Data - 128-bit UUID
    /// Core Specification Supplement, Part A, Section 1.11
    const SERVICE_DATA__128_BIT_UUID: u8 = 0x21;

    /// LE Secure Connections Confirmation Value
    /// Core Specification Supplement, Part A, Section 1.6
    const LE_SECURE_CONNECTIONS_CONFIRMATION_VALUE: u8 = 0x22;

    /// LE Secure Connections Random Value
    /// Core Specification Supplement, Part A, Section 1.6
    const LE_SECURE_CONNECTIONS_RANDOM_VALUE: u8 = 0x23;

    /// URI
    /// Core Specification Supplement, Part A, Section 1.18
    const URI: u8 = 0x24;

    /// Indoor Positioning
    /// Indoor Positioning Service
    const INDOOR_POSITIONING: u8 = 0x25;

    /// Transport Discovery Data
    /// Transport Discovery Service
    const TRANSPORT_DISCOVERY_DATA: u8 = 0x26;

    /// LE Supported Features
    /// Core Specification Supplement, Part A, Section 1.19
    const LE_SUPPORTED_FEATURES: u8 = 0x27;

    /// Channel Map Update Indication
    /// Core Specification Supplement, Part A, Section 1.20
    const CHANNEL_MAP_UPDATE_INDICATION: u8 = 0x28;

    /// PB-ADV
    /// Mesh Profile Specification, Section 5.2.1
    const PB_ADV: u8 = 0x29;
    /// Mesh Message
    /// Mesh Profile Specification, Section 3.3.1
    const MESH_MESSAGE: u8 = 0x2A;

    /// Mesh Beacon
    /// Mesh Profile Specification, Section 3.9
    const MESH_BEACON: u8 = 0x2B;

    /// BIGInfo
    /// Core Specification Supplement, Part A, Section 1.21
    const BIGInfo: u8 = 0x2C;

    /// Broadcast_Code
    /// Core Specification Supplement, Part A, Section 1.22
    const BROADCAST_CODE: u8 = 0x2D;

    /// Resolvable Set Identifier
    /// Coordinated Set Identification Profile v1.0 or later
    const RESOLVABLE_SET_IDENTIFIER: u8 = 0x2E;

    /// Advertising Interval - long
    /// Core Specification Supplement, Part A, Section 1.15
    const ADVERTISING_INTERVAL__LONG: u8 = 0x2F;

    /// Broadcast_Name
    /// Public Broadcast Profile v1.0 or later
    const BROADCAST_NAME: u8 = 0x30;

    /// Encrypted Advertising Data
    /// Core Specification Supplement, Part A, Section 1.23
    const ENCRYPTED_ADVERTISING_DATA: u8 = 0x31;

    /// Periodic Advertising Response Timing Information
    /// Core Specification Supplement, Part A, Section 1.24
    const PERIODIC_ADVERTISING_RESPONSE_TIMING_INFORMATION: u8 = 0x32;

    /// Electronic Shelf Label
    /// ESL Profile
    const ELECTRONIC_SHELF_LABEL: u8 = 0x34;

    /// 3D Information Data
    /// 3D Synchronization Profile
    const _3D_INFORMATION_DATA: u8 = 0x3D;

    /// Manufacturer Specific Data
    /// Core Specification Supplement, Part A, Section 1.4
    const MANUFACTURER_SPECIFIC_DATA: u8 = 0xFF;
}
