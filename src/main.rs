#![no_std]
#![no_main]

use ble::{
    hci::{
        HCICommand, HCIEvent, HCIPacket, LEMetaEvent, ScanEnableCommand, SetScanParametersCommand,
    },
    Ble,
};
use esp_backtrace as _;
use esp_hal::{prelude::*, timer::timg::TimerGroup};
use esp_wifi::ble::controller::BleConnector;

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

    for packet in ble.read() {
        match packet {
            HCIPacket::Event(event) => match HCIEvent::from_packet(&event) {
                HCIEvent::CommandComplete(event) => {
                    log::info!("{:?}", event);
                }
                HCIEvent::LEMetaEvent(event) => match event {
                    LEMetaEvent::AdvertisingReport(event) => {
                        for report in event.reports {
                            log::info!("{:?}", report)
                        }
                    }
                    _ => unimplemented!(),
                },
            },
            _ => unimplemented!(),
        }
    }

    loop {}
}
