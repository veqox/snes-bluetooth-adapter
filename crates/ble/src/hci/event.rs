use core::fmt::Debug;
use core::slice::Windows;

use macros::{FromU8, IntoU8};
use utils::{ByteSliceAs, Reader};

use super::HCIEventPacket;

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.7 | page 2240
// Events
const HCI_COMMAND_COMPLETE_EVENT_CODE: u8 = 0x0E;
const HCI_LE_META_EVENT_CODE: u8 = 0x3E;

#[derive(Debug, FromU8, IntoU8)]
#[repr(u8)]
pub enum HCIEventCode {
    CommandComplete = 0x0E, // 7.7.14
    LEMetaEvent = 0x3E,     // 7.7.65
}

#[derive(Debug, FromU8, IntoU8)]
#[repr(u8)]
pub enum SubeventCode {
    ConnectionComplete = 0x01,                        // 7.7.65.1
    AdvertisingReport = 0x02,                         // 7.7.65.2
    ConnectionUpdateComplete = 0x03,                  // 7.7.65.3
    ReadRemoteFeaturesPage0Complete = 0x04,           // 7.7.65.4
    LongTermKeyRequest = 0x05,                        // 7.7.65.5
    RemoteConnectionParameterRequest = 0x06,          // 7.7.65.6
    DataLengthChange = 0x07,                          // 7.7.65.7
    ReadLocalP256PublicKeyComplete = 0x08,            // 7.7.65.8
    GenerateDHKeyComplete = 0x09,                     // 7.7.65.9
    EnhancedConnectionCompleteV1 = 0x0A,              // 7.7.65.10
    EnhancedConnectionCompleteV2 = 0x29,              // 7.7.65.10
    DirectedAdvertisingReport = 0x0B,                 // 7.7.65.11
    PHYUpdateComplete = 0x0C,                         // 7.7.65.12
    ExtendedAdvertisingReport = 0x0D,                 // 7.7.65.13
    PeriodicAdvertisingSyncEstablished = 0x0E,        // 7.7.65.14
    PeriodicAdvertisingReport = 0x0F,                 // 7.7.65.15
    PeriodicAdvertisingSyncLost = 0x10,               // 7.7.65.16
    ScanTimeout = 0x11,                               // 7.7.65.17
    AdvertisingSetTerminated = 0x12,                  // 7.7.65.18
    ScanRequestReceived = 0x13,                       // 7.7.65.19
    ChannelSelectionAlgorithm = 0x14,                 // 7.7.65.20
    ConnectionlessIQReport = 0x15,                    // 7.7.65.21
    ConnectionIQReport = 0x16,                        // 7.7.65.22
    CTERequestFailed = 0x17,                          // 7.7.65.23
    PeriodicAdvertisingSyncTransferReceivedV1 = 0x18, // 7.7.65.24
    PeriodicAdvertisingSyncTransferReceivedV2 = 0x26, // 7.7.65.24
    CISEstablishedV1 = 0x19,                          // 7.7.65.25
    CISEstablishedV2 = 0x2A,                          // 7.7.65.25
    CISRequest = 0x1A,                                // 7.7.65.26
    CreateBIGComplete = 0x1B,                         // 7.7.65.27
    TerminateBIGComplete = 0x1C,                      // 7.7.65.28
    BIGSyncEstablished = 0x1D,                        // 7.7.65.29
    BIGSyncLost = 0x1E,                               // 7.7.65.30
    RequestPeerSCAComplete = 0x1F,                    // 7.7.65.31
    PathLossThreshold = 0x20,                         // 7.7.65.32
    TransmitPowerReporting = 0x21,                    // 7.7.65.33
    BIGInfoAdvertisingReport = 0x22,                  // 7.7.65.34
    SubrateChange = 0x23,                             // 7.7.65.35
    PeriodicAdvertisingSubeventDataRequest = 0x27,    // 7.7.65.36
    PeriodicAdvertisingResponseReport = 0x28,         // 7.7.65.37
    ReadAllRemoteFeaturesComplete = 0x2B,             // 7.7.65.38
    CSReadRemoteSupportedCapabilitiesComplete = 0x2C, // 7.7.65.39
    CSReadRemoteFAETableComplete = 0x2D,              // 7.7.65.40
    CSSecurityEnableComplete = 0x2E,                  // 7.7.65.41
    CSConfigComplete = 0x2F,                          // 7.7.65.42
    CSProcedureEnableComplete = 0x30,                 // 7.7.65.43
    CSSubeventResult = 0x31,                          // 7.7.65.44
    CSSubeventResultContinue = 0x32,                  // 7.7.65.45
    CSTestEndComplete = 0x33,                         // 7.7.65.46
    MonitoredAdvertisersReport = 0x34,                // 7.7.65.47
    FrameSpaceUpdateComplete = 0x35,                  // 7.7.65.48
}

