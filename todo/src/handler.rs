use hyper::{Body, Request, Response, StatusCode};
use crate::{
    state,
    domain::entity,
};
use serde::Serialize;

pub fn not_found() -> Result<Response<Body>, hyper::Error> {
    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}

pub fn healthz() -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("OK")))
}


pub struct TaskHandler {
}


#[derive(Serialize)]
pub struct GetTasksResponse<'a> {
    tasks: &'a Vec<entity::task::Task>,
}

impl TaskHandler {
    pub fn new() -> Self {
        Self{}
    }

    pub fn get_tasks(&self, _req: Request<Body>, tasks: &state::Tasks) -> Result<Response<Body>, hyper::Error> {
       let response = GetTasksResponse{ tasks};

        Ok(Response::new(Body::from(
            serde_json::to_vec(&response).expect("Serialize tasks"))))
    }
}
