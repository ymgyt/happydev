mod domain;
mod handler;

pub mod prelude {
    pub use tracing::{debug, error, info, trace, warn};
}

pub mod config {
    use std::{env, path};
    // http serverのport番号
    pub fn port() -> u16 {
        env::var("TODO_PORT")
            .expect("TODO_PORT required")
            .parse::<u16>()
            .unwrap()
    }

    // loggingのfilter directive
    pub fn log_filter() -> String {
        env::var("TODO_LOG").unwrap_or_else(|_| "todo=info".to_owned())
    }

    // アクセスを許可するOrigin(frontのjsをserveしたドメイン)
    pub fn cors_allowed_origins() -> [&'static str; 2] {
        ["localhost", "todo.ymgyt.io"]
    }

    // kvsの格納file
    pub fn kvs_file_path() -> path::PathBuf {
        use std::str::FromStr;
        env::var("TODO_KVS")
            .map(path::PathBuf::from)
            .or(path::PathBuf::from_str("./todo.kvs"))
            .expect("Get kvs file path")
    }
}

// applicationのstate
// 基本的にはexternal serviceのconnectionとかを保持する想定
// 今はstorage層を組み込んでないので、in memoryに全部もっている
pub mod state {
    use crate::config;
    use kvs::Kvs;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    // app state
    #[derive(Default)]
    pub struct State {
        // tokio::sync::RwLockがDefaultの実装を要求するのでOptionでWrapしている
        pub kvs: RwLock<Option<Kvs>>,
    }

    pub type SharedState = Arc<State>;

    impl State {
        pub fn shared() -> Result<SharedState, anyhow::Error> {
            Ok(Arc::new(State::new()?))
        }

        fn new() -> Result<Self, anyhow::Error> {
            Ok(Self {
                kvs: RwLock::new(Some(State::kvs()?)),
            })
        }

        fn kvs() -> Result<Kvs, anyhow::Error> {
            Kvs::new(config::kvs_file_path().as_path()).map_err(anyhow::Error::from)
        }
    }
}

pub mod router {
    use crate::{config, handler, prelude::*, state::SharedState};
    use http::status::StatusCode;
    use hyper::header;
    use hyper::{Body, Method, Request, Response};

    // TODO: hyper::serviceを実装してmiddlewareに切り出す
    pub async fn service(
        state: SharedState,
        req: Request<Body>,
    ) -> Result<Response<Body>, anyhow::Error> {
        // TODO: closureでconnもらって、remote_addrもだす
        // requestはrouterにmoveするので、copyしておかないといけない
        let (method, path, query) = (
            req.method().to_owned(),
            req.uri().path().to_owned(),
            req.uri().query().map(String::from),
        );
        trace!("{:?}", req.headers());

        // CORS
        // Http Request Header Originにtodo frontendをserveしたドメインがはいっているか確認する
        // Response HeaderACCESS_CONTROL_ALLOW系の値をいれないとbrowserのfetch APIがエラーを発生させる
        let mut allowed_origin: Option<header::HeaderValue> = None;
        if let Some(origin_value) = req.headers().get(header::ORIGIN) {
            if let Ok(origin) = url::Url::parse(origin_value.to_str().unwrap()) {
                if let Some(host) = origin.host_str() {
                    config::cors_allowed_origins().iter().for_each(|allowed| {
                        if host.starts_with(allowed) {
                            allowed_origin = Some(origin_value.to_owned());
                        }
                    })
                }
            };
        }

        match router(state, req).await {
            Ok(mut response) => {
                info!(
                    "{} {} {} {}",
                    response.status().as_u16(),
                    method,
                    path,
                    query.unwrap_or_default()
                );

                // 帰りのmiddleware処理。ここもmiddlewareにきりだす
                // CORS関連のheader処理
                if let Some(origin) = allowed_origin {
                    insert_cors_headers(origin, response.headers_mut());
                }

                // Content-Type設定
                response.headers_mut().insert(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                );
                Ok(response)
            }
            Err(err) => {
                error!("{} {}{} {:?}", method, path, query.unwrap_or_default(), err);
                Err(err)
            }
        }
    }

    // request entry point
    pub async fn router(
        state: SharedState,
        req: Request<Body>,
    ) -> Result<Response<Body>, anyhow::Error> {
        let (method, path) = (req.method(), req.uri().path());
        // CORSのpreflight時にはpathにPOST等のリクエストしたいPATHが指定されているので先に処理する
        if method == Method::OPTIONS {
            return Ok(Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body(Body::empty())
                .unwrap());
        }

        match path {
            _tasks if path.starts_with("/tasks") => {
                let task_handler = handler::TaskHandler::new();
                match *method {
                    Method::GET => {
                        let mut kvs = state.kvs.write().await;
                        task_handler.get_tasks(req, &mut kvs.as_mut().unwrap())
                    }
                    Method::POST => {
                        let mut kvs = state.kvs.write().await;
                        task_handler
                            .create_task(req, &mut kvs.as_mut().unwrap())
                            .await
                    }
                    Method::DELETE => {
                        let mut kvs = state.kvs.write().await;
                        task_handler.delete_task(req, &mut kvs.as_mut().unwrap())
                    }
                    _ => handler::not_found(),
                }
            }
            "/healthz" => handler::healthz(),
            _ => handler::not_found(),
        }
    }

    fn insert_cors_headers(origin: header::HeaderValue, headers: &mut header::HeaderMap) {
        // ACCESS_CONTROL_ALLOW_CREDENTIALS: trueを利用する場合、wildcard指定は利用できない
        // https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            header::HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
        );
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            header::HeaderValue::from_static("Content-Type"),
        );
        headers.insert(
            header::ACCESS_CONTROL_MAX_AGE,
            header::HeaderValue::from_static("10"), // CORSの挙動の勉強になるので短めに設定しておく
        );
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            header::HeaderValue::from_static("false"), // credential扱うようになったら許可する
        );
    }
}
