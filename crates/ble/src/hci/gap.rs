use macros::{FromU8, IntoU8};
use utils::{SliceAs, Writer};

pub const AD_FLAG_LIMITED_DISCOVERABLE_MODE: u8 = 0b0000_0001;
pub const AD_FLAG_GENERAL_DISCOVERABLE_MODE: u8 = 0b0000_0010;
pub const AD_FLAG_BR_EDR_NOT_SUPPORTED: u8 = 0b0000_0100;
pub const AD_FLAG_SIMULTANEOUS_LE_BR_EDR_CONTROLLER: u8 = 0b0000_1000;
pub const AD_FLAG_SIMULTANEOUS_LE_BR_EDR_HOST: u8 = 0b0001_0000;

// Bluetooth Assigned Numbers | Section 2.3 | page 12
#[derive(Debug, IntoU8, FromU8)]
pub enum AdvertisingDataType {
    Flags = 0x01,                              // Flags
    IncompleteListOf16BitServiceUUIDs = 0x02,  // Incomplete List of 16-bit Service UUIDs
    CompleteListOf16BitServiceUUIDs = 0x03,    // Complete List of 16-bit Service UUIDs
    IncompleteListOf32BitServiceUUIDs = 0x04,  // Incomplete List of 32-bit Service UUIDs
    CompleteListOf32BitServiceUUIDs = 0x05,    // Complete List of 32-bit Service UUIDs
    IncompleteListOf128BitServiceUUIDs = 0x06, // Incomplete List of 128-bit Service UUIDs
    CompleteListOf128BitServiceUUIDs = 0x07,   // Complete List of 128-bit Service UUIDs
    ShortenedLocalName = 0x08,                 // Shortened Local Name
    CompleteLocalName = 0x09,                  // Complete Local Name
    TxPowerLevel = 0x0A,                       // Tx Power Level
    ClassOfDevice = 0x0D,                      // Class of Device
    PeripheralConnectionIntervalRange = 0x12,  // Peripheral Connection Interval Range
    ServiceData = 0x16,                        // Service Data
    Appearance = 0x19,                         // Appearance
    LEBluetoothDeviceAddress = 0x1B,           // LE Bluetooth Device Address
    ManufacturerSpecificData = 0xFF,           // Manufacturer Specific Data
}

