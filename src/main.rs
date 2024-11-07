#![no_std]
#![no_main]

use core::cell::RefCell;

use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Event, Input, Io, Level, Output, Pull},
    prelude::*,
};
use state::State;

static SERIAL: Mutex<RefCell<Option<Output>>> = Mutex::new(RefCell::new(None));
static LATCH: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
static CLOCK: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
static STATE: Mutex<RefCell<Option<State>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    #[allow(unused)]
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    let mut io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    io.set_interrupt_handler(interrupt_handler);

    let serial = Output::new(io.pins.gpio4, Level::High);
    let mut clock = Input::new(io.pins.gpio5, Pull::Up);
    let mut latch = Input::new(io.pins.gpio6, Pull::Up);
    let mut state = State::default();
    state.set_a(true);

    critical_section::with(|cs| {
        latch.listen(Event::FallingEdge);
        LATCH.borrow_ref_mut(cs).replace(latch);

        clock.listen(Event::RisingEdge);
        CLOCK.borrow_ref_mut(cs).replace(clock);

        STATE.borrow_ref_mut(cs).replace(state);

        SERIAL.borrow_ref_mut(cs).replace(serial);
    });

    esp_println::logger::init_logger_from_env();

    loop {
        critical_section::with(|cs| STATE.borrow_ref_mut(cs).as_mut().unwrap().set_a(true));
        delay.delay_millis(1000);
        critical_section::with(|cs| STATE.borrow_ref_mut(cs).as_mut().unwrap().set_a(false));
        delay.delay_millis(1000)
    }
}

#[handler]
#[ram]
fn interrupt_handler() {
    critical_section::with(|cs| {
        if LATCH.borrow_ref(cs).as_ref().unwrap().is_interrupt_set() {
            STATE.borrow_ref_mut(cs).as_mut().unwrap().reset_cycle();

            SERIAL
                .borrow_ref_mut(cs)
                .as_mut()
                .unwrap()
                .set_level(STATE.borrow_ref_mut(cs).as_mut().unwrap().next().into());

            LATCH.borrow_ref_mut(cs).as_mut().unwrap().clear_interrupt()
        };
    });

    critical_section::with(|cs| {
        if CLOCK.borrow_ref(cs).as_ref().unwrap().is_interrupt_set() {
            if STATE.borrow_ref(cs).as_ref().unwrap().cycle() == 16 {
                SERIAL.borrow_ref_mut(cs).as_mut().unwrap().set_high();
            } else {
                SERIAL
                    .borrow_ref_mut(cs)
                    .as_mut()
                    .unwrap()
                    .set_level(STATE.borrow_ref_mut(cs).as_mut().unwrap().next().into());
            }

            CLOCK.borrow_ref_mut(cs).as_mut().unwrap().clear_interrupt()
        };
    });
}
