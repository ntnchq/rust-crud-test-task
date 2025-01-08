use axum::{Router, routing::{get}};
use crate::handlers::posts::*;
use crate::config::database::DbPool;

pub fn post_routes() -> Router<DbPool> {
    Router::new()
        .route("/", get(get_posts).post(create_post))
        .route("/:id", get(get_post).put(update_post).delete(delete_post))
}