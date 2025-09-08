use crate::models::BlogConfig;
use crate::templates::IndexTemplate;
use askama::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use chrono::Datelike;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    Render(askama::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found").into_response(),
            AppError::Render(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Template render error").into_response()
            }
        }
    }
}

impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        AppError::Render(err)
    }
}

pub async fn index(State(config): State<BlogConfig>) -> Result<impl IntoResponse, AppError> {
    let template = IndexTemplate {
        title: config.title.clone(),
        copyright: config.copyright.clone(),
        current_year: chrono::Utc::now().year().to_string(),
        posts: vec![],
    };

    Ok(Html(template.render()?))
}
