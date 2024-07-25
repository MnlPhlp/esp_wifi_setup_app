pub use blec::{BleAddress, BleDevice};
use common::*;
use flutter_rust_bridge::frb;
use log::{error, info, warn};
use tokio::sync::mpsc;

use crate::{api::state, frb_generated::StreamSink};

pub fn setup_ble() {
    info!("setting up ble");
    match blec::init() {
        Ok(_) => info!("ble setup complete"),
        Err(e) => error!("error setting up ble: {:?}", e),
    }
}

pub fn start_scan(sink: StreamSink<Vec<BleDevice>>) {
    let (tx, mut rx) = mpsc::channel(1);
    match blec::discover(tx, 3000) {
        Ok(_) => info!("scan started"),
        Err(e) => error!("error starting scan: {:?}", e),
    }
    while let Some(devices) = rx.blocking_recv() {
        sink.add(devices).unwrap();
    }
}

pub async fn connect(address: BleAddress) {
    if state::bt_connected() {
        info!("disconnecting");
        blec::disconnect().await.unwrap();
        info!("ble disconnected");
        return;
    }
    info!("connecting to {}", address);
    match blec::connect(
        address,
        SERVICE_UUID,
        vec![WIFI_UUID, IP_UUID],
        Some(|| state::set_bt_connected(false)),
    )
    .await
    {
        Ok(_) => {
            state::set_bt_connected(true);
            info!("ble connected");
            // listen for ip updates
            blec::subscribe(IP_UUID, |data| {
                let ip = String::from_utf8(data.to_vec()).unwrap();
                info!("received ip: {:?}", ip);
                state::set_ip(ip);
            })
            .await
            .unwrap();
            // read current ip once
            state::set_ip(read_ip().await);
        }
        Err(e) => error!("Error connecting to ble: {e}"),
    }
}

pub async fn send_wifi_data(ssid: String, password: String) {
    if !state::bt_connected() {
        warn!("no ble device connected");
        return;
    }
    info!("sending ssid: '{ssid}', password: '{password}'");
    let data = LoginData { ssid, password };
    blec::send_data(WIFI_UUID, data.to_bytes()).await.unwrap();
}

pub async fn read_ip() -> String {
    if !state::bt_connected() {
        return String::new();
    }
    let data = blec::recv_data(IP_UUID).await.unwrap();
    String::from_utf8(data.to_vec()).unwrap()
}

#[frb(sync)]
pub fn format_address(address: BleAddress) -> String {
    format!("{}", address)
}

#[frb(mirror(BleDevice))]
struct _BleDevice {
    address: BleAddress,
    name: String,
    is_connected: bool,
}

#[frb(mirror(BleAddress))]
struct _BleAddress {
    address: [u8; 6],
}