// Bluetooth Core Supplement spec | Part A, Section 1 | page 9
#[derive(Debug)]
pub enum AdvertisingData<'p> {
    /// Bluetooth Core Supplement Spec | Part A, Section 1.3 | page 12
    ///
    /// | Bit  | Description |
    /// | ---- | ----------- |
    /// | 0    | LE Limited Discoverable Mode |
    /// | 1    | LE General Discoverable Mode |
    /// | 2    | BR/EDR Not Supported |
    /// | 3    | Simultaneous LE and BR/EDR to Same Device Capable (Controller) |
    /// | 4    | Simultaneous LE and BR/EDR to Same Device Capable (Host) |
    /// | 5..7 | Reserved for future use |
    Flags(u8),

    /// Bluetooth Core Supplement Spec | Part A, Section 1.1 | Page 10
    IncompleteListOf16BitServiceUUIDs(&'p [u16]),
    /// Bluetooth Core Supplement Spec | Part A, Section 1.1 | Page 10
    CompleteListOf16BitServiceUUIDs(&'p [u16]),
    /// Bluetooth Core Supplement Spec | Part A, Section 1.1 | Page 10
    IncompleteListOf32BitServiceUUIDs(&'p [u32]),
    /// Bluetooth Core Supplement Spec | Part A, Section 1.1 | Page 10
    CompleteListOf32BitServiceUUIDs(&'p [u32]),
    /// Bluetooth Core Supplement Spec | Part A, Section 1.1 | Page 10
    IncompleteListOf128BitServiceUUIDs(&'p [u128]),
    /// Bluetooth Core Supplement Spec | Part A, Section 1.1 | Page 10
    CompleteListOf128BitServiceUUIDs(&'p [u128]),
    /// Bluetooth Core Supplement Spec | Part A, Section 1.2 | Page 11
    ///
    /// Bluetooth Core Spec | [Vol 4] Part E, Section 6.23 | Page 1891
    ///
    /// A UTF-8 encoded User Friendly Descriptive Name for the device with type utf8{248}.
    ShortenedLocalName(&'p str),
    /// Bluetooth Core Supplement Spec | Part A, Section 1.2 | Page 11
    ///
    /// Bluetooth Core Spec | [Vol 4] Part E, Section 6.23 | Page 1891
    ///
    /// A UTF-8 encoded User Friendly Descriptive Name for the device with type utf8{248}.
    CompleteLocalName(&'p str),
    /// Bluetooth Core Supplement Spec | Part A, Section 1.5 | Page 13
    TxPowerLevel(i8),
    /// Bluetooth Assigned Numbers | Section 2.8 | page 45
    ClassOfDevice(u32),
    /// Bluetooth Core Supplement Spec | Part A, Section 1.9 | Page 16
    PeripheralConnectionIntervalRange(&'p [u8]),
    /// Bluetooth Core Supplement Spec | Part A, Section 1.11 | Page 18
    ServiceData(&'p [u8]),
    ///  Bluetooth Core Supplement Spec | Section 1.12 | page 18
    Appearance(u16),
    /// Bluetooth Core Supplement Spec | Part A, Section 1.16 | Page 20
    LEBluetoothDeviceAddress(&'p [u8]),
    /// Bluetooth Core Supplement Spec | Part A, Section 1.14 | Page 13
    ManufacturerSpecificData(&'p [u8]),
}

impl<'p> AdvertisingData<'p> {
    pub fn write_into(&self, buf: &'p mut [u8]) -> Option<usize> {
        let mut writer = Writer::new(buf);
        match *self {
            AdvertisingData::Flags(flags) => {
                writer.write_u8((2 * size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::Flags as u8);
                writer.write_u8(flags);
            }
            AdvertisingData::IncompleteListOf16BitServiceUUIDs(uuids) => {
                writer.write_u8((uuids.len() * size_of::<u16>() + size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::IncompleteListOf16BitServiceUUIDs as u8);
                writer.write_slice(unsafe { uuids.as_u8_slice()? });
            }
            AdvertisingData::CompleteListOf16BitServiceUUIDs(uuids) => {
                writer.write_u8((uuids.len() * size_of::<u16>() + size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::CompleteListOf16BitServiceUUIDs as u8);
                writer.write_slice(unsafe { uuids.as_u8_slice()? });
            }
            AdvertisingData::IncompleteListOf32BitServiceUUIDs(uuids) => {
                writer.write_u8((uuids.len() * size_of::<u32>() + size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::IncompleteListOf32BitServiceUUIDs as u8);
                writer.write_slice(unsafe { uuids.as_u8_slice()? });
            }
            AdvertisingData::CompleteListOf32BitServiceUUIDs(uuids) => {
                writer.write_u8((uuids.len() * size_of::<u32>() + size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::CompleteListOf32BitServiceUUIDs as u8);
                writer.write_slice(unsafe { uuids.as_u8_slice()? });
            }
            AdvertisingData::IncompleteListOf128BitServiceUUIDs(uuids) => {
                writer.write_u8((uuids.len() * size_of::<u128>() + size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::IncompleteListOf128BitServiceUUIDs as u8);
                writer.write_slice(unsafe { uuids.as_u8_slice()? });
            }
            AdvertisingData::CompleteListOf128BitServiceUUIDs(uuids) => {
                writer.write_u8((uuids.len() * size_of::<u128>() + size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::CompleteListOf128BitServiceUUIDs as u8);
                writer.write_slice(unsafe { uuids.as_u8_slice()? });
            }
            AdvertisingData::ShortenedLocalName(name) => {
                writer.write_u8((name.len() + size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::ShortenedLocalName as u8);
                writer.write_slice(name.as_bytes());
            }
            AdvertisingData::CompleteLocalName(name) => {
                writer.write_u8((name.len() + size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::CompleteLocalName as u8);
                writer.write_slice(name.as_bytes());
            }
            AdvertisingData::TxPowerLevel(level) => {
                writer.write_u8((size_of::<i8>() + size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::TxPowerLevel as u8);
                writer.write_u8(level as u8);
            }
            AdvertisingData::ClassOfDevice(class) => {
                writer.write_u8((size_of::<u32>() + size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::ClassOfDevice as u8);
                writer.write_u32(class);
            }
            AdvertisingData::PeripheralConnectionIntervalRange(range) => {
                writer.write_u8((range.len() + size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::PeripheralConnectionIntervalRange as u8);
                writer.write_slice(range);
            }
            AdvertisingData::ServiceData(data) => {
                writer.write_u8((data.len() + size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::PeripheralConnectionIntervalRange as u8);
                writer.write_slice(data);
            }
            AdvertisingData::Appearance(appearance) => {
                writer.write_u8((size_of::<u16>() + size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::PeripheralConnectionIntervalRange as u8);
                writer.write_u16(appearance);
            }
            AdvertisingData::LEBluetoothDeviceAddress(address) => {
                writer.write_u8((address.len() + size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::PeripheralConnectionIntervalRange as u8);
                writer.write_slice(address);
            }
            AdvertisingData::ManufacturerSpecificData(data) => {
                writer.write_u8((data.len() + size_of::<u8>()) as u8);
                writer.write_u8(AdvertisingDataType::PeripheralConnectionIntervalRange as u8);
                writer.write_slice(data);
            }
        };

        Some(writer.pos)
    }
}
