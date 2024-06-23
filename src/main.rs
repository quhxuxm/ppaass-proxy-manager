use anyhow::Result;
use axum::Router;
use axum::routing::{get, post};
use tokio::net::TcpListener;
mod bo;
mod encryption;
mod handlers;
fn build_router() -> Router {
    Router::new()
        .route("/", get(handlers::index))
        .route("/user/create", post(handlers::user::create_user))
        .route("/user/get/:user_name", get(handlers::user::get_user))
}

#[tokio::main]
async fn main() -> Result<()> {
    let tcp_listener = TcpListener::bind("0.0.0.0:90").await?;
    let app = build_router();
    axum::serve(tcp_listener, app).await?;
    Ok(())
}
