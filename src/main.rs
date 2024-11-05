#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, prelude::*};

#[entry]
fn main() -> ! {
    #[allow(unused)]
    let delay = Delay::new();

    esp_println::logger::init_logger_from_env();

    loop {
        log::info!("Hello world!");
        delay.delay(500.millis());
    }
}
