use std::{sync::{atomic::{Ordering}}, time::Duration};
use tokio::{sync::mpsc::{channel, Receiver}};

use log::debug;

use crate::{
    error::BoomerError,
    server::{socket::{Listenable, Socket},
    message::{Message, MessageType}},
    kill::{Timeout}
};

pub async fn get_client_data(mut rx: Receiver<Message>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            Message::Message(m) => {
                match m.r#type {
                    MessageType::Ready => println!("READY!"),
                    MessageType::Heartbeat => println!("HEARTBEAT!"),
                    MessageType::Message => println!("{:?}", m.msg),
                    MessageType::Debug => debug!("{:?}", m.msg),
                    MessageType::End => break,
                }
            },
            _ => {},
        }
    }
}

pub async fn handle_client(
    mut s1: Socket,
    mut s2: Socket,
) -> Result<(Socket, Socket), BoomerError> {
    let (tx1, rx1) = channel::<Message>(2);
    let (tx2, rx2) = channel::<Message>(2);

    let id1 = s1.listen(tx1).await;
    let id2 = s2.listen(tx2).await;

    let resp1 = tokio::spawn(get_client_data(rx1));
    let resp2 = tokio::spawn(get_client_data(rx2));

    tokio::select! {
        _ = futures::future::join(resp1, resp2) => { },
    }

    s1.off(id1).await;
    s2.off(id2).await;

    return Ok((s1, s2));
}
