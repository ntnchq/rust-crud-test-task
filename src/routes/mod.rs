use axum::Router;
pub mod users;
pub mod posts;
use crate::config::database::DbPool;
use tracing::info;

pub fn create_routes() -> Router<DbPool> {
    info!("Setting up routes");
    Router::new()
        .nest("/users", users::user_routes())
        .nest("/posts", posts::post_routes())
}