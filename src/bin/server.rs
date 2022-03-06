#![feature(vec_retain_mut)]

use log::warn;
use cafe::server::server::Server;
use cafe::handler::go_do_the_thing;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let mut server = Server::new().await?;
    warn!("starting server");
    let receiver = server.get_receiver();

    tokio::spawn(async move {
        let mut receiver = receiver.unwrap();
        while let Some(two_sockets) = receiver.recv().await {
            tokio::spawn(go_do_the_thing(two_sockets));
        }
    });

    server.join_handle.await?;

    return Ok(());
}

