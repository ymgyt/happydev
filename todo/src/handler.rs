use crate::{domain::entity, prelude::*};
use hyper::body::Buf;
use hyper::{Body, Request, Response, StatusCode};
use kvs::Kvs;
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
pub struct GetTasksResponse {
    tasks: Vec<entity::task::Task>,
}

impl TaskHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_tasks(
        &self,
        req: Request<Body>,
        kvs: &mut Kvs,
    ) -> Result<Response<Body>, anyhow::Error> {
        let mut tasks: Vec<Task> = kvs
            .iter::<Task>()
            // kvsにはtask以外も格納されている可能性があるので、serialize error(=他のdata)と判断して無視する
            // ただし、その他のエラーは握りつぶさないようにする
            .filter(|r| r.is_ok() || !r.as_ref().unwrap_err().is_serialize())
            .collect::<Result<Vec<Task>, _>>()?;

        // filter
        let query: HashMap<Cow<str>, Cow<str>> = req
            .uri()
            .query()
            .map(|q| url::form_urlencoded::parse(q.as_bytes()).collect())
            .unwrap_or_default();
        if let Some(query) = query.get("query") {
            let query = query.to_ascii_lowercase();
            tasks.retain(|task| task.title().to_ascii_lowercase().contains(query.as_str()));
        }

        serde_json::to_vec(&GetTasksResponse { tasks })
            .map_err(anyhow::Error::from)
            .map(|json| Response::new(Body::from(json)))
    }

    // taskの作成
    pub async fn create_task(
        &self,
        req: Request<Body>,
        kvs: &mut Kvs,
    ) -> Result<Response<Body>, anyhow::Error> {
        // TODO: read body then acquire lock
        let create_cmd = serde_json::from_slice::<entity::task::CreateCommand>(
            hyper::body::to_bytes(req.into_body()).await?.bytes(),
        )?;
        // TODO: map err to bad request(404)
        let task = entity::task::Task::create(create_cmd)?;
        info!(?task, "Create new task");

        kvs.put(task.id().to_string(), &task)?;

        serde_json::to_vec(&task)
            .map(|serialized| Response::new(Body::from(serialized)))
            .map_err(anyhow::Error::from)
    }

    // taskの削除
    pub fn delete_task(
        &self,
        req: Request<Body>,
        kvs: &mut Kvs,
    ) -> Result<Response<Body>, anyhow::Error> {
        // /tasks/{uuid} というpathを想定
        req.uri()
            .path()
            .split('/')
            .nth(2)
            .ok_or_else(|| anyhow::anyhow!("task id not found in path"))
            .and_then(|delete_id| {
                info!("Delete task: {:?}", delete_id);
                kvs.delete::<entity::task::Task>(delete_id)
                    .map_err(anyhow::Error::from)
            })
            .and_then(|opt: Option<entity::task::Task>| match opt {
                Some(task) => serde_json::to_vec(&task)
                    .map(|serialized| Response::new(Body::from(serialized)))
                    .map_err(anyhow::Error::from),
                None => Response::builder()
                    .status(StatusCode::NO_CONTENT)
                    .body(Body::empty())
                    .map_err(anyhow::Error::from),
            })
    }
}
