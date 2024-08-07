use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;

#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    email: String,
    age: i32,
}

async fn create_user(
    Json(payload): Json<User>,
    Extension(pool): Extension<sqlx::PgPool>,
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    sqlx::query!(
        "INSERT INTO users (name, email, age) VALUES ($1, $2, $3)",
        payload.name,
        payload.email,
        payload.age,
    )
    .execute(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(payload))
}

async fn get_users(
    Extension(pool): Extension<sqlx::PgPool>,
) -> Result<Json<Vec<User>>, (axum::http::StatusCode, String)> {
    let user = sqlx::query_as!(User, "SELECT name, email, age FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(user))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let app = Router::new()
        .route("/users", post(create_user).get(get_users))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
