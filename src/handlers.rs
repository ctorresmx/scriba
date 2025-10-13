use crate::config::get_blog_header_title;
use crate::markdown::get_all_posts;
use crate::models::BlogConfig;
use crate::templates::{IndexTemplate, NotFoundTemplate, PostTemplate};
use askama::Template;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use chrono::Datelike;

#[derive(Debug)]
pub enum AppError {
    #[allow(dead_code)]
    NotFound,
    #[allow(dead_code)]
    Render(askama::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                Html("<html><head><title>404 - Page not found</title></head><body><h1>404 - Page not found</h1><p>The page you were looking for doesn't exist.</p></body></html>")
            ).into_response(),
            AppError::Render(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(format!("<html><head><title>Error</title></head><body><h1>Internal Server Error</h1><p>Template render error: {}</p></body></html>", err))
            ).into_response(),
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
        header_title: get_blog_header_title(None),
        blog_title: config.title,
        copyright: config.copyright.clone(),
        current_year: chrono::Utc::now().year().to_string(),
        posts: posts,
    };

    Ok(Html(template.render()?))
}

pub async fn post_page(
    State(config): State<BlogConfig>,
    Path((year, month, day, slug)): Path<(String, String, String, String)>,
) -> impl IntoResponse {
    let posts = get_all_posts();

    let found_post = posts.iter().find(|entry| {
        entry.formatted_date == format!("{}/{}/{}", year, month, day) && entry.slug == slug
    });

    match found_post {
        Some(post) => {
            let template = PostTemplate {
                header_title: get_blog_header_title(Some(&post.attributes.title)),
                blog_title: config.title.clone(),
                copyright: config.copyright.clone(),
                current_year: chrono::Utc::now().year().to_string(),
                post: post.clone(),
            };

            match template.render() {
                Ok(html) => (StatusCode::OK, Html(html)).into_response(),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html("<html><body><h1>Error rendering template</h1></body></html>".to_string())
                ).into_response(),
            }
        }
        None => {
            let template = NotFoundTemplate {
                header_title: "404 - Page not found".to_string(),
                blog_title: config.title,
                copyright: config.copyright.clone(),
                current_year: chrono::Utc::now().year().to_string(),
            };

            let html = template.render().unwrap_or_else(|_| {
                "<!DOCTYPE html><html><body><h1>404 - Page not found</h1></body></html>".to_string()
            });

            (StatusCode::NOT_FOUND, Html(html)).into_response()
        }
    }
}

pub async fn favicon(State(config): State<BlogConfig>) -> impl IntoResponse {
    let favicon_text = config.favicon_text.as_str();

    let svg_body = format!(
        "<svg width=\"32\" height=\"32\" xmlns=\"http://www.w3.org/2000/svg\">\
        <rect width=\"32\" height=\"32\" fill=\"#2a323c\"/>\
        <text x=\"16\" y=\"20\" font-family=\"monospace\" font-size=\"14\" fill=\"#cdd6f4\" text-anchor=\"middle\">{}</text>\
        </svg>",
        favicon_text
    );

    (
        StatusCode::OK,
        [
            ("Content-Type", "image/svg+xml"),
            ("Cache-Control", "public, max-age=86400"),
        ],
        svg_body,
    )
}

pub async fn not_found_handler(State(config): State<BlogConfig>) -> impl IntoResponse {
    let template = NotFoundTemplate {
        header_title: "404 - Page not found".to_string(),
        blog_title: config.title,
        copyright: config.copyright.clone(),
        current_year: chrono::Utc::now().year().to_string(),
    };

    let html = template.render().unwrap_or_else(|_| {
        "<!DOCTYPE html><html><body><h1>404 - Page not found</h1></body></html>".to_string()
    });

    (StatusCode::NOT_FOUND, Html(html))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ParsedPost, PostAttributes, PostStatus};

    fn create_test_config() -> BlogConfig {
        BlogConfig {
            name: "Test Blog".to_string(),
            title: "Test Title".to_string(),
            copyright: "Test Copyright".to_string(),
            posts_dir: "./test_posts".to_string(),
            favicon_text: "TB".to_string(),
            port: "8000".to_string(),
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
        // This test verifies that requesting a non-existent post returns 404 with styled template
        let config = create_test_config();
        let path = (
            "2025".to_string(),
            "01".to_string(),
            "15".to_string(),
            "nonexistent-post".to_string(),
        );

        let response = post_page(axum::extract::State(config), axum::extract::Path(path)).await;
        let response = response.into_response();

        // Verify 404 status code
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // Verify the response body contains the 404 template elements
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let html = String::from_utf8(body.to_vec()).unwrap();

        // Should contain navbar, footer, and 404 message from template
        assert!(html.contains("404 - Page not found"));
        assert!(html.contains("Test Title")); // Blog title in navbar
        assert!(html.contains("Test Copyright")); // Footer copyright
    }

    #[test]
    fn test_post_template_rendering() {
        // This test verifies that the PostTemplate can render without syntax errors
        let post = create_test_post();
        let template = PostTemplate {
            header_title: "Test Blog".to_string(),
            blog_title: "Test Blog".to_string(),
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
        assert!(html.contains(
            r#"<div class="flex flex-wrap items-center gap-1 text-sm text-base-content/70 mb-4">"#
        ));
    }
}
