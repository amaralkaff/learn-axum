use axum::{
    extract::State,
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::domain::entities::user::User;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateUserRequest {
    #[schema(example = "johndoe")]
    pub username: String,
    #[schema(example = "user")]
    pub role: String,
}

#[utoipa::path(
    get,
    path = "/users/hello",
    tag = "users",
    responses(
        (status = 200, description = "Hello message from user handler", body = String)
    )
)]
pub async fn hello() -> &'static str {
    "Hello from user handler!"
}

#[utoipa::path(
    post,
    path = "/users",
    tag = "users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let now = Utc::now();
    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, role, created_at)
        VALUES ($1, $2, $3)
        RETURNING id, username, role, created_at as "created_at!"
        "#,
        payload.username,
        payload.role,
        now
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(record) => {
            tracing::info!("User created successfully with id: {}", record.id);
            Ok(Json(User {
                id: record.id,
                username: record.username,
                role: record.role,
                created_at: record.created_at,
            }))
        },
        Err(e) => {
            tracing::error!("Failed to create user: {:?}", e);
            match e {
                sqlx::Error::Database(db_error) => {
                    if db_error.constraint().is_some() {
                        Err((
                            axum::http::StatusCode::BAD_REQUEST,
                            format!("Database constraint violation: {}", db_error),
                        ))
                    } else {
                        Err((
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Database error: {}", db_error),
                        ))
                    }
                }
                _ => Err((
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to create user: {}", e),
                )),
            }
        }
    }
} 