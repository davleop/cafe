use std::{sync::{atomic::{Ordering}}, time::Duration};
use tokio::{sync::mpsc::{channel, Sender, Receiver}};

use log::{error, debug};

use crate::{
    error::BoomerError,
    server::{socket::{Listenable, Socket},
    message::{Message, MessageType}},
    kill::{Timeout}
};

pub async fn handle_client_data(
    tx: Sender<Message>,
    mut rx: Receiver<Message>,
) {
    while let Some(msg) = rx.recv().await {
        match msg {
            Message::Message(m) => {
                let resp: Option<Message>;
                match m.r#type {
                    MessageType::Ready => {
                        resp = Some(Message::with_message(
                            MessageType::Ready,
                            "READY!".to_string(),
                        ));
                        println!("READY!");
                    },
                    MessageType::Heartbeat => {
                        resp = Some(Message::with_message(
                            MessageType::Ready,
                            "HEARTBEAT!".to_string(),
                        ));
                        println!("HEARTBEAT!");
                    },
                    MessageType::Message => {
                        resp = Some(Message::with_message(
                            MessageType::Ready,
                            m.msg.clone().unwrap(),
                        ));
                        println!("{:?}", m.msg);
                    },
                    MessageType::Debug => {
                        resp = Some(Message::with_message(
                            MessageType::Ready,
                            m.msg.clone().unwrap(),
                        ));
                        debug!("{:?}", m.msg);
                    },
                    MessageType::End => {
                        break;
                    },
                    _ => {
                        resp = None;
                        error!("unknown protocol")
                    },
                }
                tx.send(resp.unwrap()).await.expect("to be okay");
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

    let id1 = s1.listen(tx1.clone()).await;
    let id2 = s2.listen(tx2.clone()).await;

    let h1 = tokio::spawn(handle_client_data(tx1, rx1));
    let h2 = tokio::spawn(handle_client_data(tx2, rx2));

    tokio::select! {
        _ = futures::future::join(h1, h2) => { },
    }

    s1.off(id1).await;
    s2.off(id2).await;

    return Ok((s1,s2));
}
