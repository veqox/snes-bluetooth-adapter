#![no_std]
#![no_main]

use embedded_io::Write;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{chip, main};

use ble::hci::{
    AdvertisingData, HCICommand, HCIEvent, HCIPacket, SetAdvertisingParametersCommand,
    AD_FLAG_BR_EDR_NOT_SUPPORTED, AD_FLAG_GENERAL_DISCOVERABLE_MODE,
};
use esp_println::logger;
use esp_wifi::ble::controller::{BleConnector, BleConnectorError};
use log::{error, info, warn};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("{}", info);
    loop {}
}

extern crate alloc;

#[main]
fn main() -> ! {
    logger::init_logger_from_env();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    let connector = BleConnector::new(&_init, peripherals.BT);
    let mut ble = Ble::new(connector);

    ble.write(HCICommand::Reset).expect("hci failed to reset");
    ble.write(HCICommand::SetAdvertisingParameters(
        SetAdvertisingParametersCommand {
            interval_min: 0x0800,
            interval_max: 0x0800,
            advertising_type: 0x00,
            own_address_type: 0x00,
            peer_address_type: 0x00,
            peer_address: [0, 0, 0, 0, 0, 0],
            advertising_channel_map: 0x01,
            advertising_filter_policy: 0x00,
        },
    ))
    .expect("hci failed to set advertisment parameters");
    ble.write(HCICommand::SetAdvertisingData {
        data: &[
            AdvertisingData::ShortenedLocalName(chip!()),
            AdvertisingData::CompleteLocalName(chip!()),
            AdvertisingData::Flags(
                AD_FLAG_BR_EDR_NOT_SUPPORTED | AD_FLAG_GENERAL_DISCOVERABLE_MODE,
            ),
        ],
    })
    .expect("hci failed to set advertisment data");
    ble.write(HCICommand::SetScanResponseData {
        data: &[
            AdvertisingData::ShortenedLocalName(chip!()),
            AdvertisingData::CompleteLocalName(chip!()),
            AdvertisingData::Flags(
                AD_FLAG_BR_EDR_NOT_SUPPORTED | AD_FLAG_GENERAL_DISCOVERABLE_MODE,
            ),
        ],
    })
    .expect("hci failed to set scan response data");
    ble.write(HCICommand::SetAdvertisingEnable { enable: 0x1 })
        .expect("hci failed to enable advertising");

    let mut buf = [0; 255];

    loop {
        while let Some(packet) = ble.read(&mut buf) {
            match packet {
                HCIPacket::Event(event) => match HCIEvent::from_packet(&event) {
                    Some(event) => info!("{:?}", event),
                    None => warn!("received None event"),
                },
                packet => info!("{:?}", packet),
            }
        }
    }
}

pub struct Ble<'d> {
    connector: BleConnector<'d>,
}

impl<'d> Ble<'d> {
    pub fn new(connector: BleConnector<'d>) -> Ble<'d> {
        Ble { connector }
    }

    pub fn write(&mut self, command: HCICommand) -> Result<usize, BleConnectorError> {
        let mut buf = [0; 258];
        let len = command
            .write_into(&mut buf)
            .map_err(|_| BleConnectorError::Unknown)?;

        self.connector.write(&buf[..len])
    }

    pub fn read<'p>(&mut self, buf: &'p mut [u8]) -> Option<HCIPacket<'p>> {
        loop {
            match self.connector.next(buf) {
                Err(err) => warn!("{:?}", err),
                Ok(0) => continue,
                Ok(len) => return HCIPacket::from_buf(&buf[..len]),
            }
        }
    }
}
