use core::{fmt::Display, u16};

use macros::{FromU8, IntoU8, Size};
use utils::{Reader, Writer};

const HCI_COMMAND_HEADER_SIZE: usize = 3;
pub(super) const HCI_COMMAND_MAX_PACKET_SIZE: usize = 255 + HCI_COMMAND_HEADER_SIZE;

const HCI_EVENT_HEADER_SIZE: usize = 2;
const HCI_EVENT_MAX_PACKET_SIZE: usize = 255 + HCI_EVENT_HEADER_SIZE;

// Bluetooth Core spec 6.0 | [Vol 4] Part A, Section 2 | page 1726
const HCI_COMMAND_PACKET_TYPE: u8 = 0x01;
const HCI_ACL_DATA_PACKET_TYPE: u8 = 0x02;
const HCI_SYNCHRONOUS_DATA_PACKET_TYPE: u8 = 0x03;
const HCI_EVENT_PACKET_TYPE: u8 = 0x04;
const HCI_ISO_DATA_PACKET_TYPE: u8 = 0x05;

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.1 | page 1909
// Link Control commands
const OGF_LINK_CONTROL_COMMAND: u16 = 0x01;

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.2 | page 2010
// Link Policy commands
const OGF_LINK_POLICY_COMMAND: u16 = 0x02;

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.3 | page 2037
// Controller & Baseband commands
const OGF_CONTROL_AND_BASEBAND_COMMAND: u16 = 0x03;

const OCF_RESET: u16 = 0x3; // 7.3.2

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.4 | page 2190
// Informational parameters
const OGF_INFORMATIONAL_PARAMETERS_COMMAND: u16 = 0x04;

const OCF_READ_LOCAL_SUPPORTED_COMMANDS: u16 = 0x2; // 7.4.2

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.5 | page 2220
// Status parameters
const OGF_STATUS_PARAMETERS_COMMAND: u16 = 0x05;

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.6 | page 2237
// Testing commands
const OGF_TESTING_COMMAND: u16 = 0x06;

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.7 | page 2240
// Events

#[derive(Debug, FromU8, IntoU8)]
#[repr(u8)]
pub enum HCIEventCode {
    CommandComplete = 0x0E, // 7.7.14
    LEMetaEvent = 0x3E,     // 7.7.65
}

#[derive(Debug, FromU8, IntoU8)]
#[repr(u8)]
pub enum LESubeventCode {
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

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.8 | page 2483
// LE Controller commands
const OGF_LE_CONTROLLER_COMMAND: u16 = 0x08;

const OCF_SET_SCAN_PARAMETERS: u16 = 0x0B; // 7.8.10
const OCF_SET_SCAN_ENABLE: u16 = 0x0C; // 7.8.11

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 5.4.1 | page 1872
// [..] Each command is assigned a 2 byte Opcode used to uniquely identify different types of commands.
// The Opcode parameter is divided into two fields, called the Opcode Group Field (OGF) and Opcode Command Field (OCF).
// The OGF occupies the upper 6 bits of the Opcode, while the OCF occupies the remaining 10 bits. [..]

pub const HCI_RESET_COMMAND: u16 = OCF_RESET | OGF_CONTROL_AND_BASEBAND_COMMAND << 10;
pub const HCI_SET_SCAN_ENABLE_COMMAND: u16 = OCF_SET_SCAN_ENABLE | OGF_LE_CONTROLLER_COMMAND << 10;
pub const HCI_SET_SCAN_PARAMETERS_COMMAND: u16 =
    OCF_SET_SCAN_PARAMETERS | OGF_LE_CONTROLLER_COMMAND << 10;
pub const HCI_READ_LOCAL_SUPPORTED_COMMANDS_COMMAND: u16 =
    OCF_READ_LOCAL_SUPPORTED_COMMANDS | OGF_INFORMATIONAL_PARAMETERS_COMMAND << 10;

#[derive(Debug)]
pub struct HCICommandPacket<'p> {
    pub opcode: u16,
    pub parameters: &'p [u8],
}

#[derive(Debug)]
pub enum HCICommand {
    Reset,                                       // 7.3.2 Reset command
    SetScanParameters(SetScanParametersCommand), // 7.8.10 LE Set Scan Paramaters command
    ScanEnable(ScanEnableCommand),               // 7.8.11 LE Set Scan Enable command
}

