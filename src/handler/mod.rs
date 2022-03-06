use crate::{error::BoomerError, server::socket::Socket};

pub mod wait;

pub async fn go_do_the_thing(
    (s1, s2): (Socket, Socket)
) -> Result<(), BoomerError> {
    let (mut s1, mut s2) = wait::wait_for_client(s1, s2).await?;

    let res1 = s1.close().await;
    let res2 = s2.close().await;

    res1?;
    res2?;

    return Ok(());
}
