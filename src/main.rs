#![no_std]
#![no_main]

use ble::{
    hci::{
        AdvertisingData, HCICommand, HCIEvent, HCIPacket, SetAdvertisingParametersCommand,
        AD_FLAG_BR_EDR_NOT_SUPPORTED, AD_FLAG_GENERAL_DISCOVERABLE_MODE,
    },
    Ble,
};
use esp_backtrace as _;
use esp_hal::{chip, prelude::*, timer::timg::TimerGroup};
use esp_wifi::ble::controller::BleConnector;

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    esp_alloc::heap_allocator!(72 * 1024);

    loop {
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
                AdvertisingData::IncompleteListOf16BitServiceUUIDs(&[0x1809]),
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
                AdvertisingData::IncompleteListOf16BitServiceUUIDs(&[0x1809]),
            ],
        })
        .expect("hci failed to set scan response data");
        ble.write(HCICommand::SetAdvertisingEnable { enable: 0x0 })
            .expect("hci failed to enable advertising");

        for packet in ble.read() {
            match packet {
                HCIPacket::Event(event) => match HCIEvent::from_packet(&event) {
                    Some(event) => log::info!("{:?}", event),
                    None => log::warn!("parsing went to shit"),
                },
                _ => unimplemented!(),
            }
        }
    }
}
