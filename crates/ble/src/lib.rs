#![no_std]

pub mod packet;

use embedded_io::{ErrorType, Read, Write};
use esp_wifi::ble::controller::{BleConnector, BleConnectorError};
use packet::{HCICommandPacket, HCIPacket};

pub struct Ble<'d> {
    connector: &'d mut BleConnector<'d>,
    buf: [u8; 255],
}

impl<'d> ErrorType for Ble<'d> {
    type Error = BleConnectorError;
}

impl<'d> Ble<'d> {
    pub fn new(connector: &'d mut BleConnector<'d>) -> Ble<'d> {
        Ble {
            connector,
            buf: [0; 255],
        }
    }

    pub fn reset(&mut self) -> Result<usize, BleConnectorError> {
        self.write(
            HCICommandPacket {
                opcode: 0x0C03,
                length: 0,
                parameters: &[],
            }
            .into(),
        )
    }

    pub fn set_le_scan_enable(&mut self) -> Result<usize, BleConnectorError> {
        self.write(
            HCICommandPacket {
                opcode: 0x0C20,
                length: 0x02,
                parameters: &[0x01, 0x00],
            }
            .into(),
        )
    }

    pub fn set_le_scan_parameters(&mut self) -> Result<usize, BleConnectorError> {
        self.write(
            HCICommandPacket {
                opcode: 0x0B20,
                length: 0x07,
                parameters: &[0x01, 0x10, 0x00, 0x10, 0x00, 0x00, 0x00],
            }
            .into(),
        )
    }

    pub fn write(&mut self, packet: HCIPacket) -> Result<usize, BleConnectorError> {
        let len = packet.read(&mut self.buf).unwrap();
        self.connector.write(&self.buf[..len])
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, BleConnectorError> {
        self.connector.read(buf)
    }
}
