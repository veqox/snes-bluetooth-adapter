use crate::hci::{BufWriter, WriteHCI, gap::AdvData};
use ble_macros::command;
use embedded_io::Write;

// const OGF_LINK_CONTROL_COMMAND: u16 = 0x01;
// const OGF_LINK_POLICY_COMMAND: u16 = 0x02;
const OGF_CONTROL_AND_BASEBAND_COMMAND: u16 = 0x03;
// const OGF_INFORMATIONAL_PARAMETERS_COMMAND: u16 = 0x04;
// const OGF_STATUS_PARAMETERS_COMMAND: u16 = 0x05;
// const OGF_TESTING_COMMAND: u16 = 0x06;
const OGF_LE_CONTROLLER_COMMAND: u16 = 0x08;

pub trait Command {
    const TYPE: u8 = 0x01;
    const OGF: u16;
    const OCF: u16;
    const OPCODE: u16 = (Self::OGF << 10) | Self::OCF;
    const PARAM_LEN: u8;

    fn write_into<W: Write>(&self, w: &mut W) -> Result<(), W::Error> {
        Self::TYPE.write_into(w)?;
        Self::OPCODE.write_into(w)?;
        Self::PARAM_LEN.write_into(w)?;
        self.write_payload(w)?;

        Ok(())
    }

    fn write_payload<W: Write>(&self, w: &mut W) -> Result<(), W::Error>;
}

#[command(ogf = OGF_CONTROL_AND_BASEBAND_COMMAND, ocf = 0x03)]
pub struct Reset {}

#[command(ogf = OGF_LE_CONTROLLER_COMMAND, ocf = 0x06)]
pub struct SetAdvParameters {
    pub interval_min: u16,
    pub interval_max: u16,
    pub advertising_type: u8,
    pub own_address_type: u8,
    pub peer_address_type: u8,
    pub peer_address: [u8; 6],
    pub advertising_channel_map: u8,
    pub advertising_filter_policy: u8,
}

pub struct SetAdvData<'a> {
    pub data: &'a [AdvData],
}

impl<'a> SetAdvData<'a> {
    pub fn new(data: &'a [AdvData]) -> Self {
        Self { data }
    }
}

impl<'a> Command for SetAdvData<'_> {
    const OGF: u16 = OGF_LE_CONTROLLER_COMMAND;
    const OCF: u16 = 0x08;
    const PARAM_LEN: u8 = 32;

    fn write_payload<W: Write>(&self, w: &mut W) -> Result<(), W::Error> {
        let mut buf = [0; (Self::PARAM_LEN - 1) as usize];
        let mut writer = BufWriter::new(&mut buf);

        for ad in self.data {
            ad.write_into(&mut writer).unwrap();
        }

        (writer.pos as u8).write_into(w)?;
        writer.buf.write_into(w)?;

        Ok(())
    }
}

pub struct SetScanResponseData<'a> {
    pub data: &'a [AdvData],
}

impl<'a> SetScanResponseData<'a> {
    pub fn new(data: &'a [AdvData]) -> Self {
        Self { data }
    }
}

impl<'a> Command for SetScanResponseData<'_> {
    const OGF: u16 = OGF_LE_CONTROLLER_COMMAND;
    const OCF: u16 = 0x09;
    const PARAM_LEN: u8 = 32;

    fn write_payload<W: Write>(&self, w: &mut W) -> Result<(), W::Error> {
        let mut buf = [0; (Self::PARAM_LEN - 1) as usize];
        let mut writer = BufWriter::new(&mut buf);

        for ad in self.data {
            ad.write_into(&mut writer).unwrap();
        }

        (writer.pos as u8).write_into(w)?;
        writer.buf.write_into(w)?;

        Ok(())
    }
}

#[command(ogf = OGF_LE_CONTROLLER_COMMAND, ocf = 0x0A)]
pub struct SetAdvEnable {
    pub enable: u8,
}
