use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::borrow::Cow;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::ops::Deref;
use std::str::FromStr;
use tracing::instrument;
use tracing_subscriber::EnvFilter;

#[instrument(level = "debug")]
async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

async fn run() -> anyhow::Result<()> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_target(true)
        .with_max_level(tracing::Level::TRACE)
        .with_env_filter(EnvFilter::new("happydev_ci=trace"))
        .try_init()
        .map_err(|e| anyhow::anyhow!(e))?;

    let addr: Cow<str> = match std::env::var("CI_ADDR") {
        Ok(addr) => addr.into(),
        Err(_) => "127.0.0.1:3333".into(),
    };
    let addr = SocketAddr::from_str(addr.deref())?;

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_world)) });

    let server = Server::bind(&addr).serve(make_svc);

    tracing::info!(%addr,"running...");
    server.await?;

    Ok(())
}

fn main() {
    tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            if let Err(err) = run().await {
                tracing::error!("{}", err);
            }
        });
}
