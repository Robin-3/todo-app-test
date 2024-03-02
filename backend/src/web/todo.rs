use super::{filter_auth::do_auth, filter_utils::with_db};
use crate::{
    model::{Db, TodoMac, TodoPatch},
    security::UserCtx,
};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use warp::{reply::Json, Filter};

pub fn todo_rest_filters(
    base_path: &'static str,
    db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let todos_path = warp::path(base_path).and(warp::path("todos"));
    let common = with_db(db.clone()).and(do_auth(db.clone()));

    // LIST todos `GET /todos`
    let list = todos_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(todo_list);

    // GET todo `GET /todos/[id_todo]`
    let get = todos_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(todo_get);

    // CREATE todo `POST /todos`
    let create = todos_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(todo_create);

    // UPDATE todo `PATCH /todos/[id_todo] with body TodoPatch`
    let update = todos_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(todo_update);

    // DELETE todo `DELETE /todos/[id_todo]`
    let delete = todos_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(todo_delete);

    list.or(get).or(create).or(update).or(delete)
}

async fn todo_list(db: Arc<Db>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    let todos = TodoMac::list(&db, &utx).await?;
    json_response(todos)
}

async fn todo_get(db: Arc<Db>, utx: UserCtx, id_todo: i64) -> Result<Json, warp::Rejection> {
    let todos = TodoMac::get(&db, &utx, id_todo).await?;
    json_response(todos)
}

async fn todo_create(db: Arc<Db>, utx: UserCtx, patch: TodoPatch) -> Result<Json, warp::Rejection> {
    let todos = TodoMac::create(&db, &utx, patch).await?;
    json_response(todos)
}

async fn todo_update(
    db: Arc<Db>,
    utx: UserCtx,
    id_todo: i64,
    patch: TodoPatch,
) -> Result<Json, warp::Rejection> {
    let todos = TodoMac::update(&db, &utx, id_todo, patch).await?;
    json_response(todos)
}

async fn todo_delete(db: Arc<Db>, utx: UserCtx, id_todo: i64) -> Result<Json, warp::Rejection> {
    let todos = TodoMac::delete(&db, &utx, id_todo).await?;
    json_response(todos)
}

fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({"data": data });
    Ok(warp::reply::json(&response))
}

#[cfg(test)]
#[path = "../_tests/web_todo.rs"]
mod tests;
