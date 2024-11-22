use core::fmt::Display;

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
    pub fn read(self, buf: &mut [u8]) -> Result<usize, HCIPacketError> {
        match self {
            Self::HCICommandPacket(packet) => {
                buf[0] = 0x01;
                buf[1..=2].copy_from_slice(&packet.opcode.to_be_bytes());
                buf[3] = packet.length;
                buf[4..(packet.length as usize + 4)].copy_from_slice(&packet.parameters);
                Ok(1 + 2 + 1 + packet.length as usize)
            }
            Self::HCIEventPacket(packet) => {
                buf[0] = 0x04;
                buf[1] = packet.evcode;
                buf[2] = packet.length;
                buf[3..(packet.length as usize + 3)].copy_from_slice(&packet.parameters);
                Ok(1 + 1 + 1 + packet.length as usize)
            }
        }
    }
}

#[derive(Debug)]
pub struct HCICommandPacket<'p> {
    pub opcode: u16,
    pub length: u8,
    pub parameters: &'p [u8],
}

impl<'p> Into<HCIPacket<'p>> for HCICommandPacket<'p> {
    fn into(self) -> HCIPacket<'p> {
        HCIPacket::HCICommandPacket(self)
    }
}

#[derive(Debug)]
pub struct HCIEventPacket<'p> {
    pub evcode: u8,
    pub length: u8,
    pub parameters: &'p [u8],
}
