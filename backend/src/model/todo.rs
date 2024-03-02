use super::db::Db;
use crate::{model, security::UserCtx};
use serde::{Deserialize, Serialize};

const TABLE_NAME: &str = "todo";

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id_todo: i64,
    pub id_creator: i64,
    pub title: String,
    pub status: TodoStatus,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct TodoPatch {
    pub title: Option<String>,
    pub status: Option<TodoStatus>,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
    Open,
    Close,
}

impl TodoStatus {
    fn get_id_status(&self) -> i64 {
        match self {
            TodoStatus::Open => 1,
            TodoStatus::Close => 2,
        }
    }
}

// Todo Model Access Controller
pub struct TodoMac;

impl TodoMac {
    pub async fn create(db: &Db, utx: &UserCtx, data: TodoPatch) -> Result<Todo, model::Error> {
        let sql = "INSERT INTO todo (id_creator, title) VALUES ($1, $2) RETURNING id_todo, id_creator, title, (SELECT todo_status_enum.status FROM todo_status_enum WHERE todo.id_status = todo_status_enum.id_status) as status";
        let query = sqlx::query_as::<_, Todo>(sql)
            .bind(utx.user_id)
            .bind(data.title.unwrap_or_else(|| "untitled".to_string()));

        let mut tx = db.begin().await?;
        let todo = query.fetch_one(&mut *tx).await?;
        tx.commit().await?;

        Ok(todo)
    }

    pub async fn get(db: &Db, _utx: &UserCtx, id_todo: i64) -> Result<Todo, model::Error> {
        let sql = "SELECT todo.id_todo, todo.id_creator, todo.title, todo_status_enum.status FROM todo, todo_status_enum WHERE todo.id_status = todo_status_enum.id_status AND todo.id_todo = $1";
        let query = sqlx::query_as::<_, Todo>(sql).bind(id_todo);

        let mut tx = db.begin().await?;
        let result = query.fetch_one(&mut *tx).await;
        tx.commit().await?;

        handle_fetch_one_result(result, TABLE_NAME, id_todo)
    }

    pub async fn update(
        db: &Db,
        utx: &UserCtx,
        id_todo: i64,
        data: TodoPatch,
    ) -> Result<Todo, model::Error> {
        let sql;
        let query;
        match data.status {
            Some(data_status) => {
                sql = "UPDATE todo SET title = $1, id_status = $2, id_modified = $3, time_modified = CURRENT_TIMESTAMP WHERE id_todo = $4 RETURNING id_todo, id_creator, title, (SELECT todo_status_enum.status FROM todo_status_enum WHERE todo.id_status = todo_status_enum.id_status) as status";
                query = sqlx::query_as::<_, Todo>(sql)
                    .bind(data.title.unwrap_or_else(|| "untitled".to_string()))
                    .bind(data_status.get_id_status())
                    .bind(utx.user_id)
                    .bind(id_todo);
            }
            None => {
                sql = "UPDATE todo SET title = $1, id_modified = $2, time_modified = CURRENT_TIMESTAMP WHERE id_todo = $3 RETURNING id_todo, id_creator, title, (SELECT todo_status_enum.status FROM todo_status_enum WHERE todo.id_status = todo_status_enum.id_status) as status";
                query = sqlx::query_as::<_, Todo>(sql)
                    .bind(data.title.unwrap_or_else(|| "untitled".to_string()))
                    .bind(utx.user_id)
                    .bind(id_todo);
            }
        };

        let mut tx = db.begin().await?;
        let result = query.fetch_one(&mut *tx).await;
        tx.commit().await?;

        handle_fetch_one_result(result, TABLE_NAME, id_todo)
    }

    pub async fn list(db: &Db, _utx: &UserCtx) -> Result<Vec<Todo>, model::Error> {
        let sql = "SELECT todo.id_todo, todo.id_creator, todo.title, todo_status_enum.status FROM todo, todo_status_enum WHERE todo.id_status = todo_status_enum.id_status ORDER BY todo.id_todo DESC";

        let query = sqlx::query_as::<_, Todo>(sql);

        let mut tx = db.begin().await?;
        let todos = query.fetch_all(&mut *tx).await?;
        tx.commit().await?;

        Ok(todos)
    }

    pub async fn delete(db: &Db, _utx: &UserCtx, id_todo: i64) -> Result<Todo, model::Error> {
        let sql = "DELETE FROM todo WHERE id_todo = $1 RETURNING id_todo, id_creator, title, (SELECT todo_status_enum.status FROM todo_status_enum WHERE todo.id_status = todo_status_enum.id_status) as status";

        let query = sqlx::query_as::<_, Todo>(sql).bind(id_todo);

        let mut tx = db.begin().await?;
        let result = query.fetch_one(&mut *tx).await;
        tx.commit().await?;

        handle_fetch_one_result(result, TABLE_NAME, id_todo)
    }
}

fn handle_fetch_one_result(
    result: Result<Todo, sqlx::Error>,
    typ: &'static str,
    id_todo: i64,
) -> Result<Todo, model::Error> {
    result.map_err(|sqlx_error| match sqlx_error {
        sqlx::Error::RowNotFound => model::Error::EntityNotFound(typ, id_todo.to_string()),
        other => model::Error::SqlxError(other),
    })
}

#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
