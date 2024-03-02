use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::{fs, path::PathBuf};

const SQLITE_URL: &str = "sqlite://app.db";
// app db
const SQLITE_APP_MAX_CON: u32 = 5;
// sql files
const SQL_DIR: &str = "sql/";
const SQL_RECREATE: &str = "sql/00-recreate-db.sql";

pub type Db = Pool<Sqlite>;

pub async fn init_db() -> Result<Db, sqlx::Error> {
    // -- Create the db (dev only)
    {
        create_db(SQLITE_URL).await?;
        let root_db = new_db_pool(SQLITE_URL, 1).await?;
        pexec(&root_db, SQL_RECREATE).await?;
    }

    // -- Run the app sql file
    let app_db = new_db_pool(SQLITE_URL, SQLITE_APP_MAX_CON).await?;
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    paths.sort();
    // execute each file
    for path in paths {
        if let Some(path) = path.to_str() {
            // only .sql and not the recreate
            if path.ends_with(".sql") && path != SQL_RECREATE {
                pexec(&app_db, path).await?;
            }
        }
    }

    // returning the app db
    // create_db(SQLITE_URL).await?;
    // new_db_pool(SQLITE_URL, SQLITE_APP_MAX_CON).await
    Ok(app_db)
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    // Read the file
    let content = fs::read_to_string(file).map_err(|ex| {
        println!("ERROR reading {} (cause: {:?})", file, ex);
        ex
    })?;

    // TODO: Make the split more sql proof
    let sqls: Vec<&str> = content.split(';').collect();

    for sql in sqls {
        match sqlx::query(sql).execute(db).await {
            Ok(_) => (),
            Err(ex) => println!("WARNING - pexec - Sql file '{}' FAILED cause: {}", file, ex),
        }
    }

    Ok(())
}

async fn create_db(url: &str) -> Result<(), sqlx::Error> {
    if !Sqlite::database_exists(url).await.unwrap_or(false) {
        Sqlite::create_database(url).await?
    }
    Ok(())
}

async fn new_db_pool(url: &str, max_con: u32) -> Result<Db, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(max_con)
        .connect(url)
        .await
}

#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;
