use core::{any::type_name, fmt::Debug};
use utils::Reader;

// Bluetooth Core spec 6.0 | [Vol 4] Part A, Section 2 | page 1726
#[derive(Debug)]
pub enum HCIPacket<'p> {
    Command(HCICommandPacket<'p>),
    ACLData(HCIACLDataPacket<'p>),
    Event(HCIEventPacket<'p>),
    Unkown(&'p [u8]),
}

impl<'p> HCIPacket<'p> {
    pub(crate) const COMMAND_PACKET_TYPE: u8 = 0x01;
    pub(crate) const ACL_DATA_PACKET_TYPE: u8 = 0x02;
    pub(crate) const SYNCHRONOUS_DATA_PACKET_TYPE: u8 = 0x03;
    pub(crate) const EVENT_PACKET_TYPE: u8 = 0x04;
    pub(crate) const ISO_DATA_PACKET_TYPE: u8 = 0x05;

    pub fn from_buf(buf: &'p [u8]) -> Option<HCIPacket<'p>> {
        let mut reader = Reader::new(buf);
        let packet_type = reader.read_u8()?;

        Some(match packet_type {
            Self::COMMAND_PACKET_TYPE => {
                let opcode = reader.read_u16()?;
                let len = reader.read_u8()? as usize;
                let data = reader.read_u8_slice(len)?;

                Self::Command(HCICommandPacket::new(opcode, len, data))
            }
            Self::ACL_DATA_PACKET_TYPE => {
                let header = reader.read_u16()?;
                let handle = (header & 0b1111_1111_1111_0000) >> 4;
                let flags = (header & 0b0000_0000_0000_1111) as u8;
                let packet_boundary_flag = (flags & 0b0000_1100) >> 2;
                let broadcast_flag = flags & 0b0000_0011;
                let len = reader.read_u16()? as usize;
                let data = reader.read_u8_slice(len)?;

                Self::ACLData(HCIACLDataPacket::new(
                    handle,
                    packet_boundary_flag,
                    broadcast_flag,
                    len,
                    data,
                ))
            }
            Self::SYNCHRONOUS_DATA_PACKET_TYPE => {
                log::warn!("Synchonous data packet type not implemented yet");
                Self::Unkown(buf)
            }
            Self::EVENT_PACKET_TYPE => {
                let evcode = reader.read_u8()?;
                let len = reader.read_u8()? as usize;
                let data = reader.read_u8_slice(len)?;

                Self::Event(HCIEventPacket::new(evcode, len, data))
            }
            Self::ISO_DATA_PACKET_TYPE => {
                log::warn!("ISO data packet type not implemented yet");
                Self::Unkown(buf)
            }
            _ => {
                log::warn!("Unknown HCI packet type: {}", packet_type);
                Self::Unkown(buf)
            }
        })
    }
}

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 5.4.4 | page 1877
// The Host shall be able to accept HCI Event packets with up to 255 octets of data excluding the HCI Event packet header
pub struct HCIEventPacket<'p> {
    pub evcode: u8,
    pub len: usize,
    pub parameters: &'p [u8],
}

impl<'p> HCIEventPacket<'p> {
    #[allow(unused)]
    const HEADER_SIZE: usize = 2;

    #[allow(unused)]
    const MAX_PARAMETERS_SIZE: usize = 255;

    #[allow(unused)]
    const MAX_PACKET_SIZE: usize = Self::HEADER_SIZE + Self::MAX_PARAMETERS_SIZE;

    pub fn new(evcode: u8, len: usize, buf: &'p [u8]) -> Self {
        Self {
            evcode,
            len,
            parameters: &buf[..len],
        }
    }
}

impl Debug for HCIEventPacket<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("evcode", &self.evcode)
            .field("len", &self.len)
            .field("parameters", &&self.parameters[..self.len])
            .finish()
    }
}

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 5.4.1 | page 1872
// Controllers shall be able to accept HCI Command packets with up to 255 bytes of data excluding the HCI Command packet header.
pub struct HCICommandPacket<'p> {
    pub opcode: u16,
    pub len: usize,
    pub parameters: &'p [u8],
}

impl<'p> HCICommandPacket<'p> {
    #[allow(unused)]
    const HEADER_SIZE: usize = 3;

    #[allow(unused)]
    const MAX_PARAMETERS_SIZE: usize = 255;

    #[allow(unused)]
    const MAX_PACKET_SIZE: usize = Self::HEADER_SIZE + Self::MAX_PARAMETERS_SIZE;

    pub fn new(opcode: u16, len: usize, buf: &'p [u8]) -> Self {
        Self {
            opcode,
            len,
            parameters: &buf[..len],
        }
    }
}

impl Debug for HCICommandPacket<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("opcode", &self.opcode)
            .field("len", &self.len)
            .field("parameters", &&self.parameters[..self.len])
            .finish()
    }
}

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 5.4.2 | page 1874
// Hosts and Controllers shall be able to accept HCI ACL Data packets with up to 27 bytes of data excluding the HCI ACL Data packet header [...]
// The HCI ACL Data packet header is the first 4 octets of the packet.
pub struct HCIACLDataPacket<'p> {
    pub handle: u16,              // 12 bits
    pub packet_boundary_flag: u8, // 2 bits
    pub broadcast_flag: u8,       // 2 bits
    pub len: usize,
    pub data: &'p [u8],
}

impl<'p> HCIACLDataPacket<'p> {
    #[allow(unused)]
    const HEADER_SIZE: usize = 4;

    #[allow(unused)]
    const MAX_DATA_LENGTH: usize = 27;

    #[allow(unused)]
    const MAX_PACKET_SIZE: usize = Self::HEADER_SIZE + Self::MAX_DATA_LENGTH;

    pub fn new(
        handle: u16,
        packet_boundary_flag: u8,
        broadcast_flag: u8,
        len: usize,
        buf: &'p [u8],
    ) -> Self {
        Self {
            handle,
            packet_boundary_flag,
            broadcast_flag,
            len,
            data: &buf[..len],
        }
    }
}

impl Debug for HCIACLDataPacket<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("handle", &self.handle)
            .field("packet_boundary_flag", &self.packet_boundary_flag)
            .field("broadcast_flag", &self.broadcast_flag)
            .field("len", &self.len)
            .field("data", &&self.data[..self.len])
            .finish()
    }
}
