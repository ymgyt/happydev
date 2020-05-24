use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::net::SocketAddr;
use todo::{config, router};
use tracing::{error, info};

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

fn init_logger() {
    tracing_subscriber::FmtSubscriber::builder()
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc3339())
        .with_target(true)
        .with_env_filter(config::log_filter())
        .init();
}

#[tokio::main]
async fn main() {
    init_logger();

    let addr = SocketAddr::from(([0, 0, 0, 0], config::port()));

    let server = Server::bind(&addr)
        .serve(make_service_fn(|_| async {
            Ok::<_, std::convert::Infallible>(service_fn(router::service))
        }))
        .with_graceful_shutdown(shutdown_signal());

    info!(%addr, "Listening...");

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}
