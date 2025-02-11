use axum::{
    extract::State,
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::domain::entities::post::Post;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreatePostRequest {
    #[schema(example = "My First Post")]
    pub title: String,
    #[schema(example = "This is the content of my first post")]
    pub body: String,
    #[schema(example = 1)]
    pub user_id: i32,
}

#[utoipa::path(
    get,
    path = "/posts/hello",
    tag = "posts",
    responses(
        (status = 200, description = "Hello message from post handler", body = String)
    )
)]
pub async fn hello() -> &'static str {
    "Hello from post handler!"
}

#[utoipa::path(
    post,
    path = "/posts",
    tag = "posts",
    request_body = CreatePostRequest,
    responses(
        (status = 201, description = "Post created successfully", body = Post),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_post(
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<Post>, (axum::http::StatusCode, String)> {
    let now = Utc::now();
    let result = sqlx::query!(
        r#"
        INSERT INTO posts (title, body, user_id, status, created_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, title, body, user_id, status, created_at as "created_at!"
        "#,
        payload.title,
        payload.body,
        payload.user_id,
        "published",
        now
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(record) => {
            tracing::info!("Post created successfully with id: {}", record.id);
            Ok(Json(Post {
                id: record.id,
                title: record.title,
                body: record.body,
                user_id: record.user_id,
                status: record.status,
                created_at: record.created_at,
            }))
        },
        Err(e) => {
            tracing::error!("Failed to create post: {:?}", e);
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
                    format!("Failed to create post: {}", e),
                )),
            }
        }
    }
} 