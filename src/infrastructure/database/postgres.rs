use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub async fn create_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Test koneksi
    match test_connection(&pool).await {
        Ok(_) => {
            tracing::info!("Successfully connected to the database");
            pool
        }
        Err(e) => {
            tracing::error!("Failed to connect to the database: {}", e);
            panic!("Database connection failed");
        }
    }
}

async fn test_connection(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map(|_| ())
} 