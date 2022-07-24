use crate::{
    models::Event,
    models::{AppError, Device, DeviceType},
    ws::something,
    Peers, Result, Sender,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::{uuid, Uuid};
use warp::{ws::WebSocket, Error};
use warp::{
    ws::{Message, Ws},
    Filter, Rejection, Reply,
};

#[derive(Deserialize, Serialize)]
pub struct RegistrationBody {
    device_id: String,
    name: String,
    device_type: String,
}

//create a new device and add it to our peer list
pub async fn handle_registeration(body: RegistrationBody, peers: Peers) -> Result<impl Reply> {
    let peers_maybe = peers.lock().await;
    let device_maybe = peers_maybe.get(&body.device_id.to_string());
    let id = Uuid::new_v4().to_string();

    let res = match device_maybe {
        Some(d) => Err(warp::reject::custom(AppError {
            reason: String::from("Device already exists"),
        })),
        None => {
            let newDevice = Device {
                name: body.name,
                device_id: Uuid::parse_str(&id).unwrap(),
                device_type: DeviceType::from_str(&body.device_type).unwrap(),
                sender: None,
            };
            peers.lock().await.insert(id, newDevice);
            Ok(())
        }
    };
    println!("device added");
    Ok(String::from("device added"))
}

pub async fn handle_ws(ws: Ws, peer_id: String, peers: Peers) -> Result<impl Reply> {
    let device = peers.lock().await.get(&peer_id).cloned();

    match device {
        Some(d) => Ok(ws.on_upgrade(move |socket| something(socket, peer_id, peers, d))),
        None => Err(warp::reject::not_found()),
    }
}
