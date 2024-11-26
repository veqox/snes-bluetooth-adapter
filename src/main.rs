#![no_std]
#![no_main]

use ble::{
    packet::{HCICommand, HCIEventCode, HCIPacket, ScanEnableCommand, SetScanParametersCommand},
    Ble,
};
use esp_backtrace as _;
use esp_hal::{prelude::*, timer::timg::TimerGroup};
use esp_wifi::ble::controller::BleConnector;
use utils::Reader;

const MAX_BLE_PACKET_SIZE: usize = 255;

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    esp_alloc::heap_allocator!(72 * 1024);

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);

    let init = esp_wifi::init(
        esp_wifi::EspWifiInitFor::Ble,
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap_or_else(|err| {
        log::error!("{:?}", err);
        panic!("ble failed to initialize");
    });

    let mut connector = BleConnector::new(&init, peripherals.BT);
    let mut ble = Ble::new(&mut connector);
    ble.write(HCICommand::Reset).expect("hci failed to reset");
    ble.write(HCICommand::SetScanParameters(SetScanParametersCommand {
        scan_type: 0x01,
        scan_interval: 0x10,
        scan_window: 0x10,
        own_address_type: 0x00,
        scanning_filter_policy: 0x00,
    }))
    .expect("hci failed to set scan parameters");
    ble.write(HCICommand::ScanEnable(ScanEnableCommand {
        scan_enable: 0x01,
        filter_duplicates: 0x01,
    }))
    .expect("hci failed to enable scan");

    loop {
        let mut buf = [0; MAX_BLE_PACKET_SIZE];

        match ble.read(&mut buf) {
            Ok(0) => continue,
            Ok(len) => match HCIPacket::read_from_slice(&buf[0..len]) {
                Ok(packet) => match packet {
                    HCIPacket::HCIEventPacket(packet) => match packet.evcode {
                        HCIEventCode::CommandComplete => log::info!("hci: command complete"),
                        HCIEventCode::LEMetaEvent => {
                            let mut reader = Reader::new(packet.parameters);
                            let sub_event_code =
                                ble::packet::LESubeventCode::from(reader.read_u8());
                            let num_of_reports = reader.read_u8();

                            log::info!("hci: LE advertising report");
                            log::info!("sub_event_code: {:?}", sub_event_code);
                            log::info!("num_of_reports: {}", num_of_reports);

                            for _ in 0..num_of_reports {
                                let event_type = reader.read_u8();
                                let address_type = reader.read_u8();
                                let address = reader.read_slice(6);
                                let data_length = reader.read_u8();
                                let data = reader.read_slice(data_length as usize);
                                let rssi = reader.read_u8() as i8;

                                log::info!("event_type: {}", event_type);
                                log::info!("address_type: {}", address_type);
                                log::info!(
                                    "address: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                                    address[0],
                                    address[1],
                                    address[2],
                                    address[3],
                                    address[4],
                                    address[5],
                                );
                                log::info!("data_length: {}", data_length);
                                log::info!("data: {:?}", data);
                                log::info!("rss: {}", rssi);
                            }
                        }
                    },
                    _ => log::warn!("unexpected packet: {:?}", packet),
                },
                Err(err) => log::warn!("{:?}", err),
            },
            Err(err) => log::warn!("{:?}", err),
        };
    }
}
