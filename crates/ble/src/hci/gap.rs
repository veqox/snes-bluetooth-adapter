use embedded_io::Write;

use crate::hci::WriteHCI;

pub const AD_FLAG_LIMITED_DISCOVERABLE_MODE: u8 = 0b0000_0001;
pub const AD_FLAG_GENERAL_DISCOVERABLE_MODE: u8 = 0b0000_0010;
pub const AD_FLAG_BR_EDR_NOT_SUPPORTED: u8 = 0b0000_0100;
pub const AD_FLAG_SIMULTANEOUS_LE_BR_EDR_CONTROLLER: u8 = 0b0000_1000;
pub const AD_FLAG_SIMULTANEOUS_LE_BR_EDR_HOST: u8 = 0b0001_0000;

pub enum AdvData {
    Flags(u8),
    ShortenedLocalName(&'static str),
    CompleteLocalName(&'static str),
}

impl AdvData {
    pub fn len(&self) -> u8 {
        (match self {
            AdvData::Flags(_) => size_of::<u8>() + size_of::<u8>(),
            AdvData::ShortenedLocalName(name) => name.len() + size_of::<u8>(),
            AdvData::CompleteLocalName(name) => name.len() + size_of::<u8>(),
        }) as u8
    }

    pub fn ad_type(&self) -> u8 {
        match self {
            AdvData::Flags(_) => 0x01,
            AdvData::ShortenedLocalName(_) => 0x08,
            AdvData::CompleteLocalName(_) => 0x09,
        }
    }
}

impl WriteHCI for AdvData {
    fn write_into<W: Write>(&self, w: &mut W) -> Result<(), W::Error> {
        self.len().write_into(w)?;
        self.ad_type().write_into(w)?;

        match self {
            AdvData::Flags(flags) => flags.write_into(w)?,
            AdvData::ShortenedLocalName(name) => name.as_bytes().write_into(w)?,
            AdvData::CompleteLocalName(name) => name.as_bytes().write_into(w)?,
        };

        Ok(())
    }
}
