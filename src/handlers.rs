use crate::markdown::get_all_posts;
use crate::models::BlogConfig;
use crate::templates::{IndexTemplate, PostTemplate};
use askama::Template;
use axum::extract::{Path, State};
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
    let posts = get_all_posts();

    let template = IndexTemplate {
        title: config.title.clone(),
        copyright: config.copyright.clone(),
        current_year: chrono::Utc::now().year().to_string(),
        posts: posts,
    };

    Ok(Html(template.render()?))
}

pub async fn post_page(
    State(config): State<BlogConfig>,
    Path((year, month, day, slug)): Path<(String, String, String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let posts = get_all_posts();

    let found_post = posts.iter().find(|entry| {
        entry.formatted_date == format!("{}/{}/{}", year, month, day) && entry.slug == slug
    });

    match found_post {
        Some(post) => {
            let template = PostTemplate {
                title: config.title.clone(),
                copyright: config.copyright.clone(),
                current_year: chrono::Utc::now().year().to_string(),
                post: post.clone(),
            };

            Ok(Html(template.render()?))
        }
        None => Err(AppError::NotFound),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{PostAttributes, PostStatus, ParsedPost};

    fn create_test_config() -> BlogConfig {
        BlogConfig {
            name: "Test Blog".to_string(),
            title: "Test Title".to_string(),
            copyright: "Test Copyright".to_string(),
            posts_dir: "./test_posts".to_string(),
            favicon_text: "TB".to_string(),
        }
    }

    fn create_test_post() -> ParsedPost {
        ParsedPost {
            attributes: PostAttributes {
                title: "Test Post".to_string(),
                date: "2025-01-15".to_string(),
                author: "Test Author".to_string(),
                tags: vec!["test".to_string()],
                status: PostStatus::Published,
                excerpt: Some("Test excerpt".to_string()),
            },
            content: "Test content".to_string(),
            slug: "test-post".to_string(),
            url: "/2025/01/15/test-post".to_string(),
            formatted_date: "2025/01/15".to_string(),
        }
    }

    #[tokio::test]
    async fn test_post_page_returns_404_for_nonexistent_post() {
        // This test verifies that requesting a non-existent post returns 404 instead of panicking
        let config = create_test_config();
        let path = ("2025".to_string(), "01".to_string(), "15".to_string(), "nonexistent-post".to_string());
        
        let result = post_page(axum::extract::State(config), axum::extract::Path(path)).await;
        
        match result {
            Err(AppError::NotFound) => {
                // This is the expected behavior - should return NotFound error
            }
            Ok(_) => panic!("Expected NotFound error, but got success"),
            Err(other) => panic!("Expected NotFound error, but got: {:?}", other),
        }
    }

    #[test]
    fn test_post_template_rendering() {
        // This test verifies that the PostTemplate can render without syntax errors
        let post = create_test_post();
        let template = PostTemplate {
            title: "Test Blog".to_string(),
            copyright: "Test Copyright".to_string(),
            current_year: "2025".to_string(),
            post: post,
        };

        let result = template.render();
        assert!(result.is_ok(), "Template rendering failed: {:?}", result);

        let html = result.unwrap();
        // Verify the HTML contains expected elements and is properly formed
        assert!(html.contains("Test Post")); // Post title
        assert!(html.contains("Test Author")); // Author
        assert!(html.contains("2025/01/15")); // Date
        assert!(html.contains("Test content")); // Content
        assert!(html.contains("test")); // Tag
        // Verify the div tag is properly closed (this verifies our HTML syntax fix)
        assert!(html.contains(r#"<div class="flex flex-wrap items-center gap-1 text-sm text-base-content/70 mb-4">"#));
    }
}
