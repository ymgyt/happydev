pub mod server {
    use crate::Server;

    pub fn server_main(addr: String) -> Result<(), crate::KvsError> {
        tokio::runtime::Builder::new()
            .enable_all()
            .threaded_scheduler()
            .build()
            .unwrap()
            .block_on(async {
                tracing_subscriber::FmtSubscriber::builder()
                    .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc3339())
                    .with_target(true)
                    .with_env_filter(
                        std::env::var("KVS_LOG").unwrap_or_else(|_| "kvs=info".to_owned()),
                    )
                    .init();

                // TODO handle signal
                Server::new().run(addr).await
            })
    }
}

pub mod client {
    use crate::protocol::message;

    pub fn client_main(addr: String) -> Result<(), crate::KvsError> {
        tokio::runtime::Builder::new()
            .enable_all()
            .threaded_scheduler()
            .build()
            .unwrap()
            .block_on(async {
                tracing_subscriber::FmtSubscriber::builder()
                    .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc3339())
                    .with_target(true)
                    .with_env_filter(
                        std::env::var("KVS_LOG").unwrap_or_else(|_| "kvs=info".to_owned()),
                    )
                    .init();

                let stream = tokio::net::TcpStream::connect(&addr).await?;
                tracing::info!(?addr, "Successfully connected");

                let mut operator = message::Operator::with_stream(stream)?;
                let echo_request = message::Message::from_payload(message::Payload::EchoRequest {
                    message: "Hello kvs!".to_owned(),
                })?;
                operator.send(echo_request).await?;

                if let message::Payload::EchoResponse { message, .. } = operator.receive().await? {
                    tracing::info!("Got response from server {}", message);
                }
                Ok(())
            })
    }
}
