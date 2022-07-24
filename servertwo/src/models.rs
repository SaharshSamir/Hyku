use std::str::FromStr;

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::Sender;
use warp::reject::Reject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
     Computer,
     Mobile,
}

impl FromStr for DeviceType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Computer" => Ok(DeviceType::Computer),
            "Mobile" => Ok(DeviceType::Mobile),
            _ => Err(()),

        }
    }
}

#[derive(Debug)]
pub struct AppError {
    pub reason: String
}

impl Reject for AppError{}

#[derive(Debug, Clone)]
pub struct Device {
    pub device_id: Uuid,
    pub name: String,
    pub device_type: DeviceType,
    pub sender: Sender 
}

pub struct User {
    pub username: String,
    pub password: String,
    pub device: Device,
    pub user_id: Uuid,
}


pub struct Payload {
    pub to: Uuid,
    pub from: Uuid,
    pub crossUser: bool,
    pub msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub fromDevice: Uuid,
    pub fromUser: Uuid,
}
