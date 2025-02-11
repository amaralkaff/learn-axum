use axum::{
    extract::State,
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::domain::entities::follow::Follow;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateFollowRequest {
    #[schema(example = 1)]
    pub following_user_id: i32,
    #[schema(example = 2)]
    pub followed_user_id: i32,
}

/// Get hello message from follow handler
#[utoipa::path(
    get,
    path = "/follows/hello",
    tag = "follows",
    responses(
        (status = 200, description = "Hello message from follow handler", body = String)
    )
)]
pub async fn hello() -> &'static str {
    "Hello from follow handler!"
}

#[utoipa::path(
    post,
    path = "/follows",
    tag = "follows",
    request_body = CreateFollowRequest,
    responses(
        (status = 201, description = "Follow relationship created successfully", body = Follow),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_follow(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateFollowRequest>,
) -> Result<Json<Follow>, (axum::http::StatusCode, String)> {
    let now = Utc::now();
    let result = sqlx::query!(
        r#"
        INSERT INTO follows (following_user_id, followed_user_id, created_at)
        VALUES ($1, $2, $3)
        RETURNING following_user_id, followed_user_id, created_at as "created_at!"
        "#,
        payload.following_user_id,
        payload.followed_user_id,
        now
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(record) => {
            tracing::info!("Follow relationship created successfully");
            Ok(Json(Follow {
                following_user_id: record.following_user_id,
                followed_user_id: record.followed_user_id,
                created_at: record.created_at,
            }))
        },
        Err(e) => {
            tracing::error!("Failed to create follow relationship: {:?}", e);
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
                    format!("Failed to create follow relationship: {}", e),
                )),
            }
        }
    }
} 