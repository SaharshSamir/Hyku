use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Filter, Rejection, Reply};

mod controllers;
mod models;
mod routes;
mod ws;

pub type Sender = Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>;
pub type Peers = Arc<Mutex<HashMap<String, models::Device>>>;
pub type Result<T> = std::result::Result<T, Rejection>;

//1, Handle connections: Handle the case where a client connects to our server. This is where we
//add the peer to our peer map. If the client already exists in our peer map, means they are
//currently connected to our server as a websocket

pub async fn home_handler() -> Result<impl Reply> {
    Ok(String::from("Hey sup"))
}

#[tokio::main]
async fn main() {
    let mut peers: Peers = Arc::new(Mutex::new(HashMap::new()));
    //handling ws connection
    let ws_routes = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_peers(peers.clone()))
        .and_then(controllers::handle_ws);

    //this is where a user gets created / authenticated
    let register_routes = warp::path!("register")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_peers(peers.clone()))
        .and_then(controllers::handle_registeration);

    let home = warp::path!("home").and(warp::get()).and_then(home_handler);
    let routes = register_routes
        .or(ws_routes)
        .or(home)
        .with(warp::cors().allow_any_origin());
    warp::serve(routes).run(([0, 0, 0, 0], 6969)).await;
}

fn with_peers(peers: Peers) -> impl Filter<Extract = (Peers,), Error = Infallible> + Clone {
    warp::any().map(move || peers.clone())
}
