pub mod server {
    use crate::Server;

    pub fn server_main(addr: String) {
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
                if let Err(err) = Server::new().run(addr).await {
                    tracing::error!("{:?}", err);
                    std::process::exit(1);
                }
            })
    }
}

pub mod client {}
