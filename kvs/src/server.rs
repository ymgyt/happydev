use crate::Result;
use std::fmt;
use tokio::net::{TcpListener, ToSocketAddrs};
use tracing::{error, info};

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {} // clippyにdefaultいわれるかも
    }

    pub async fn run<A: ToSocketAddrs + fmt::Debug>(self, addr: A) -> Result<()> {
        info!(?addr, "Binding...",);
        let mut listener = TcpListener::bind(addr).await?;

        loop {
            match listener.accept().await {
                Ok((_conn, remote)) => {
                    info!("Accept new connection {:?}", remote);
                }
                Err(err) => {
                    error!("{:?}", err);
                }
            }
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
