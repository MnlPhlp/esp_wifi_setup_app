use log::LevelFilter;

// setup logging
flutter_logger::flutter_logger_init!(LevelFilter::Info);

#[frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
