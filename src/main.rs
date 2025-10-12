use axum::{Router, routing::get};
use tower_http::services::ServeDir;

mod config;
mod handlers;
mod markdown;
mod models;
mod templates;

use crate::{
    config::get_blog_config,
    handlers::{favicon, index, not_found_handler, post_page},
};

#[tokio::main]
async fn main() {
    let config = get_blog_config();
    let app = Router::new()
        .route("/", get(index))
        .route("/{year}/{month}/{day}/{slug}", get(post_page))
        .route("/favicon.ico", get(favicon))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(not_found_handler)
        .with_state(config);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
