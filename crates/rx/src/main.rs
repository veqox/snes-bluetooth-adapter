#![no_std]
#![no_main]

use esp_hal::main;
use esp_println::logger;
use log::info;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[main]
fn main() -> ! {
    logger::init_logger(log::LevelFilter::Info);

    info!("Hello World");

    loop {}
}
