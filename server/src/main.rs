//1. create a tcplistener which return something
//2. loop over the listener.accept() which gives a stream, corresponding to each client
//3. pass the stream to tokio_tungstenite which gives a stream
//4. split the stream into outgoing and incoming

//(broadcast_listener, broadcast_listener) from mpsc channel
//

#[allow(unused_imports)]
use crate::{
    conn::{handle_connection, PeerMap},
    models::User,
};
#[allow(unused_imports)]
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
#[allow(unused_imports)]
use tokio::net::{TcpListener, TcpStream};

pub mod conn;
pub mod models;

#[tokio::main]
async fn main() {
    //get this value from env
    const SERVER_ADDR: &str = "0.0.0.0:6969";
    //create a TcpListener which gives us the stream, and the socket address of the client that has connected
    let try_listener = TcpListener::bind(SERVER_ADDR).await;
    let listener = try_listener.expect("binding problem");

    //create a hashmap that maps one client to a sender (sender which we get from an mpsc channel)
    let clients: PeerMap = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, client_addr)) = listener.accept().await {
        //make a scocket server in in handle_connection function
        tokio::spawn(handle_connection(clients.clone(), stream, client_addr));
    }
}
