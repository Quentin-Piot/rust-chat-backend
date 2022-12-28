use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

use crate::entities::prelude::User;
use axum::{
    extract::{FromRef, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::entities::user;

mod entities;

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
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from_str(&server_url).unwrap();
    tracing::debug!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root(State(state): State<AppState>) -> &'static str {
    "Hello, World!"
}

async fn create_user(
    State(state): State<AppState>,
) -> Result<PostResponse, (StatusCode, &'static str)> {
    // insert your application logic here
    let user = user::ActiveModel {
        email: ActiveValue::Set("test@test".to_owned()),
        password: ActiveValue::Set("password".to_owned()),
        ..Default::default()
    };

    user.save(&state.database).await.expect("ooops");

    Ok((StatusCode::CREATED))
}

pub type PostResponse = (StatusCode);
