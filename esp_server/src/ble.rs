use std::sync::Mutex;

use esp32_nimble::{BLEDevice, NimbleProperties};
use esp_idf_svc::nvs::{EspNvs, NvsDefault};

use common::{LoginData, IP_UUID, SERVICE_UUID, WIFI_UUID};
use log::info;

use crate::web;

pub fn setup(storage: &'static Mutex<EspNvs<NvsDefault>>) -> anyhow::Result<()> {
    let ble_dev = BLEDevice::take();
    let adv = ble_dev.get_advertising();
    let server = ble_dev.get_server();
    server.on_connect(|_, _| {
        adv.lock().stop().unwrap();
    });
    server.on_disconnect(|_, _| {
        adv.lock().start().unwrap();
    });

    let service = server.create_service(SERVICE_UUID.into());

    // get ip
    let ip_charac = service.lock().create_characteristic(
        IP_UUID.into(),
        NimbleProperties::READ | NimbleProperties::NOTIFY,
    );
    ip_charac
        .lock()
        .on_read(move |val, _| val.set_value(web::get_ip().as_bytes()));

    // set wifi information
    let wifi_charac = service
        .lock()
        .create_characteristic(WIFI_UUID.into(), NimbleProperties::WRITE);
    wifi_charac.lock().on_write(move |args| {
        let data = LoginData::from_bytes(args.recv_data());
        info!(
            "Setting wifi login to\n  ssid: {}\n  password: {}",
            data.ssid, data.password
        );
        let mut storage = storage.lock().unwrap();
        storage.set_str("SSID", &data.ssid).unwrap();
        storage.set_str("PASSWORD", &data.password).unwrap();
        web::setup_wifi(&data.ssid, &data.password).unwrap();
        let mut ip = ip_charac.lock();
        ip.set_value(web::get_ip().as_bytes());
        ip.notify();
    });

    let mut adv = adv.lock();
    adv.set_data(esp32_nimble::BLEAdvertisementData::new().name("esp_server"))?;
    adv.start()?;

    Ok(())
}
