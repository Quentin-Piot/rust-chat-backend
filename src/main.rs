use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

use crate::groups::dto::CreateGroup;
use crate::groups::mutation::GroupMutation;
use axum::extract::Path;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use sea_orm::{Database, DatabaseConnection};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::users::dto::{CreateUser, UpdateUser};
use crate::users::mutation::UserMutation;

mod entities;
mod groups;
mod users;

#[derive(Clone)]
struct AppState {
    database: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_chat_backend=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let database = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let state = AppState { database };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .route("/groups", post(create_group))
        .route("/groups/:id", delete(delete_group))
        .with_state(state);

    let addr = SocketAddr::from_str(&server_url).unwrap();
    tracing::debug!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

// USER

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<PostResponse, (StatusCode, &'static str)> {
    UserMutation::create_user(&state.database, payload)
        .await
        .expect("Can't create user");
    Ok(StatusCode::CREATED)
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> Result<PostResponse, (StatusCode, &'static str)> {
    UserMutation::update_user(&state.database, id, payload)
        .await
        .expect("Can't update user");
    Ok(StatusCode::OK)
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<PostResponse, (StatusCode, &'static str)> {
    UserMutation::delete_user(&state.database, id)
        .await
        .expect("Can't delete user");
    Ok(StatusCode::OK)
}

// GROUP

async fn create_group(
    State(state): State<AppState>,
    Json(payload): Json<CreateGroup>,
) -> Result<PostResponse, (StatusCode, &'static str)> {
    GroupMutation::create_group(&state.database, payload)
        .await
        .expect("Can't create group");
    Ok(StatusCode::CREATED)
}

async fn delete_group(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<PostResponse, (StatusCode, &'static str)> {
    GroupMutation::delete_group(&state.database, id)
        .await
        .expect("Can't delete group");
    Ok(StatusCode::OK)
}
pub type PostResponse = StatusCode;
