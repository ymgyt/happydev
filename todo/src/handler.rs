use hyper::{Body, Request, Response, StatusCode};

pub fn not_found() -> Result<Response<Body>, hyper::Error> {
    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}

pub fn healthz() -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("OK")))
}

pub struct TaskHandler {}

impl TaskHandler {
    pub fn new() -> Self {
        TaskHandler {}
    }

    pub fn get_tasks(&self, _req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        Ok(Response::new(Body::from(
            r#"{"tasks":[
            {"id": "A1", "title": "task 1", "category": "/xxx/yyy", "content": "content ..."},
            {"id": "A2", "title": "task 2", "category": "/xxx/yyy", "content": "content ..."},
            {"id": "A3", "title": "task 3", "category": "/xxx/yyy", "content": "content ..."},
            {"id": "A4", "title": "task 4", "category": "/xxx/yyy", "content": "content ..."}
        ]}"#,
        )))
    }
}
