mod domain;
mod infrastructure;
mod interfaces;

use axum::{
    routing::{get, post},
    Router,
    serve,
};
use dotenv::dotenv;
use infrastructure::database::postgres::create_pool;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use interfaces::handlers::{
    user_handler,
    post_handler,
    follow_handler,
};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use domain::entities::{user::User, post::Post, follow::Follow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(value_type = String, format = "date-time", example = "2024-02-11T00:00:00Z")]
struct CustomDateTime(DateTime<Utc>);

#[derive(OpenApi)]
#[openapi(
    paths(
        user_handler::hello,
        user_handler::create_user,
        post_handler::hello,
        post_handler::create_post,
        follow_handler::hello,
        follow_handler::create_follow
    ),
    components(
        schemas(
            User, 
            Post, 
            Follow, 
            post_handler::CreatePostRequest,
            user_handler::CreateUserRequest,
            follow_handler::CreateFollowRequest,
            CustomDateTime
        )
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "posts", description = "Post management endpoints"),
        (name = "follows", description = "Follow management endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // Load .env file
    if let Ok(_) = dotenv() {
        println!("Successfully loaded .env file");
    } else {
        println!("Warning: .env file not found");
    }
    
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tracing::info!("Starting application...");

    // Create database pool
    tracing::info!("Creating database pool...");
    let pool = create_pool().await;
    tracing::info!("Database pool created successfully");

    // Build router
    tracing::info!("Building router...");
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users/hello", get(user_handler::hello))
        .route("/users", post(user_handler::create_user))
        .route("/posts/hello", get(post_handler::hello))
        .route("/posts", post(post_handler::create_post))
        .route("/follows/hello", get(follow_handler::hello))
        .route("/follows", post(follow_handler::create_follow))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);
    tracing::info!("Router built successfully");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Starting server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Server started successfully, listening on {}", addr);
    println!("Swagger UI available at: http://localhost:3000/swagger-ui/");
    
    serve(listener, app).await.unwrap();
}
