#![no_std]
#![no_main]

use ble::hci::{
    command::{Command, Reset, SetAdvData, SetAdvEnable, SetAdvParameters, SetScanResponseData},
    gap::{AD_FLAG_BR_EDR_NOT_SUPPORTED, AD_FLAG_GENERAL_DISCOVERABLE_MODE, AdvData},
};

use esp_hal::{chip, clock, interrupt::software, main, timer::timg};
use esp_println::logger;
use esp_radio::ble::controller::{self};
use log::{info, warn};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[main]
fn main() -> ! {
    logger::init_logger(log::LevelFilter::Info);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let config = esp_hal::Config::default().with_cpu_clock(clock::CpuClock::max());
    let peripherals = esp_hal::init(config);
    let timg0 = timg::TimerGroup::new(peripherals.TIMG0);
    let sw_int = software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);

    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);

    let radio_controller = esp_radio::init().unwrap();
    let mut connector =
        controller::BleConnector::new(&radio_controller, peripherals.BT, Default::default())
            .unwrap();

    Reset::new()
        .write_into(&mut connector)
        .expect("failed to send reset command");

    SetAdvParameters::new(
        0x0800,
        0x0800,
        0x00,
        0x00,
        0x00,
        [0, 0, 0, 0, 0, 0],
        0x01,
        0x00,
    )
    .write_into(&mut connector)
    .expect("failed to send set_adv_parameters command");

    SetAdvData::new(&[
        AdvData::Flags(AD_FLAG_BR_EDR_NOT_SUPPORTED | AD_FLAG_GENERAL_DISCOVERABLE_MODE),
        AdvData::ShortenedLocalName(chip!()),
        AdvData::CompleteLocalName(chip!()),
    ])
    .write_into(&mut connector)
    .expect("failed to send set_adv_data command");

    SetScanResponseData::new(&[
        AdvData::Flags(AD_FLAG_BR_EDR_NOT_SUPPORTED | AD_FLAG_GENERAL_DISCOVERABLE_MODE),
        AdvData::ShortenedLocalName(chip!()),
        AdvData::CompleteLocalName(chip!()),
    ])
    .write_into(&mut connector)
    .expect("failed to send set_scan_response_data command");

    SetAdvEnable::new(0x01)
        .write_into(&mut connector)
        .expect("failed to send set_adv_enable command");

    let mut buf = [0; 255];

    loop {
        match connector.next(&mut buf) {
            Ok(0) => continue,
            Ok(len) => info!("received {} bytes: {:?}", len, &buf[..len]),
            Err(err) => warn!("read failed with {}", err),
        }
    }
}
