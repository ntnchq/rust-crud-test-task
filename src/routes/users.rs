use axum::{Router, routing::{get}};
use crate::handlers::users::*;
use crate::config::database::DbPool;

pub fn user_routes() -> Router<DbPool> {
    Router::new()
        .route("/", get(get_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
}
