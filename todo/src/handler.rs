use crate::{
    domain::{entity, vo},
    prelude::*,
    state,
};
use hyper::body::Buf;
use hyper::{Body, Request, Response, StatusCode};
use serde::Serialize;
use std::{borrow::Cow, collections::HashMap};

pub fn not_found() -> Result<Response<Body>, anyhow::Error> {
    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}

pub fn healthz() -> Result<Response<Body>, anyhow::Error> {
    Ok(Response::new(Body::from("OK")))
}

pub struct TaskHandler {}

#[derive(Serialize)]
pub struct GetTasksResponse<'a, 'b> {
    tasks: &'a [&'b entity::task::Task],
}

impl TaskHandler {
    pub fn new() -> Self {
        Self {}
    }

    // taskの取得
    pub fn get_tasks(
        &self,
        req: Request<Body>,
        tasks: &[entity::task::Task],
    ) -> Result<Response<Body>, anyhow::Error> {
        let query: HashMap<Cow<str>, Cow<str>> = req
            .uri()
            .query()
            .map(|q| url::form_urlencoded::parse(q.as_bytes()).collect())
            .unwrap_or_default();

        // queryによる検索
        let tasks: Vec<&entity::task::Task> = if let Some(query) = query.get("query") {
            // とりあえずtitle決め打ちの検索をおこなう
            let query = query.to_ascii_lowercase();
            tasks
                .iter()
                .filter(|&task| task.title().to_ascii_lowercase().contains(query.as_str()))
                .collect()
        } else {
            tasks.iter().map(|task| task).collect()
        };

        serde_json::to_vec(&GetTasksResponse {
            tasks: tasks.as_slice(),
        })
        .map_err(anyhow::Error::from)
        .and_then(|v| Ok(Response::new(Body::from(v))))
    }

    // taskの作成
    pub async fn create_task(
        &self,
        req: Request<Body>,
        tasks: &mut state::Tasks,
    ) -> Result<Response<Body>, anyhow::Error> {
        // TODO: read body then acquire lock
        let create_cmd = serde_json::from_slice::<entity::task::CreateCommand>(
            hyper::body::to_bytes(req.into_body()).await?.bytes(),
        )?;
        // TODO: map err to bad request(404)
        let task = entity::task::Task::create(create_cmd)?;
        info!(?task, "Create new task");

        serde_json::to_vec(&task)
            .and_then(|serialized| {
                tasks.push(task);
                Ok(Response::new(Body::from(serialized)))
            })
            .map_err(anyhow::Error::from)
    }

    // taskの削除
    pub fn delete_task(
        &self,
        req: Request<Body>,
        tasks: &mut state::Tasks,
    ) -> Result<Response<Body>, anyhow::Error> {
        // tasks/{uuid} というpathを想定
        req.uri()
            .path()
            .split('/')
            .nth(2)
            .ok_or_else(|| anyhow::anyhow!("task id not found in path"))
            .and_then(|task_id: &str| task_id.parse::<vo::TaskId>())
            .map(|delete_id| {
                info!("Delete task: {:?}", delete_id);
                tasks.retain(|task| task.id().ne(&delete_id))
            })
            .and_then(|_| {
                Response::builder()
                    .status(StatusCode::NO_CONTENT)
                    .body(Body::empty())
                    .map_err(anyhow::Error::from)
            })
    }
}
