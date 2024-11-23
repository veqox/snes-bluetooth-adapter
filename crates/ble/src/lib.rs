#![no_std]
#![allow(dead_code)]

pub mod packet;

use embedded_io::{Read, Write};
use esp_wifi::ble::controller::{BleConnector, BleConnectorError};
use packet::{HCICommandPacket, HCIPacket};

pub struct Ble<'d> {
    connector: &'d mut BleConnector<'d>,
    buf: [u8; 1000],
}

impl<'d> Ble<'d> {
    pub fn new(connector: &'d mut BleConnector<'d>) -> Ble<'d> {
        Ble {
            connector,
            buf: [0; 1000],
        }
    }

    pub fn reset(&mut self) -> Result<usize, BleConnectorError> {
        self.write(
            HCICommandPacket {
                opcode: packet::HCI_RESET_COMMAND,
                parameters: &[],
            }
            .into(),
        )
    }

    pub fn set_le_scan_enable(&mut self) -> Result<usize, BleConnectorError> {
        self.write(
            HCICommandPacket {
                opcode: packet::HCI_SET_SCAN_ENABLE_COMMAND,
                parameters: &[0x01, 0x00],
            }
            .into(),
        )
    }

    pub fn set_le_scan_parameters(&mut self) -> Result<usize, BleConnectorError> {
        // 0x01: active scanning
        // 0x10 + 0x00: scan interval
        // 0x10 + 0x00: scan window
        // 0x00: own address type
        // 0x00: filter policy
        self.write(
            HCICommandPacket {
                opcode: packet::HCI_SET_SCAN_ENABLE_COMMAND,
                parameters: &[0x01, 0x10, 0x00, 0x10, 0x00, 0x00, 0x00],
            }
            .into(),
        )
    }

    pub fn write(&mut self, packet: HCIPacket) -> Result<usize, BleConnectorError> {
        let len = packet.write_to_buffer(&mut self.buf).unwrap();
        self.connector.write(&self.buf[..len])
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, BleConnectorError> {
        self.connector.read(buf)
    }
}
