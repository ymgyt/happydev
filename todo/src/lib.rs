mod domain;
mod handler;

pub mod prelude {
    pub use tracing::{debug, error, info, trace, warn};
}

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
        std::env::var("TODO_LOG").unwrap_or_else(|_| "todo=info".to_owned())
    }

    // アクセスを許可するOrigin(frontのjsをserveしたドメイン)
    pub fn cors_allowed_origins() -> [&'static str; 2] {
        ["localhost", "todo.ymgyt.io"]
    }
}

pub mod state {
    use crate::domain::{entity::task, vo};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    // app state
    pub type SharedState = Arc<State>;

    // in memory tasks
    pub type Tasks = Vec<task::Task>;

    pub struct State {
        pub tasks: RwLock<Tasks>,
    }

    impl State {
        pub fn shared() -> SharedState {
            Arc::new(State::new())
        }

        pub fn new() -> Self {
            let mut tasks = Vec::new();

            // dummyの初期状態を作成する
            for i in 0..5 {
                let t = task::Task::create(task::CreateCommand {
                    title: format!("task {}", i + 1),
                    category: vo::Category::default(),
                    content: "content...".to_string(),
                })
                .expect("Create task");
                tasks.push(t);
            }

            Self {
                tasks: RwLock::new(tasks),
            }
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
        let (method, path) = (req.method().to_owned(), req.uri().path().to_owned());
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
                info!("{} {} {}", method, path, response.status());

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
                error!("{} {} {:?}", method, path, err);
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
                        let tasks = state.tasks.read().await;
                        task_handler.get_tasks(req, &tasks)
                    }
                    Method::POST => {
                        let mut tasks = state.tasks.write().await;
                        task_handler.create_task(req, &mut tasks).await
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
