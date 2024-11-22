#![no_std]
#![no_main]

use ble::Ble;
use esp_backtrace as _;
use esp_hal::{prelude::*, timer::timg::TimerGroup};
use esp_println;
use esp_wifi::ble::controller::BleConnector;

const MAX_BLE_PACKET_SIZE: usize = 255;

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    esp_alloc::heap_allocator!(72 * 1024);

    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

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
            Ok(len) => log::info!("{:?}", &buf[0..len]),
            Err(err) => log::warn!("{:?}", err),
        };
    }
}
