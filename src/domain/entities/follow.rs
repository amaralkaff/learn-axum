use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Follow {
    #[schema(example = 1)]
    pub following_user_id: i32,
    #[schema(example = 2)]
    pub followed_user_id: i32,
    #[schema(value_type = String, format = "date-time", example = "2024-02-11T00:00:00Z")]
    pub created_at: DateTime<Utc>,
}