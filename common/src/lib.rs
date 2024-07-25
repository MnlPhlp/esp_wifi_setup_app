use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

pub const SERVICE_UUID: Uuid = uuid!("2470ef7b-4e58-404f-9ac5-a36e5583ce54");
/// characteristic to set wifi login data
pub const WIFI_UUID: Uuid = uuid!("93246f8c-2bf2-4633-abe3-5a085ce72f30");
/// characteristic to get ip of the esp
pub const IP_UUID: Uuid = uuid!("e36e0c05-238d-4eb4-b2de-7088d2cf62a1");

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub ssid: String,
    pub password: String,
}

impl LoginData {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn from_bytes(data: &[u8]) -> Self {
        bincode::deserialize(data).unwrap()
    }
}
