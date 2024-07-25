# esp_wifi_setup_app
Example project for using a flutter app with rust backend and the blec crate to setup wifi credentials on an esp32 running rust

## Building the App
The app uses [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) to call rust code from dart.

```sh
cargo install flutter_rust_bridge_codgen
flutter_rust_bridge_codgen generate
flutter run
```

### Building the ESP32 firmware
The `flutter_app` example works together with the `esp_server` example to set wifi credentials for the esp32.
To run the code on the esp you have to setup the rust toolchain for esp (see [esp-rs esp-idf-template](https://github.com/esp-rs/esp-idf-template#prerequisites)).
Then connect a esp32 to your pc and run `cargo run` in the `esp_server` folder. After that you should be able to use the app to connect and setup wifi on the esp.
