#![no_std]
#![no_main]

use ble::{
    packet::{HCIEventCode, HCIPacket},
    Ble,
};
use esp_backtrace as _;
use esp_hal::{prelude::*, timer::timg::TimerGroup};
use esp_wifi::ble::controller::BleConnector;

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
    ble.set_le_scan_parameters()
        .expect("hci failed to set scan parameters");
    ble.set_le_scan_enable().expect("hci failed to enable scan");

    loop {
        let mut buf = [0; MAX_BLE_PACKET_SIZE];

        match ble.read(&mut buf) {
            Ok(0) => continue,
            Ok(len) => match HCIPacket::read_from_slice(&buf[0..len]) {
                Ok(packet) => match packet {
                    HCIPacket::HCIEventPacket(packet) => match packet.evcode {
                        HCIEventCode::CommandComplete => log::info!("hci: LE connection complete"),
                        HCIEventCode::LEMetaEvent => {
                            log::info!("hci: LE advertising report");
                            log::info!(
                                "sub_event_code: {:?}",
                                ble::packet::LESubeventCode::from(packet.parameters[0]),
                            );
                            log::info!("num_of_reports: {}", packet.parameters[1]);
                            for i in 0..packet.parameters[1] {
                                let offset = 2 + i as usize * 12;
                                log::info!("event_type: {}", packet.parameters[offset]);
                                log::info!("address_type: {}", packet.parameters[offset + 1]);
                                log::info!(
                                    "address: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                                    packet.parameters[offset + 2],
                                    packet.parameters[offset + 3],
                                    packet.parameters[offset + 4],
                                    packet.parameters[offset + 5],
                                    packet.parameters[offset + 6],
                                    packet.parameters[offset + 7],
                                );
                                log::info!("data_length: {}", packet.parameters[offset + 8]);
                                log::info!(
                                    "data: {:?}",
                                    &packet.parameters[offset + 9
                                        ..offset + 9 + packet.parameters[offset + 8] as usize]
                                );
                                log::info!(
                                    "rss: {}",
                                    packet.parameters[10 + packet.parameters[offset + 8] as usize]
                                        as i8
                                )
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
