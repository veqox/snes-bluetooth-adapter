use core::{fmt::Display, u16};

const HCI_COMMAND_HEADER_SIZE: usize = 3;
const HCI_COMMAND_MAX_PACKET_SIZE: usize = 255 + HCI_COMMAND_HEADER_SIZE;

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
pub enum HCIOpcode {}

#[derive(Debug)]
pub struct HCICommandPacket<'p> {
    pub opcode: u16,
    pub parameters: &'p [u8],
}

impl<'p> Into<HCIPacket<'p>> for HCICommandPacket<'p> {
    fn into(self) -> HCIPacket<'p> {
        HCIPacket::HCICommandPacket(self)
    }
}

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 5.4.4 | page 1877
#[derive(Debug)]
pub struct HCIEventPacket<'p> {
    pub evcode: u8,
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
                writer.write_u8(packet.evcode);
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
                let evcode = reader.read_u8();
                let parameters_len = reader.read_u8() as usize;
                let parameters = reader.read_slice(parameters_len);

                Ok(HCIEventPacket { evcode, parameters }.into())
            }
            _ => Err(HCIPacketError),
        }
    }
}

struct Writer<'p> {
    buf: &'p mut [u8],
    pos: usize,
}

impl<'p> Writer<'p> {
    fn new(buf: &'p mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    fn write_u8(&mut self, value: u8) {
        self.buf[self.pos] = value;
        self.pos += 1;
    }

    fn write_u16(&mut self, value: u16) {
        self.buf[self.pos..(self.pos + 2)].copy_from_slice(&value.to_le_bytes());
        self.pos += 2;
    }

    fn write_slice(&mut self, slice: &[u8]) {
        self.buf[self.pos..(self.pos + slice.len())].copy_from_slice(slice);
        self.pos += slice.len();
    }
}

struct Reader<'p> {
    buf: &'p [u8],
    pos: usize,
}

impl<'p> Reader<'p> {
    fn new(buf: &'p [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    fn read_u8(&mut self) -> u8 {
        let value = self.buf[self.pos];
        self.pos += 1;
        value
    }

    fn read_u16(&mut self) -> u16 {
        let value = u16::from_le_bytes([self.buf[self.pos], self.buf[self.pos + 1]]);
        self.pos += 2;
        value
    }

    fn read_slice(&mut self, len: usize) -> &'p [u8] {
        let slice = &self.buf[self.pos..(self.pos + len)];
        self.pos += len;
        slice
    }
}
