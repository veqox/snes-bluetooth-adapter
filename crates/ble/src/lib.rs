#![no_std]

pub mod hci;

use embedded_io::Write;
use esp_wifi::ble::controller::{BleConnector, BleConnectorError};
use hci::{HCICommand, HCIPacket};

pub struct Ble<'d> {
    connector: BleConnector<'d>,
}

impl<'d> Ble<'d> {
    pub fn new(connector: BleConnector<'d>) -> Ble<'d> {
        Ble { connector }
    }

    pub fn write(&mut self, command: HCICommand) -> Result<usize, BleConnectorError> {
        let mut buf = [0; 258];
        let len = command
            .write_into(&mut buf)
            .ok_or_else(|| BleConnectorError::Unknown)?;
        self.connector.write(&buf[..len])
    }

    pub fn read(&mut self) -> Option<HCIPacket> {
        let mut buf = [0; 255];
        loop {
            match self.connector.get_next(&mut buf) {
                Err(err) => log::warn!("{:?}", err),
                Ok(0) => continue,
                Ok(len) => return Some(HCIPacket::from_buf(&buf[..len])?),
            }
        }
    }
}
