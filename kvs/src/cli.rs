pub mod server {
    use crate::Server;

    pub fn server_main(addr: String) -> Result<(), crate::KvsError>{
        tokio::runtime::Builder::new()
            .enable_all()
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
    pub fn client_main(addr: String) -> Result<(), crate::KvsError>{
        tokio::runtime::Builder::new()
            .enable_all()
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

                let mut stream =  tokio::net::TcpStream::connect(addr).await?;
                tracing::info!("Ok!");
                Ok(())
            })
    }
}
