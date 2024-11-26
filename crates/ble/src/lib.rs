#![no_std]
#![allow(dead_code)]

pub mod packet;

use embedded_io::{Read, Write};
use esp_wifi::ble::controller::{BleConnector, BleConnectorError};
use packet::{HCICommand, HCI_COMMAND_MAX_PACKET_SIZE};

pub struct Ble<'d> {
    connector: &'d mut BleConnector<'d>,
}

impl<'d> Ble<'d> {
    pub fn new(connector: &'d mut BleConnector<'d>) -> Ble<'d> {
        Ble { connector }
    }

    pub fn write(&mut self, command: HCICommand) -> Result<usize, BleConnectorError> {
        let mut buf = [0; HCI_COMMAND_MAX_PACKET_SIZE];
        let len = command.write_to_buffer(&mut buf);
        self.connector.write(&buf[..len])
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, BleConnectorError> {
        self.connector.read(buf)
    }
}
