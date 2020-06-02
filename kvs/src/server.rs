use crate::{
    protocol::message::{self, Operator},
    Result,
};
use std::{fmt, net::SocketAddr};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
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
                Ok((conn, remote)) => {
                    info!(?remote, "Accept new connection");
                    let worker = Worker::new(conn, remote)?;
                    tokio::task::spawn(async move {
                        if let Err(err) = worker.dispatch().await {
                            error!("{}", err);
                        };
                    });
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

struct Worker {
    remote: SocketAddr,
    operator: Operator,
}

impl Worker {
    fn new(stream: TcpStream, remote: SocketAddr) -> Result<Self> {
        Ok(Self {
            remote,
            operator: Operator::with_stream(stream)?,
        })
    }
    async fn dispatch(mut self) -> Result<()> {
        info!(remote=?self.remote, "Worker dispatched");

        let payload = self.operator.receive().await?;
        info!("Receive! {:?}", payload);
        if let message::Payload::EchoRequest { message, .. } = payload {
            self.operator
                .send(message::Message::from_payload(
                    message::Payload::EchoResponse { message },
                )?)
                .await?;
        }

        Ok(())
    }
}
