use std::{str::FromStr as _, sync::Mutex, thread};

use embassy_time::{Duration, Instant};
use esp_idf_hal::{io::EspIOError, modem::WifiModemPeripheral, peripheral::Peripheral};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::{
        server::{Configuration as HttpServerConfig, EspHttpServer},
        Method,
    },
    nvs::{EspNvs, EspNvsPartition, NvsDefault},
    wifi::{ClientConfiguration, Configuration, EspWifi},
};
use log::{info, warn};

static INDEX_HTML: &str = include_str!("./index.html");
static WIFI: Mutex<Option<EspWifi>> = Mutex::new(None);

pub fn setup<M, MODEM>(
    modem: MODEM,
    nvs: EspNvsPartition<NvsDefault>,
    storage: &'static Mutex<EspNvs<NvsDefault>>,
) -> anyhow::Result<()>
where
    M: WifiModemPeripheral,
    MODEM: Peripheral<P = M> + 'static,
{
    // Setup wifi
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let wifi_driver = EspWifi::new(modem, sys_loop, Some(nvs))?;
    *WIFI.lock().unwrap() = Some(wifi_driver);

    // Setup HTTP server
    let http_server = Box::leak(Box::new(EspHttpServer::new(&HttpServerConfig::default())?));

    // define routes
    http_server.fn_handler("/", Method::Get, |request| {
        // Respond with OK status
        let mut response = request.into_ok_response()?;
        // Return Requested Object (Index Page)
        response.write(get_index().as_bytes())?;
        Ok::<(), EspIOError>(())
    })?;

    // load ssid and password
    let ssid_buff = &mut [0; 50];
    let ssid = storage.lock().unwrap().get_str("SSID", ssid_buff).unwrap();
    if !ssid.is_some_and(|ssid| !ssid.is_empty()) {
        warn!("ssid not set, skipping Wifi setup");
        return Ok(());
    };
    let ssid = ssid.unwrap();

    let pass_buff = &mut [0; 50];
    let password = storage
        .lock()
        .unwrap()
        .get_str("PASSWORD", pass_buff)
        .unwrap();
    if !password.is_some_and(|pw| !pw.is_empty()) {
        warn!("password not set, skipping Wifi setup");
        return Ok(());
    };
    let password = password.unwrap();

    info!("Setting up wifi for SSID '{ssid}' with password: '{password}'");
    setup_wifi(ssid, password)?;

    Ok(())
}

fn get_index() -> String {
    INDEX_HTML.replace("<IP-ADDRESS>", &get_ip())
}

pub fn setup_wifi(ssid: &str, password: &str) -> anyhow::Result<()> {
    info!("Setting up wifi");
    let mut wifi = WIFI.lock().unwrap();
    let Some(wifi_driver) = wifi.as_mut() else {
        warn!("Wifi Driver is not setup");
        return Ok(());
    };
    // stop current connection
    if wifi_driver.is_connected()? {
        wifi_driver.disconnect()?;
    }

    wifi_driver
        .set_configuration(&Configuration::Client(ClientConfiguration {
            ssid: heapless::String::from_str(ssid).unwrap(),
            password: heapless::String::from_str(password).unwrap(),
            ..Default::default()
        }))
        .unwrap();
    if !wifi_driver.is_started()? {
        wifi_driver.start()?;
    }
    wifi_driver.connect()?;
    let start = Instant::now();
    // try to connect with timeout
    while !wifi_driver.is_connected()? || wifi_driver.sta_netif().get_ip_info()?.ip.is_unspecified()
    {
        if start.elapsed() > Duration::from_secs(10) {
            warn!("Could not connect to wifi");
            return Ok(());
        }
        thread::sleep(std::time::Duration::from_millis(100));
    }
    let ip = wifi_driver.sta_netif().get_ip_info()?.ip.to_string();
    info!("\n#######################\n# IP: {ip:15} #\n#######################",);
    Ok(())
}

pub fn get_ip() -> String {
    match WIFI.lock().unwrap().as_ref() {
        Some(wifi) => wifi
            .sta_netif()
            .get_ip_info()
            .map(|ip_info| ip_info.ip.to_string())
            .unwrap_or_default(),
        None => String::default(),
    }
}
