use core::fmt::Debug;
use macros::{FromU8, IntoU8};
use utils::Reader;

// Bluetooth Core spec 6.0 | [Vol 4] Part A, Section 2 | page 1726
pub(crate) const HCI_COMMAND_PACKET_TYPE: u8 = 0x01;
pub(crate) const HCI_ACL_DATA_PACKET_TYPE: u8 = 0x02;
pub(crate) const HCI_SYNCHRONOUS_DATA_PACKET_TYPE: u8 = 0x03;
pub(crate) const HCI_EVENT_PACKET_TYPE: u8 = 0x04;
pub(crate) const HCI_ISO_DATA_PACKET_TYPE: u8 = 0x05;

#[derive(Debug)]
pub enum HCIPacket {
    Event(HCIEventPacket),
    Command(HCICommandPacket),
}

impl HCIPacket {
    pub fn from_buf(buf: &[u8]) -> HCIPacket {
        let mut reader = Reader::new(buf);
        let packet_type = reader.read_u8();
        match packet_type {
            HCI_COMMAND_PACKET_TYPE => {
                let opcode = reader.read_u16();
                let len = reader.read_u8() as usize;
                let data = reader.read_slice(len);

                HCIPacket::Command(HCICommandPacket::new(opcode, len, data))
            }
            HCI_ACL_DATA_PACKET_TYPE => unimplemented!("ACL data packet type not implemented yet"),
            HCI_SYNCHRONOUS_DATA_PACKET_TYPE => {
                unimplemented!("Synchonous data packet type not implemented yet")
            }
            HCI_EVENT_PACKET_TYPE => {
                let evcode = reader.read_u8();
                let len = reader.read_u8() as usize;
                let data = reader.read_slice(len);

                HCIPacket::Event(HCIEventPacket::new(evcode, len, data))
            }
            HCI_ISO_DATA_PACKET_TYPE => unimplemented!("ISO data packet type not implemented yet"),
            _ => panic!("Unknown HCI packet type: {}", packet_type),
        }
    }
}

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 5.4.4 | page 1877
// The Host shall be able to accept HCI Event packets with up to 255 octets of data excluding the HCI Event packet header
const HCI_EVENT_HEADER_SIZE: usize = 2;
const HCI_EVENT_MAX_PARAMETERS_SIZE: usize = 255;
const HCI_EVENT_MAX_PACKET_SIZE: usize = HCI_COMMAND_HEADER_SIZE + HCI_COMMAND_MAX_PARAMETERS_SIZE;

pub struct HCIEventPacket {
    pub evcode: u8,
    pub len: usize,
    pub parameters: [u8; HCI_EVENT_MAX_PARAMETERS_SIZE],
}

impl HCIEventPacket {
    pub fn new(evcode: u8, len: usize, buf: &[u8]) -> Self {
        let mut parameters = [0; 255];
        parameters[..len].copy_from_slice(buf);

        Self {
            evcode,
            len,
            parameters,
        }
    }
}

impl Debug for HCIEventPacket {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "HCIEventPacket {{ evcode: {}, len: {}, parameters: {:?} }}",
            self.evcode,
            self.len,
            &self.parameters[..self.len]
        )
    }
}

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 5.4.1 | page 1872
// Controllers shall be able to accept HCI Command packets with up to 255 bytes of data excluding the HCI Command packet header.
const HCI_COMMAND_HEADER_SIZE: usize = 3;
const HCI_COMMAND_MAX_PARAMETERS_SIZE: usize = 255;
const HCI_COMMAND_MAX_PACKET_SIZE: usize =
    HCI_COMMAND_HEADER_SIZE + HCI_COMMAND_MAX_PARAMETERS_SIZE;

pub struct HCICommandPacket {
    pub opcode: u16,
    pub len: usize,
    pub parameters: [u8; HCI_COMMAND_MAX_PARAMETERS_SIZE],
}

impl HCICommandPacket {
    pub fn new(opcode: u16, len: usize, buf: &[u8]) -> Self {
        let mut parameters = [0; 255];
        parameters[..len].copy_from_slice(buf);

        Self {
            opcode,
            len,
            parameters,
        }
    }
}

impl Debug for HCICommandPacket {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "HCICommandPacket {{ opcode: {}, len: {}, parameters: {:?} }}",
            self.opcode,
            self.len,
            &self.parameters[..self.len]
        )
    }
}
