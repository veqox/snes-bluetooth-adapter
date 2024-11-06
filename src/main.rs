#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, prelude::*};

#[entry]
fn main() -> ! {
    #[allow(unused)]
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    esp_println::logger::init_logger_from_env();

    let mut state = state::State::default();
    state.set_b(false);
    state.set_y(false);
    state.set_start(false);

    log::info!("{}", state);

    loop {
        let cycle = state.cycle();
        let btn_state = state.next();
        log::info!("{}: {}", cycle, btn_state);
        delay.delay(500.millis());
    }
}
