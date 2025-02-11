use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "john_doe")]
    pub username: String,
    #[schema(example = "user")]
    pub role: String,
    #[schema(value_type = String, format = "date-time", example = "2024-02-11T00:00:00Z")]
    pub created_at: DateTime<Utc>,
}