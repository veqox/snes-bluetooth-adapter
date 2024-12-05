use core::{
    mem::offset_of,
    ptr::{addr_of, write},
    u16,
};

use macros::Size;
use utils::{SliceAs, Writer};

use super::{gap::AdvertisingData, AdvertisingDataType};

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

const OCF_SET_ADVERTISING_PARAMETERS: u16 = 0x06; // 7.8.5
const OCF_SET_ADVERTISING_DATA: u16 = 0x08; // 7.8.7
const OCF_SET_RESPONSE_DATA: u16 = 0x9; // 7.7.8
const OCF_SET_ADVERTISING_ENABLE: u16 = 0x0A; // 7.8.9
const OCF_SET_SCAN_PARAMETERS: u16 = 0x0B; // 7.8.10
const OCF_SET_SCAN_ENABLE: u16 = 0x0C; // 7.8.11

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 5.4.1 | page 1872
// [...] Each command is assigned a 2 byte Opcode used to uniquely identify different types of commands.
// The Opcode parameter is divided into two fields, called the Opcode Group Field (OGF) and Opcode Command Field (OCF).
// The OGF occupies the upper 6 bits of the Opcode, while the OCF occupies the remaining 10 bits. [...]

const fn opcode(ocf: u16, ogf: u16) -> u16 {
    ocf | ogf << 10
}

#[derive(Debug)]
pub enum HCICommand<'p> {
    Reset,                                                     // 7.3.2
    SetAdvertisingParameters(SetAdvertisingParametersCommand), // 7.8.5
    SetAdvertisingData { data: &'p [AdvertisingData<'p>] },    // 7.8.7
    SetScanResponseData { data: &'p [AdvertisingData<'p>] },   // 7.8.8
    SetAdvertisingEnable { enable: u8 },                       // 7.8.9
    SetScanParameters(SetScanParametersCommand),               // 7.8.10
    ScanEnable(ScanEnableCommand),                             // 7.8.11
}

impl<'p> HCICommand<'p> {
    pub fn write_into(self, buf: &mut [u8]) -> Option<usize> {
        let mut writer = Writer::new(buf);
        writer.write_u8(super::packet::HCI_COMMAND_PACKET_TYPE);

        match self {
            Self::Reset => {
                writer.write_u16(opcode(OCF_RESET, OGF_CONTROL_AND_BASEBAND_COMMAND));
                writer.write_u8(0);
            }
            Self::SetAdvertisingParameters(command) => {
                writer.write_u16(opcode(
                    OCF_SET_ADVERTISING_PARAMETERS,
                    OGF_LE_CONTROLLER_COMMAND,
                ));
                writer.write_u8(command.size() as u8);
                writer.write_u16(command.interval_min);
                writer.write_u16(command.interval_max);
                writer.write_u8(command.advertising_type);
                writer.write_u8(command.own_address_type);
                writer.write_u8(command.peer_address_type);
                writer.write_slice(&command.peer_address);
                writer.write_u8(command.advertising_channel_map);
                writer.write_u8(command.advertising_filter_policy);
            }
            Self::ScanEnable(command) => {
                writer.write_u16(opcode(OCF_SET_SCAN_ENABLE, OGF_LE_CONTROLLER_COMMAND));
                writer.write_u8(command.size() as u8);
                writer.write_u8(command.scan_enable);
                writer.write_u8(command.filter_duplicates);
            }
            Self::SetScanParameters(command) => {
                writer.write_u16(opcode(OCF_SET_SCAN_PARAMETERS, OGF_LE_CONTROLLER_COMMAND));
                writer.write_u8(command.size() as u8);
                writer.write_u8(command.scan_type);
                writer.write_u16(command.scan_interval);
                writer.write_u16(command.scan_window);
                writer.write_u8(command.own_address_type);
                writer.write_u8(command.scanning_filter_policy);
            }
            Self::SetAdvertisingData { data } => {
                writer.write_u16(opcode(OCF_SET_ADVERTISING_DATA, OGF_LE_CONTROLLER_COMMAND));
                writer.write_u8(32);
                let mut data_buf = [0; 31];

                let mut offset = 0;

                for data in data.iter() {
                    let len = data.write_into(&mut data_buf[offset..])?;
                    offset += len;
                }

                writer.write_u8(offset as u8);
                writer.write_slice(&data_buf);
            }
            Self::SetScanResponseData { data } => {
                writer.write_u16(opcode(OCF_SET_RESPONSE_DATA, OGF_LE_CONTROLLER_COMMAND));
                writer.write_u8(32);
                let mut data_buf = [0; 31];

                let mut offset = 0;

                for data in data.iter() {
                    let len = data.write_into(&mut data_buf[offset..])?;
                    offset += len;
                }

                writer.write_u8(offset as u8);
                writer.write_slice(&data_buf);
            }
            Self::SetAdvertisingEnable { enable } => {
                writer.write_u16(opcode(
                    OCF_SET_ADVERTISING_ENABLE,
                    OGF_LE_CONTROLLER_COMMAND,
                ));
                writer.write_u8(size_of::<u8>() as u8);
                writer.write_u8(enable);
            }
        }

        Some(writer.pos)
    }
}

// 7.8.5 LE Set Advertising Parameters command
#[derive(Debug, Size)]
pub struct SetAdvertisingParametersCommand {
    pub interval_min: u16,
    pub interval_max: u16,
    pub advertising_type: u8,
    pub own_address_type: u8,
    pub peer_address_type: u8,
    pub peer_address: [u8; 6],
    pub advertising_channel_map: u8,
    pub advertising_filter_policy: u8,
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
