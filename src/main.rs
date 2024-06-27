use anyhow::Result;
use axum::Router;
use axum::routing::{get, post};
use sqlx::{Pool, Sqlite, SqlitePool};
use sqlx::migrate::MigrateDatabase;
use tokio::net::TcpListener;
mod bo;
mod encryption;
mod handlers;

const DB_PATH: &str = r#"sqlite://ppaass.db"#;
fn build_router(db: Pool<Sqlite>) -> Router {
    Router::new()
        .route("/user/create", post(handlers::user::create_user))
        .route("/user/get/:user_name", get(handlers::user::get_user))
        .with_state(db)
}
async fn init_database() -> Result<Pool<Sqlite>> {
    if Sqlite::database_exists(DB_PATH).await? {
        return Ok(SqlitePool::connect(DB_PATH).await?);
    };
    Sqlite::create_database(DB_PATH).await?;
    let connection_pool = SqlitePool::connect(DB_PATH).await?;
    sqlx::query(
        r#"
            CREATE TABLE ppaass_user (
                user_name    TEXT PRIMARY KEY,
                proxy_private_key  TEXT NOT NULL,
                agent_public_key  TEXT NOT NULL
            );
            "#,
    )
    .execute(&connection_pool)
    .await?;
    return Ok(connection_pool);
}
#[tokio::main]
async fn main() -> Result<()> {
    let db = init_database().await?;
    let tcp_listener = TcpListener::bind("0.0.0.0:90").await?;
    let app = build_router(db);
    axum::serve(tcp_listener, app).await?;
    Ok(())
}
