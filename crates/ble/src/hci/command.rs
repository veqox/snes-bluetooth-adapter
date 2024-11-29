use macros::Size;
use utils::Writer;

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

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.8 | page 2483
// LE Controller commands
const OGF_LE_CONTROLLER_COMMAND: u16 = 0x08;

const OCF_SET_SCAN_PARAMETERS: u16 = 0x0B; // 7.8.10
const OCF_SET_SCAN_ENABLE: u16 = 0x0C; // 7.8.11

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 5.4.1 | page 1872
// [...] Each command is assigned a 2 byte Opcode used to uniquely identify different types of commands.
// The Opcode parameter is divided into two fields, called the Opcode Group Field (OGF) and Opcode Command Field (OCF).
// The OGF occupies the upper 6 bits of the Opcode, while the OCF occupies the remaining 10 bits. [...]

pub const HCI_RESET_COMMAND: u16 = OCF_RESET | OGF_CONTROL_AND_BASEBAND_COMMAND << 10;
pub const HCI_SET_SCAN_ENABLE_COMMAND: u16 = OCF_SET_SCAN_ENABLE | OGF_LE_CONTROLLER_COMMAND << 10;
pub const HCI_SET_SCAN_PARAMETERS_COMMAND: u16 =
    OCF_SET_SCAN_PARAMETERS | OGF_LE_CONTROLLER_COMMAND << 10;
pub const HCI_READ_LOCAL_SUPPORTED_COMMANDS_COMMAND: u16 =
    OCF_READ_LOCAL_SUPPORTED_COMMANDS | OGF_INFORMATIONAL_PARAMETERS_COMMAND << 10;

#[derive(Debug)]
pub enum HCICommand {
    Reset,                                       // 7.3.2
    SetScanParameters(SetScanParametersCommand), // 7.8.10
    ScanEnable(ScanEnableCommand),               // 7.8.11
}

impl HCICommand {
    pub fn write_into(self, buf: &mut [u8]) -> usize {
        let mut writer = Writer::new(buf);
        writer.write_u8(super::packet::HCI_COMMAND_PACKET_TYPE);
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
