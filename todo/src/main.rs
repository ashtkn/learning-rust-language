use actix_web::{web, get, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use thiserror::Error;

use r2d2::{Pool};
use r2d2_sqlite::{SqliteConnectionManager};
use rusqlite::params;

struct TodoEntry {
    id: u32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
    
    #[error("Failed to get connection")]
    ConnectionPoolError(#[from] r2d2::Error),

    #[error("Failed SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}

impl ResponseError for MyError {}

#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    // データの準備
    let conn = db.get()?;
    let mut statement = conn.prepare("select id, text from todo")?;
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry {
            id, text
        })
    })?;
    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }

    // レスポンス
    let html = IndexTemplate { entries };
    let response_body = html.render()?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    // データベースの準備
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");
    let conn = pool
        .get()
        .expect("Fai;ed to get the connection from the pool.");
    conn.execute(
        "create table if not exists todo (
            id integer primary key autoincrement,
            text text not null
        )",
        params![],
    )
    .expect("Failed to create a table `todo`.");

    // サーバーを起動
    HttpServer::new(move || App::new().service(index).data(pool.clone()))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