#[derive(Debug)]
pub enum HCIEvent<'p> {
    CommandComplete(CommandCompleteEvent<'p>), // 7.7.14
    LEMetaEvent(LEMetaEvent<'p>),              // 7.7.65
}

impl<'p> HCIEvent<'p> {
    pub fn from_packet(packet: &'p HCIEventPacket) -> Option<HCIEvent<'p>> {
        let mut reader = Reader::new(&packet.parameters);

        Some(match packet.evcode.into() {
            HCIEventCode::CommandComplete => HCIEvent::CommandComplete(CommandCompleteEvent {
                num_hci_command_packets: reader.read_u8()?,
                command_opcode: reader.read_u16()?,
                return_parameters: reader.read_slice(packet.len - reader.pos)?,
            }),
            HCIEventCode::LEMetaEvent => HCIEvent::LEMetaEvent(match reader.read_u8()?.into() {
                SubeventCode::AdvertisingReport => {
                    LEMetaEvent::AdvertisingReport(AdvertisingReportIterator {
                        num_reports: reader.read_u8()?,
                        reader: Reader::new(reader.read_slice(packet.len - reader.pos)?),
                    })
                }
                _ => unimplemented!(),
            }),
            _ => unimplemented!(),
        })
    }
}

#[derive(Debug)]
pub struct CommandCompleteEvent<'p> {
    pub num_hci_command_packets: u8,
    pub command_opcode: u16,
    pub return_parameters: &'p [u8],
}

#[derive(Debug)]
pub enum LEMetaEvent<'p> {
    AdvertisingReport(AdvertisingReportIterator<'p>), // 7.7.65.2
    ReadAllRemoteFeaturesComplete(&'p [u8]),          // 7.7.65.38
}

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.7.65.2 | page 2327
#[derive(Debug)]
pub struct AdvertisingReport<'p> {
    pub event_type: u8,
    pub address_type: u8,
    pub address: &'p [u8],
    pub data: AdvertisingDataIterator<'p>,
    pub rssi: i8,
}

#[derive(Debug)]
pub struct AdvertisingReportIterator<'p> {
    pub num_reports: u8,
    pub reader: Reader<'p>,
}

impl<'p> Iterator for AdvertisingReportIterator<'p> {
    type Item = AdvertisingReport<'p>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reader.remaining() == 0 {
            return None;
        }

        Some(AdvertisingReport {
            event_type: self.reader.read_u8()?,
            address_type: self.reader.read_u8()?,
            address: self.reader.read_slice(6)?,
            data: {
                let len = self.reader.read_u8()? as usize;
                AdvertisingDataIterator {
                    reader: Reader::new(self.reader.read_slice(len)?),
                }
            },
            rssi: self.reader.read_u8()? as i8,
        })
    }
}

#[derive(Debug)]
pub struct AdvertisingDataIterator<'p> {
    pub reader: Reader<'p>,
}

impl<'p> Iterator for AdvertisingDataIterator<'p> {
    type Item = AdvertisingData<'p>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reader.remaining() == 0 {
            return None;
        }

        let len = self.reader.read_u8()?;
        let ad_type = self.reader.read_u8()?.into();
        let data = self.reader.read_slice(len as usize - size_of::<u8>())?;
        let mut reader = Reader::new(data);

        match ad_type {
            AdvertisingDataType::Flags => Some(AdvertisingData::Flags(data[0])),
            AdvertisingDataType::IncompleteListOf16BitServiceUUIDs => {
                Some(AdvertisingData::IncompleteListOf16BitServiceUUIDs({
                    unsafe { data.as_u16_slice()? }
                }))
            }
            AdvertisingDataType::CompleteListOf16BitServiceUUIDs => {
                Some(AdvertisingData::CompleteListOf16BitServiceUUIDs({
                    unsafe { data.as_u16_slice()? }
                }))
            }
            AdvertisingDataType::IncompleteListOf32BitServiceUUIDs => {
                Some(AdvertisingData::IncompleteListOf32BitServiceUUIDs({
                    unsafe { data.as_u32_slice()? }
                }))
            }
            AdvertisingDataType::CompleteListOf32BitServiceUUIDs => {
                Some(AdvertisingData::CompleteListOf32BitServiceUUIDs({
                    unsafe { data.as_u32_slice()? }
                }))
            }
            AdvertisingDataType::IncompleteListOf128BitServiceUUIDs => {
                Some(AdvertisingData::IncompleteListOf128BitServiceUUIDs({
                    unsafe { data.as_u128_slice()? }
                }))
            }
            AdvertisingDataType::CompleteListOf128BitServiceUUIDs => {
                Some(AdvertisingData::CompleteListOf128BitServiceUUIDs({
                    unsafe { data.as_u128_slice()? }
                }))
            }
            AdvertisingDataType::ShortenedLocalName => Some(AdvertisingData::ShortenedLocalName(
                core::str::from_utf8(data).ok()?,
            )),
            AdvertisingDataType::CompleteLocalName => Some(AdvertisingData::CompleteLocalName(
                core::str::from_utf8(data).ok()?,
            )),
            AdvertisingDataType::TxPowerLevel => {
                Some(AdvertisingData::TxPowerLevel(reader.read_u8()? as i8))
            }
            AdvertisingDataType::ClassOfDevice => {
                Some(AdvertisingData::ClassOfDevice(reader.read_u32()?))
            }
            AdvertisingDataType::PeripheralConnectionIntervalRange => {
                Some(AdvertisingData::PeripheralConnectionIntervalRange(data))
            }
            AdvertisingDataType::ServiceData => Some(AdvertisingData::ServiceData(data)),
            AdvertisingDataType::Appearance => {
                Some(AdvertisingData::Appearance(reader.read_u16()?))
            }
            AdvertisingDataType::ManufacturerSpecificData => {
                Some(AdvertisingData::ManufacturerSpecificData(data))
            }
        }
    }
}

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
    /// Bluetooth Core Supplement Spec | Part A, Section 1.14 | Page 13
    ManufacturerSpecificData(&'p [u8]),
}
