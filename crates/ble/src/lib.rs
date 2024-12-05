#![no_std]

pub mod hci;

use embedded_io::Write;
use esp_wifi::ble::controller::{BleConnector, BleConnectorError};
use hci::{HCICommand, HCIPacket};

pub struct Ble<'d> {
    connector: &'d mut BleConnector<'d>,
}

impl<'d> Ble<'d> {
    pub fn new(connector: &'d mut BleConnector<'d>) -> Ble<'d> {
        Ble { connector }
    }

    pub fn write(&mut self, command: HCICommand) -> Result<usize, BleConnectorError> {
        let mut buf = [0; 258];
        let len = command.write_into(&mut buf).ok_or_else(|| {
            log::warn!("we got apeshit");
            BleConnectorError::Unknown
        })?;
        self.connector.write(&buf[..len])
    }

    pub fn read(self) -> HCIPacketIterator<'d> {
        HCIPacketIterator::new(self)
    }
}

pub struct HCIPacketIterator<'d> {
    ble: Ble<'d>,
}

impl<'d> HCIPacketIterator<'d> {
    pub fn new(ble: Ble<'d>) -> Self {
        Self { ble }
    }
}

impl<'d> Iterator for HCIPacketIterator<'d> {
    type Item = HCIPacket;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 255];
        loop {
            match self.ble.connector.get_next(&mut buf) {
                Err(err) => log::info!("{:?}", err),
                Ok(0) => continue,
                Ok(len) => return Some(HCIPacket::from_buf(&buf[..len])?),
            }
        }
    }
}
