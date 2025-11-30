use core::fmt::Debug;

use crate::hci::{BufReader, event::HCIEvent};

#[derive(Debug)]
pub enum HCIPacket<'a> {
    ACLData(ACLData<'a>),
    SynchronousData(SynchronousData<'a>),
    Event(HCIEvent<'a>),
    Unknown { packet_type: u8, data: &'a [u8] },
}

impl<'a> HCIPacket<'a> {
    pub fn from_bytes(buf: &'a [u8]) -> Self {
        let mut reader = BufReader::new(buf);

        match reader.read_u8() {
            ACLData::TYPE => Self::ACLData(ACLData::from_reader(&mut reader)),
            SynchronousData::TYPE => {
                Self::SynchronousData(SynchronousData::from_reader(&mut reader))
            }
            HCIEvent::TYPE => Self::Event(HCIEvent::from_reader(&mut reader)),
            packet_type => Self::Unknown {
                packet_type,
                data: reader.read_bytes(reader.remaining()),
            },
        }
    }
}

pub struct ACLData<'a> {
    pub handle: u16,
    pub boundary_flag: u8,
    pub broadcast_flag: u8,
    pub data: &'a [u8],
}

impl<'a> Debug for ACLData<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ACLDataPacket")
            .field("handle", &self.handle)
            .field(
                "boundary_flag",
                &format_args!("0x{:02X}", self.boundary_flag),
            )
            .field(
                "broadcast_flag",
                &format_args!("0x{:02X}", self.broadcast_flag),
            )
            .field("data", &self.data)
            .finish()
    }
}

impl<'a> ACLData<'a> {
    pub const TYPE: u8 = 0x02;

    pub(crate) fn from_reader(reader: &mut BufReader<'a>) -> Self {
        let handle_with_flags = reader.read_u16_le();

        let handle = handle_with_flags >> 4;
        let boundary_flag = (handle_with_flags >> 2) as u8 | 0x3;
        let broadcast_flag = handle_with_flags as u8 | 0x3;

        let data_len = reader.read_u16_le() as usize;
        let data = reader.read_bytes(data_len);

        Self {
            handle,
            boundary_flag,
            broadcast_flag,
            data,
        }
    }
}

pub struct SynchronousData<'a> {
    pub handle: u16,
    pub status_flag: u8,
    pub data: &'a [u8],
}

impl<'a> Debug for SynchronousData<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SynchronousDataPacket")
            .field("handle", &self.handle)
            .field("status_flag", &format_args!("0x{:02X}", self.status_flag))
            .field("data", &self.data)
            .finish()
    }
}

impl<'a> SynchronousData<'a> {
    pub const TYPE: u8 = 0x03;

    pub(crate) fn from_reader(reader: &mut BufReader<'a>) -> Self {
        let handle_with_flags = reader.read_u16_le();

        let handle = handle_with_flags >> 4;
        let status_flag = (handle_with_flags >> 2) as u8 | 0x3;

        let data_len = reader.read_u8() as usize;
        let data = reader.read_bytes(data_len);

        Self {
            handle,
            status_flag,
            data,
        }
    }
}

pub struct Event<'a> {
    pub ev_code: u8,
    pub params: &'a [u8],
}

impl<'a> Debug for Event<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EventPacket")
            .field("ev_code", &format_args!("0x{:02X}", self.ev_code))
            .field("params", &self.params)
            .finish()
    }
}
