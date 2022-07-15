use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type Date = DateTime<Utc>;

#[derive(Serialize, Deserialize, Debug)]
enum DeviceType {
    Computer,
    Laptop,
    Mobile,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Device {
    name: String,
    id: Uuid,
    owner: Uuid,
    deviceType: DeviceType,
    lastOnline: Date,
}

#[allow(non_snake_case)]
pub struct User {
    name: String,
    id: Uuid,
    devices: Vec<Device>,
    lastOnline: Date,
}


#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Payload {
    to: Uuid,
    from: Uuid,
    crossUser: bool,
    msg: String,
}
