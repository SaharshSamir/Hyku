use crate::{models::Device, models::Event, ws, Peers, Result};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{ws::WebSocket, Error};

pub async fn something(socket: WebSocket, peer_id: String, peers: Peers, mut client: Device) {
    let (channel_sendr, channel_rcv) = mpsc::unbounded_channel();
    let (ws_sendr, mut ws_recv) = socket.split();

    let channel_rcv = UnboundedReceiverStream::new(channel_rcv);
    //any messages received by the channel, forward it to the outgoing socket, to the client
    tokio::task::spawn(channel_rcv.forward(ws_sendr).map(|result| {
        if let Err(e) = result {
            println!("could not forward stream, {}", e);
        }
    }));

    //give the client a channel sender to send messages to the output stream
    client.sender = Some(channel_sendr);
    peers.lock().await.insert(peer_id, client);

    
    while let msg = ws_recv.next().await {
        println!("{:?}", msg);
    }

    println!("Do something");
}
