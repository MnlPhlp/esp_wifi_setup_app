mod ble;
mod web;

use std::sync::Mutex;

use edge_executor::block_on;
use edge_executor::Executor;
use embassy_time::Timer;
use esp_idf_hal::prelude::*;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::nvs::EspNvs;
use log::info;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    // setup timer
    esp_idf_svc::timer::embassy_time_driver::link();

    let ex: &mut Executor = Box::leak(Box::new(Executor::new()));
    let peripherals = Peripherals::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let storage = Box::leak(Box::new(Mutex::new(EspNvs::new(
        nvs.clone(),
        "storage",
        true,
    )?)));

    info!("Setting up ble");
    ble::setup(storage)?;

    info!("Setting up web server");
    web::setup(peripherals.modem, nvs, storage)?;

    info!("Starting tasks");
    block_on(ex.run(async move {
        loop {
            Timer::after_millis(500).await;
        }
    }));
    panic!("Executor exited")
}
