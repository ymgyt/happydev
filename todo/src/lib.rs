mod handler;

pub mod config {
    // http serverのport番号
    pub fn port() -> u16 {
        std::env::var("TODO_PORT")
            .expect("TODO_PORT required")
            .parse::<u16>()
            .unwrap()
    }

    // loggingのfilter directive
    pub fn log_filter() -> String {
        std::env::var("TODO_LOG").unwrap_or_else(|_|"todo=info".to_owned())
    }

    // アクセスを許可するOrigin(frontのjsをserveしたドメイン)
    pub fn cors_allowed_origins() -> [&'static str; 2] {
        ["localhost", "todo.ymgyt.io"]
    }
}

pub mod prelude {
    pub use tracing::{debug, error, info, trace, warn};
}

pub mod router {
    use crate::{config, handler, prelude::*};
    use hyper::header;
    use hyper::{Body, Method, Request, Response};

    // TODO: serviceを実装してmiddlewareに切り出す
    pub async fn service(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        // TODO: closureでconnもらって、remote_addrもだす
        // requestはrouterにmoveするので、copyしておかないといけない
        let (method, path) = (req.method().to_owned(), req.uri().path().to_owned());

        // CORS
        // Http Request Header Originにtodo frontendをserveしたドメインがはいっているか確認する
        // Response HeaderACCESS_CONTROL_ALLOW系の値をいれないとbrowserのfetch APIがエラーを発生させる
        let mut allow_cors_access: bool = false;
        if let Some(origin) = req.headers().get(header::ORIGIN) {
            if let Ok(origin) = url::Url::parse(origin.to_str().unwrap()) {
                trace!("origin {:?}", origin.host_str());
                if let Some(host) = origin.host_str() {
                    config::cors_allowed_origins().iter().for_each(|allowed| {
                        if host.starts_with(allowed) {
                            allow_cors_access = true;
                        }
                    })
                }
            };
        }

        match router(req).await {
            Ok(mut response) => {
                info!("{} {} {}", method, path, response.status());

                // 帰りのmiddleware処理。ここもmiddlewareにきりだす
                // 許可されたfront(js)の場合、Response Headerにその旨明示する。
                if allow_cors_access {
                    response.headers_mut().insert(
                        header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        header::HeaderValue::from_static("*"),
                    );
                }

                // Content-Type設定
                response.headers_mut().insert(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                );
                Ok(response)
            }
            Err(err) => {
                error!("{} {} {:?}", method, path, err);
                Err(err)
            }
        }
    }

    // request entry point
    pub async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let (method, path) = (req.method(), req.uri().path());
        match path {
            _tasks if path.starts_with("/tasks") => {
                let task_handler = handler::TaskHandler::new();
                match *method {
                    Method::GET => task_handler.get_tasks(req),
                    _ => handler::not_found(),
                }
            }
            _healthz if path.starts_with("/healthz") => {
               handler::healthz()
            }
            _ => handler::not_found(),
        }
    }
}
