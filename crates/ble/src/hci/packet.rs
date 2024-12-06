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
    Command(HCICommandPacket),
    ACLData(HCIACLDataPacket),
    Event(HCIEventPacket),
}

impl HCIPacket {
    pub fn from_buf(buf: &[u8]) -> Option<HCIPacket> {
        let mut reader = Reader::new(buf);
        let packet_type = reader.read_u8()?;

        Some(match packet_type {
            HCI_COMMAND_PACKET_TYPE => {
                let opcode = reader.read_u16()?;
                let len = reader.read_u8()? as usize;
                let data = reader.read_slice(len)?;

                HCIPacket::Command(HCICommandPacket::new(opcode, len, data))
            }
            HCI_ACL_DATA_PACKET_TYPE => {
                let header = reader.read_u16()?;
                let handle = (header & 0b1111_1111_1111_0000) >> 4;
                let flags = (header & 0b0000_0000_0000_1111) as u8;
                let packet_boundary_flag = (flags & 0b0000_1100) >> 2;
                let broadcast_flag = (flags & 0b0000_0011);
                let len = reader.read_u16()? as usize;
                let data = reader.read_slice(len)?;

                HCIPacket::ACLData(HCIACLDataPacket::new(
                    handle,
                    packet_boundary_flag,
                    broadcast_flag,
                    len,
                    data,
                ))
            }
            HCI_SYNCHRONOUS_DATA_PACKET_TYPE => {
                log::warn!("Synchonous data packet type not implemented yet");
                return None;
            }
            HCI_EVENT_PACKET_TYPE => {
                let evcode = reader.read_u8()?;
                let len = reader.read_u8()? as usize;
                let data = reader.read_slice(len)?;

                HCIPacket::Event(HCIEventPacket::new(evcode, len, data))
            }
            HCI_ISO_DATA_PACKET_TYPE => {
                log::warn!("ISO data packet type not implemented yet");
                return None;
            }
            _ => {
                log::warn!("Unknown HCI packet type: {}", packet_type);
                return None;
            }
        })
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
        let mut parameters = [0; HCI_EVENT_MAX_PARAMETERS_SIZE];
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
        let mut parameters = [0; HCI_COMMAND_MAX_PARAMETERS_SIZE];
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

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 5.4.2 | page 1874
// Hosts and Controllers shall be able to accept HCI ACL Data packets with up to 27 bytes of data excluding the HCI ACL Data packet header [...]
// The HCI ACL Data packet header is the first 4 octets of the packet.
const HCI_ACL_DATA_HEADER_SIZE: usize = 4;
const HCI_ACL_DATA_MAX_DATA_LENGTH: usize = 27;
const HCI_ACL_DATA_MAX_PACKET_SIZE: usize = HCI_ACL_DATA_HEADER_SIZE + HCI_ACL_DATA_MAX_DATA_LENGTH;

pub struct HCIACLDataPacket {
    pub handle: u16,              // 12 bits
    pub packet_boundary_flag: u8, // 2 bits
    pub broadcast_flag: u8,       // 2 bits
    pub len: usize,
    pub data: [u8; HCI_ACL_DATA_MAX_DATA_LENGTH],
}

impl HCIACLDataPacket {
    pub fn new(
        handle: u16,
        packet_boundary_flag: u8,
        broadcast_flag: u8,
        len: usize,
        buf: &[u8],
    ) -> Self {
        let mut data = [0; HCI_ACL_DATA_MAX_DATA_LENGTH];
        data[..len as usize].copy_from_slice(buf);

        Self {
            handle,
            packet_boundary_flag,
            broadcast_flag,
            len,
            data,
        }
    }
}

impl Debug for HCIACLDataPacket {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "HCIACLDataPacket {{ handle: {}, packet_boundary_flag: {}, broadcast_flag: {}, len: {}, data: {:?} }}",
            self.handle,
            self.packet_boundary_flag,
            self.broadcast_flag,
            self.len,
            &self.data[..self.len as usize]
        )
    }
}
