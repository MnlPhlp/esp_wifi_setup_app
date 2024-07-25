use paste::paste;
use std::sync::{OnceLock, RwLock};

use crate::frb_generated::StreamSink;

/// Creates getter and setter methods for a state field
macro_rules! get_set {
    ($name:ident, $type:ty) => {
        pub fn $name() -> $type {
            STATE.read().unwrap().$name.clone()
        }

        paste! {
            pub fn [<set_ $name>](val: $type) {
                let mut state = STATE.write().unwrap();
                state.$name = val;
                if let Some(sink) = STATE_SINK.get(){
                    sink.add(state.clone()).unwrap();
                }
            }
        }
    };
}

static STATE: RwLock<State> = RwLock::new(State::new());
static STATE_SINK: OnceLock<StreamSink<State>> = OnceLock::new();

#[derive(Default, Clone)]
pub struct State {
    pub bt_connected: bool,
    pub ip: String,
}
impl State {
    #[flutter_rust_bridge::frb(sync)]
    pub const fn new() -> Self {
        Self {
            bt_connected: false,
            ip: String::new(),
        }
    }
}

get_set!(bt_connected, bool);
get_set!(ip, String);

pub fn init_state_sink(sink: StreamSink<State>) {
    if STATE_SINK.set(sink).is_err() {
        log::error!("state sink already set");
    }
}
