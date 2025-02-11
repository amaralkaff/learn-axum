use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Post {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "My First Post")]
    pub title: String,
    #[schema(example = "This is the content of my first post")]
    pub body: String,
    #[schema(example = 1)]
    pub user_id: i32,
    #[schema(example = "published")]
    pub status: String,
    #[schema(value_type = String, format = "date-time", example = "2024-02-11T00:00:00Z")]
    pub created_at: DateTime<Utc>,
}