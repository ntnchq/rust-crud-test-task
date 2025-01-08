use axum::{extract::Path, Json};
use axum::extract::State;
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::user::User;
use crate::schema::{posts, users};
use crate::config::database::DbPool;
use chrono::Utc;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub nick: String,
}


#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub nick: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn get_users(State(pool): State<DbPool>) -> Json<Vec<UserResponse>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let users = users::dsl::users
        .filter(users::dsl::deleted_at.is_null()) // Исключаем удаленных пользователей
        .load::<User>(&mut conn)
        .expect("Failed to load users");

    let response = users
        .into_iter()
        .map(|user| UserResponse {
            id: user.id,
            nick: user.nick,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
        })
        .collect();

    Json(response)
}



pub async fn get_user(Path(id): Path<Uuid>, State(pool): State<DbPool>) -> Json<Option<UserResponse>> {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let user = users::dsl::users
        .filter(users::dsl::id.eq(id))
        .first::<User>(&mut conn)
        .ok();

    Json(user.map(|u| UserResponse {
        id: u.id,
        nick: u.nick,
        created_at: u.created_at.to_string(),
        updated_at: u.updated_at.to_string(),
    }))
}

pub async fn create_user(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUser>,
) -> Json<UserResponse> {
    tracing::info!("Received request to create user: {:?}", payload);

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            tracing::error!("Failed to get DB connection: {}", err);
            panic!("DB connection error");
        }
    };

    let new_user = User {
        id: Uuid::new_v4(),
        nick: payload.nick,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        deleted_at: None,
    };

    if let Err(err) = diesel::insert_into(users::dsl::users)
        .values(&new_user)
        .execute(&mut conn)
    {
        tracing::error!("Failed to insert user: {}", err);
        panic!("DB insert error");
    }

    tracing::info!("Successfully created user: {:?}", new_user);

    Json(UserResponse {
        id: new_user.id,
        nick: new_user.nick,
        created_at: new_user.created_at.to_string(),
        updated_at: new_user.updated_at.to_string(),
    })
}

pub async fn update_user(
    State(pool): State<DbPool>, // State всегда последним
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateUser>,
) -> Json<Option<UserResponse>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let updated_user = diesel::update(users::dsl::users.filter(users::dsl::id.eq(id)))
        .set((
            users::dsl::nick.eq(payload.nick),
            users::dsl::updated_at.eq(Utc::now().naive_utc()),
        ))
        .get_result::<User>(&mut conn)
        .ok();

    Json(updated_user.map(|u| UserResponse {
        id: u.id,
        nick: u.nick,
        created_at: u.created_at.to_string(),
        updated_at: u.updated_at.to_string(),
    }))
}


pub async fn delete_user(Path(id): Path<Uuid>, State(pool): State<DbPool>) -> Json<bool> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let posts_deleted = diesel::update(posts::dsl::posts.filter(posts::dsl::user_id.eq(id)))
        .set(posts::dsl::deleted_at.eq(Some(Utc::now().naive_utc())))
        .execute(&mut conn)
        .expect("Failed to delete posts");

    let user_deleted = diesel::update(users::dsl::users.filter(users::dsl::id.eq(id)))
        .set(users::dsl::deleted_at.eq(Some(Utc::now().naive_utc())))
        .execute(&mut conn)
        .expect("Failed to delete user");

    Json(posts_deleted > 0 || user_deleted > 0)
}

