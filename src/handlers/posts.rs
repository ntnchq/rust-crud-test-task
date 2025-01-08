use axum::{extract::{State, Path}, Json};
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::config::database::DbPool;
use crate::models::post::Post;
use crate::schema::posts;

#[derive(Deserialize)]
pub struct CreatePost {
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn get_posts(State(pool): State<DbPool>) -> Json<Vec<PostResponse>> {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let posts = posts::dsl::posts
        .filter(posts::dsl::deleted_at.is_null())
        .load::<Post>(&mut conn)
        .expect("Error loading posts");

    let response = posts
        .into_iter()
        .map(|post| PostResponse {
            id: post.id,
            user_id: post.user_id,
            title: post.title,
            body: post.body,
            created_at: post.created_at.to_string(),
            updated_at: post.updated_at.to_string(),
        })
        .collect();

    Json(response)
}

pub async fn get_post(Path(id): Path<Uuid>, State(pool): State<DbPool>) -> Json<Option<PostResponse>> {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let post = posts::dsl::posts
        .filter(posts::dsl::id.eq(id))
        .filter(posts::dsl::deleted_at.is_null())
        .first::<Post>(&mut conn)
        .ok();

    Json(post.map(|p| PostResponse {
        id: p.id,
        user_id: p.user_id,
        title: p.title,
        body: p.body,
        created_at: p.created_at.to_string(),
        updated_at: p.updated_at.to_string(),
    }))
}

pub async fn create_post(
    State(pool): State<DbPool>,
    Json(payload): Json<CreatePost>,
) -> Json<PostResponse> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let new_post = Post {
        id: Uuid::new_v4(),
        user_id: payload.user_id,
        title: payload.title,
        body: payload.body,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        deleted_at: None,
    };

    diesel::insert_into(posts::dsl::posts)
        .values(&new_post)
        .execute(&mut conn)
        .expect("Failed to insert post");

    Json(PostResponse {
        id: new_post.id,
        user_id: new_post.user_id,
        title: new_post.title,
        body: new_post.body,
        created_at: new_post.created_at.to_string(),
        updated_at: new_post.updated_at.to_string(),
    })
}

pub async fn update_post(
    Path(id): Path<Uuid>,
    State(pool): State<DbPool>,
    Json(payload): Json<CreatePost>,
) -> Json<Option<PostResponse>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let updated_post = diesel::update(posts::dsl::posts.filter(posts::dsl::id.eq(id)))
        .set((
            posts::dsl::title.eq(payload.title),
            posts::dsl::body.eq(payload.body),
            posts::dsl::updated_at.eq(Utc::now().naive_utc()),
        ))
        .get_result::<Post>(&mut conn)
        .ok();

    Json(updated_post.map(|p| PostResponse {
        id: p.id,
        user_id: p.user_id,
        title: p.title,
        body: p.body,
        created_at: p.created_at.to_string(),
        updated_at: p.updated_at.to_string(),
    }))
}

pub async fn delete_post(Path(id): Path<Uuid>, State(pool): State<DbPool>) -> Json<bool> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let rows_affected = diesel::update(posts::dsl::posts.filter(posts::dsl::id.eq(id)))
        .set(posts::dsl::deleted_at.eq(Some(Utc::now().naive_utc())))
        .execute(&mut conn)
        .expect("Failed to delete post");

    Json(rows_affected > 0)
}
