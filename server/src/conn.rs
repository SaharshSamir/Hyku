use crate::models::Payload;
use futures::{future, Future, SinkExt};
use futures_util::{stream::TryStreamExt, StreamExt};
use serde;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::io::Error as IoError;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite;
use tokio_tungstenite::tungstenite::Message;

pub type Sender = mpsc::UnboundedSender<Payload>;
pub type Receiver = mpsc::UnboundedReceiver<Payload>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Sender>>>;

#[allow(non_snake_case)]
pub async fn handle_connection(
    peerMap: PeerMap,
    tcp_stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), IoError> {
    println!("Connection coming from {addr}");

    let ws_stream = tokio_tungstenite::accept_async::<TcpStream>(tcp_stream)
        .await
        .expect("tunstenite pooped itself");
    let (mut outgoing, incoming) = ws_stream.split();

    let (tx, rx) = mpsc::unbounded_channel();
    let tx: Sender = tx;
    let mut rx: Receiver = rx;
    peerMap.lock().unwrap().insert(addr, tx);

    //let broadcast_from_others = rx.map(Ok).forward(outgoing);

    //1.
    tokio::select! {
        _ = incoming.try_for_each(|msg| {
            println!("{:?}", msg);
            future::ok(())
        }) => {
            println!("incoming stream closed");
        }
        Some(msg) = rx.recv() => {
            println!("{:?}", msg);
            // outgoing.send(msg).await.unwrap();
            outgoing.send(Message::Text(format!("{:?}", msg))).await.unwrap();
        }
    }
    //let broadcast_from_others = future::select(broadcast_incoming, broadcast_incoming).await;
    Ok(())
}
