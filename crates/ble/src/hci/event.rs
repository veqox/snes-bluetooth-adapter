use core::fmt::Debug;

use macros::{FromU8, IntoU8};
use utils::Reader;

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
    pub fn from_packet(packet: &'p HCIEventPacket) -> HCIEvent<'p> {
        let mut reader = Reader::new(&packet.parameters);

        match packet.evcode.into() {
            HCIEventCode::CommandComplete => HCIEvent::CommandComplete(CommandCompleteEvent {
                num_hci_command_packets: reader.read_u8(),
                command_opcode: reader.read_u16(),
                return_parameters: reader.read_slice(packet.len - reader.pos),
            }),
            HCIEventCode::LEMetaEvent => HCIEvent::LEMetaEvent(match reader.read_u8().into() {
                SubeventCode::AdvertisingReport => {
                    LEMetaEvent::AdvertisingReport(AdvertisingReportEvent {
                        reports: AdvertisingReportIterator {
                            num_reports: reader.read_u8(),
                            reader: Reader::new(reader.read_slice(packet.len - reader.pos)),
                        },
                    })
                }
                _ => unimplemented!(),
            }),
            _ => unimplemented!(),
        }
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
    AdvertisingReport(AdvertisingReportEvent<'p>), // 7.7.65.2
    ReadAllRemoteFeaturesComplete(ReadAllRemoteFeaturesCompleteEvent<'p>), // 7.7.65.38
}

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.7.65.2 | page 2327
#[derive(Debug)]
pub struct AdvertisingReportEvent<'p> {
    pub reports: AdvertisingReportIterator<'p>,
}

#[derive(Debug)]
pub struct AdvertisingReport<'p> {
    pub event_type: u8,
    pub address_type: u8,
    pub address: &'p [u8],
    pub data: AdvertisingResponseDataIterator<'p>,
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
            event_type: self.reader.read_u8(),
            address_type: self.reader.read_u8(),
            address: self.reader.read_slice(6),
            data: {
                let len = self.reader.read_u8() as usize;
                AdvertisingResponseDataIterator {
                    reader: Reader::new(self.reader.read_slice(len)),
                }
            },
            rssi: self.reader.read_u8() as i8,
        })
    }
}

#[derive(Debug)]
pub struct AdvertisingResponseData<'p> {
    pub data: AdvertisingData<'p>,
}

#[derive(Debug)]
pub struct AdvertisingResponseDataIterator<'p> {
    pub reader: Reader<'p>,
}

impl<'p> Iterator for AdvertisingResponseDataIterator<'p> {
    type Item = AdvertisingResponseData<'p>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reader.remaining() == 0 {
            return None;
        }

        let len = self.reader.read_u8();
        let ad_type = self.reader.read_u8().into();
        let data = self.reader.read_slice(len as usize - size_of::<u8>());

        match ad_type {
            AdvertisingDataType::Flags => Some(AdvertisingResponseData {
                data: AdvertisingData::Flags(data[0]),
            }),
            AdvertisingDataType::IncompleteListOf16BitServiceUUIDs => {
                Some(AdvertisingResponseData {
                    data: AdvertisingData::IncompleteListOf16BitServiceUUIDs(&data),
                })
            }
            AdvertisingDataType::CompleteListOf16BitServiceUUIDs => Some(AdvertisingResponseData {
                data: AdvertisingData::CompleteListOf16BitServiceUUIDs(&data),
            }),
            AdvertisingDataType::ShortenedLocalName => Some(AdvertisingResponseData {
                data: AdvertisingData::ShortenedLocalName(core::str::from_utf8(data).unwrap()),
            }),
            AdvertisingDataType::CompleteLocalName => Some(AdvertisingResponseData {
                data: AdvertisingData::CompleteLocalName(core::str::from_utf8(data).unwrap()),
            }),
            AdvertisingDataType::TxPowerLevel => Some(AdvertisingResponseData {
                data: AdvertisingData::TxPowerLevel(data[0] as i8),
            }),
            AdvertisingDataType::ClassOfDevice => Some(AdvertisingResponseData {
                data: AdvertisingData::ClassOfDevice(u32::from_le_bytes([
                    data[0], data[1], data[2], data[3],
                ])),
            }),
            AdvertisingDataType::PeripheralConnectionIntervalRange => {
                Some(AdvertisingResponseData {
                    data: AdvertisingData::PeripheralConnectionIntervalRange(data),
                })
            }
            AdvertisingDataType::ServiceData => Some(AdvertisingResponseData {
                data: AdvertisingData::ServiceData(data),
            }),
            AdvertisingDataType::Appearance => Some(AdvertisingResponseData {
                data: AdvertisingData::Appearance(u16::from_le_bytes([data[0], data[1]])),
            }),
            AdvertisingDataType::ManufacturerSpecificData => Some(AdvertisingResponseData {
                data: AdvertisingData::ManufacturerSpecificData(data),
            }),
        }
    }
}

// Bluetooth Assigned Numbers | Section 2.3 | page 12
#[derive(Debug, IntoU8, FromU8)]
pub enum AdvertisingDataType {
    Flags = 0x01,                             // Flags
    IncompleteListOf16BitServiceUUIDs = 0x02, // Incomplete List of 16-bit Service UUIDs
    CompleteListOf16BitServiceUUIDs = 0x03,   // Complete List of 16-bit Service UUIDs
    ShortenedLocalName = 0x08,                // Shortened Local Name
    CompleteLocalName = 0x09,                 // Complete Local Name
    TxPowerLevel = 0x0A,                      // Tx Power Level
    ClassOfDevice = 0x0D,                     // Class of Device
    PeripheralConnectionIntervalRange = 0x12, // Peripheral Connection Interval Range
    ServiceData = 0x16,                       // Service Data
    Appearance = 0x19,                        // Appearance
    ManufacturerSpecificData = 0xFF,          // Manufacturer Specific Data
}

#[derive(Debug)]
pub enum AdvertisingData<'p> {
    Flags(u8),
    IncompleteListOf16BitServiceUUIDs(&'p [u8]),
    CompleteListOf16BitServiceUUIDs(&'p [u8]),
    ShortenedLocalName(&'p str),
    CompleteLocalName(&'p str),
    TxPowerLevel(i8),
    ClassOfDevice(u32),
    PeripheralConnectionIntervalRange(&'p [u8]),
    ServiceData(&'p [u8]),
    Appearance(u16),
    ManufacturerSpecificData(&'p [u8]),
}

#[derive(Debug)]
pub struct ReadAllRemoteFeaturesCompleteEvent<'p> {
    pub data: &'p [u8],
}
