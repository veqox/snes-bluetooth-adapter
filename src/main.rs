#![no_std]
#![no_main]

use embedded_io::{Read, Write};
use esp_backtrace as _;
use esp_hal::{prelude::*, timer::timg::TimerGroup};
use esp_println;
use esp_wifi::ble::controller::BleConnector;

const MAX_BLE_PACKET_SIZE: usize = 33;

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

    match connector.write(&[0x08, 0x09]) {
        Ok(len) => log::info!("{} bytes written", len),
        Err(err) => log::warn!("{:?}", err),
    }

    loop {
        let mut buf = [0; MAX_BLE_PACKET_SIZE];

        match connector.read(&mut buf) {
            Ok(0) => continue,
            Ok(len) => log::info!("{:?}", &buf[0..len]),
            Err(err) => log::warn!("{:?}", err),
        };
    }
}
