use anyhow::Result;
use axum::Router;
use axum::routing::{get, post};
use rusqlite::Connection;
use tokio::net::TcpListener;
mod bo;
mod encryption;
mod handlers;

fn build_router() -> Router {
    Router::new()
        .route("/user/create", post(handlers::user::create_user))
        .route("/user/get/:user_name", get(handlers::user::get_user))
}

fn init_database() -> Result<()> {
    let sqlite_connection = Connection::open(r#"C:\Development\ppaass_db.csv"#)?;
    sqlite_connection.execute(
        r#"
                CREATE TABLE ppaass_user (
                    user_name    TEXT PRIMARY KEY,
                    proxy_private_key  TEXT NOT NULL,
                    agent_public_key  TEXT NOT NULL
                )
                "#,
        (),
    )?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    init_database()?;
    let tcp_listener = TcpListener::bind("0.0.0.0:90").await?;
    let app = build_router();
    axum::serve(tcp_listener, app).await?;
    Ok(())
}