impl HCICommand {
    pub fn write_to_buffer(self, buf: &mut [u8]) -> usize {
        let mut writer = Writer::new(buf);
        writer.write_u8(HCI_COMMAND_PACKET_TYPE);
        match self {
            Self::Reset => {
                writer.write_u16(HCI_RESET_COMMAND);
                writer.write_u8(0);
            }
            Self::ScanEnable(command) => {
                writer.write_u16(HCI_SET_SCAN_ENABLE_COMMAND);
                writer.write_u8(command.size() as u8);
                writer.write_u8(command.scan_enable);
                writer.write_u8(command.filter_duplicates);
            }
            Self::SetScanParameters(command) => {
                writer.write_u16(HCI_SET_SCAN_PARAMETERS_COMMAND);
                writer.write_u8(command.size() as u8);
                writer.write_u8(command.scan_type);
                writer.write_u16(command.scan_interval);
                writer.write_u16(command.scan_window);
                writer.write_u8(command.own_address_type);
                writer.write_u8(command.scanning_filter_policy);
            }
        }
        writer.pos
    }
}

// 7.8.10 LE Set Scan Paramaters command
#[derive(Debug, Size)]
pub struct SetScanParametersCommand {
    pub scan_type: u8,
    pub scan_interval: u16,
    pub scan_window: u16,
    pub own_address_type: u8,
    pub scanning_filter_policy: u8,
}

// 7.8.11 LE Set Scan Enable command
#[derive(Debug, Size)]
pub struct ScanEnableCommand {
    pub scan_enable: u8,
    pub filter_duplicates: u8,
}

impl<'p> Into<HCIPacket<'p>> for HCICommandPacket<'p> {
    fn into(self) -> HCIPacket<'p> {
        HCIPacket::HCICommandPacket(self)
    }
}

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 5.4.4 | page 1877
#[derive(Debug)]
pub struct HCIEventPacket<'p> {
    pub evcode: HCIEventCode,
    pub parameters: &'p [u8],
}

impl<'p> Into<HCIPacket<'p>> for HCIEventPacket<'p> {
    fn into(self) -> HCIPacket<'p> {
        HCIPacket::HCIEventPacket(self)
    }
}

#[derive(Debug)]
pub enum HCIPacket<'p> {
    HCICommandPacket(HCICommandPacket<'p>),
    HCIEventPacket(HCIEventPacket<'p>),
}

#[derive(Debug)]
pub struct HCIPacketError;

impl Display for HCIPacketError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("HCIPacketError")
    }
}

impl<'p> HCIPacket<'p> {
    pub fn write_to_buffer(self, buf: &mut [u8]) -> Result<usize, HCIPacketError> {
        let mut writer = Writer::new(buf);
        match self {
            Self::HCICommandPacket(packet) => {
                writer.write_u8(HCI_COMMAND_PACKET_TYPE);
                writer.write_u16(packet.opcode);
                writer.write_u8(packet.parameters.len() as u8);
                writer.write_slice(packet.parameters);
                Ok(writer.pos)
            }
            Self::HCIEventPacket(packet) => {
                writer.write_u8(HCI_EVENT_PACKET_TYPE);
                writer.write_u8(packet.evcode.into());
                writer.write_u8(packet.parameters.len() as u8);
                writer.write_slice(packet.parameters);
                Ok(writer.pos)
            }
        }
    }

    pub fn read_from_slice(slice: &'p [u8]) -> Result<Self, HCIPacketError> {
        let mut reader = Reader::new(slice);
        match reader.read_u8() {
            HCI_COMMAND_PACKET_TYPE => {
                let opcode = reader.read_u16();
                let parameters_len = reader.read_u8() as usize;
                let parameters = reader.read_slice(parameters_len);

                Ok(HCICommandPacket { opcode, parameters }.into())
            }
            HCI_EVENT_PACKET_TYPE => {
                let evcode = reader.read_u8().into();
                let parameters_len = reader.read_u8() as usize;
                let parameters = reader.read_slice(parameters_len);

                Ok(HCIEventPacket { evcode, parameters }.into())
            }
            _ => Err(HCIPacketError),
        }
    }
}
