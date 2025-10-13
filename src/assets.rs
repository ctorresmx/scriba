use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::path::PathBuf;
use tokio::fs;

use crate::models::BlogConfig;

const SUPPORTED_IMAGE_TYPES: &[(&str, &str)] = &[
    (".jpg", "image/jpeg"),
    (".jpeg", "image/jpeg"),
    (".png", "image/png"),
    (".gif", "image/gif"),
    (".webp", "image/webp"),
    (".svg", "image/svg+xml"),
];

pub async fn serve_asset(State(config): State<BlogConfig>, Path(path): Path<String>) -> Response {
    // Validate path for security - block path traversal attempts
    if path.contains("../") || path.contains("..\\") {
        eprintln!("Blocked path traversal attempt in asset path: {}", path);
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }

    // Block unsafe protocols and UNC paths
    if path.starts_with("file:") || path.starts_with("ftp:") || path.starts_with("\\\\") {
        eprintln!("Blocked unsafe protocol in asset path: {}", path);
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }

    // Construct the full file path
    let posts_dir = PathBuf::from(&config.posts_dir);
    let file_path = posts_dir.join(&path);

    // Additional security check: ensure resolved path is within posts directory
    match file_path.canonicalize() {
        Ok(canonical_path) => {
            let canonical_posts_dir = match posts_dir.canonicalize() {
                Ok(dir) => dir,
                Err(_) => {
                    eprintln!("Failed to canonicalize posts directory");
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
                        .into_response();
                }
            };

            if !canonical_path.starts_with(&canonical_posts_dir) {
                eprintln!("Blocked asset path outside posts directory: {}", path);
                return (StatusCode::FORBIDDEN, "Forbidden").into_response();
            }
        }
        Err(_) => {
            // File doesn't exist, return 404
            return (StatusCode::NOT_FOUND, "Image not found").into_response();
        }
    }

    // Get file extension
    let ext = match file_path.extension().and_then(|e| e.to_str()) {
        Some(e) => format!(".{}", e.to_lowercase()),
        None => {
            return (StatusCode::BAD_REQUEST, "Unsupported file type").into_response();
        }
    };

    // Check if it's a supported image type
    let mime_type = SUPPORTED_IMAGE_TYPES
        .iter()
        .find(|(extension, _)| *extension == ext)
        .map(|(_, mime)| *mime);

    if mime_type.is_none() {
        return (StatusCode::BAD_REQUEST, "Unsupported file type").into_response();
    }

    // Read the file
    let file_content = match fs::read(&file_path).await {
        Ok(content) => content,
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                return (StatusCode::NOT_FOUND, "Image not found").into_response();
            }

            eprintln!("Error serving image: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response();
        }
    };

    // Return the image with appropriate headers
    (
        StatusCode::OK,
        [
            ("Content-Type", mime_type.unwrap()),
            ("Cache-Control", "public, max-age=31536000, immutable"),
        ],
        file_content,
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::{Path, State};
    use std::fs as std_fs;
    use tempfile::TempDir;

    fn create_test_config(temp_dir: &TempDir) -> BlogConfig {
        BlogConfig {
            name: "Test Blog".to_string(),
            title: "Test Title".to_string(),
            copyright: "Test Copyright".to_string(),
            posts_dir: temp_dir.path().to_string_lossy().to_string(),
            favicon_text: "TB".to_string(),
            port: "8000".to_string(),
        }
    }

    #[tokio::test]
    async fn test_serve_asset_jpg() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);

        let image_content = vec![0xFF, 0xD8, 0xFF];
        std_fs::write(temp_dir.path().join("test.jpg"), &image_content).unwrap();

        let response = serve_asset(State(config), Path("test.jpg".to_string())).await;

        assert_eq!(response.status(), StatusCode::OK);
        let headers = response.headers();
        assert_eq!(headers.get("Content-Type").unwrap(), "image/jpeg");
        assert_eq!(
            headers.get("Cache-Control").unwrap(),
            "public, max-age=31536000, immutable"
        );

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(body.to_vec(), image_content);
    }

    #[tokio::test]
    async fn test_serve_asset_png() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);

        let image_content = vec![0x89, 0x50, 0x4E, 0x47];
        std_fs::write(temp_dir.path().join("test.png"), &image_content).unwrap();

        let response = serve_asset(State(config), Path("test.png".to_string())).await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get("Content-Type").unwrap(), "image/png");
    }

    #[tokio::test]
    async fn test_serve_asset_case_insensitive() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);

        let image_content = vec![0x89, 0x50, 0x4E, 0x47];
        std_fs::write(temp_dir.path().join("test.PNG"), &image_content).unwrap();

        let response = serve_asset(State(config), Path("test.PNG".to_string())).await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get("Content-Type").unwrap(), "image/png");
    }

    #[tokio::test]
    async fn test_serve_asset_unsupported_file_type() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);

        std_fs::write(temp_dir.path().join("test.txt"), "content").unwrap();

        let response = serve_asset(State(config), Path("test.txt".to_string())).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(body, "Unsupported file type");
    }

    #[tokio::test]
    async fn test_serve_asset_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);

        let response = serve_asset(State(config), Path("nonexistent.png".to_string())).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(body, "Image not found");
    }

    #[tokio::test]
    async fn test_serve_asset_nested_path() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);

        std_fs::create_dir_all(temp_dir.path().join("subfolder")).unwrap();
        let image_content = vec![0x89, 0x50, 0x4E, 0x47];
        std_fs::write(temp_dir.path().join("subfolder/nested.png"), &image_content).unwrap();

        let response = serve_asset(State(config), Path("subfolder/nested.png".to_string())).await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get("Content-Type").unwrap(), "image/png");

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(body.to_vec(), image_content);
    }

    #[tokio::test]
    async fn test_serve_asset_blocks_path_traversal() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);

        let malicious_paths = vec![
            "../../../etc/passwd",
            "..\\..\\..\\windows\\system32\\config\\sam",
            "../config.json",
            "subfolder/../../../secret.txt",
            "..\\..\\sensitive.png",
        ];

        for malicious_path in malicious_paths {
            let response =
                serve_asset(State(config.clone()), Path(malicious_path.to_string())).await;

            assert_eq!(
                response.status(),
                StatusCode::FORBIDDEN,
                "Failed to block path: {}",
                malicious_path
            );
            let body = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap();
            assert_eq!(body, "Forbidden");
        }
    }

    #[tokio::test]
    async fn test_serve_asset_blocks_unsafe_protocols() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);

        let unsafe_paths = vec![
            "file:///etc/passwd",
            "ftp://malicious.com/image.png",
            "\\\\server\\share\\image.png",
        ];

        for unsafe_path in unsafe_paths {
            let response = serve_asset(State(config.clone()), Path(unsafe_path.to_string())).await;

            assert_eq!(
                response.status(),
                StatusCode::FORBIDDEN,
                "Failed to block path: {}",
                unsafe_path
            );
            let body = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap();
            assert_eq!(body, "Forbidden");
        }
    }

    #[tokio::test]
    async fn test_serve_asset_file_without_extension() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);

        std_fs::write(temp_dir.path().join("README"), "content").unwrap();

        let response = serve_asset(State(config), Path("README".to_string())).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(body, "Unsupported file type");
    }

    #[tokio::test]
    async fn test_serve_asset_all_supported_types() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);

        let test_images = vec![
            ("test.jpg", vec![0xFF, 0xD8, 0xFF], "image/jpeg"),
            ("test.jpeg", vec![0xFF, 0xD8, 0xFF], "image/jpeg"),
            ("test.png", vec![0x89, 0x50, 0x4E, 0x47], "image/png"),
            ("test.gif", vec![0x47, 0x49, 0x46, 0x38], "image/gif"),
            ("test.webp", vec![0x52, 0x49, 0x46, 0x46], "image/webp"),
            ("test.svg", b"<svg></svg>".to_vec(), "image/svg+xml"),
        ];

        for (filename, content, expected_mime) in test_images {
            std_fs::write(temp_dir.path().join(filename), &content).unwrap();

            let response = serve_asset(State(config.clone()), Path(filename.to_string())).await;

            assert_eq!(
                response.status(),
                StatusCode::OK,
                "Failed for file: {}",
                filename
            );
            assert_eq!(
                response.headers().get("Content-Type").unwrap(),
                expected_mime,
                "Wrong MIME type for: {}",
                filename
            );

            let body = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap();
            assert_eq!(body.to_vec(), content, "Wrong content for: {}", filename);
        }
    }

    #[tokio::test]
    async fn test_serve_asset_legitimate_nested_paths() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);

        std_fs::create_dir_all(temp_dir.path().join("images/gallery")).unwrap();
        let image_content = vec![0x89, 0x50, 0x4E, 0x47];
        std_fs::write(
            temp_dir.path().join("images/gallery/photo.png"),
            &image_content,
        )
        .unwrap();

        let legitimate_paths = vec![
            ("images/gallery/photo.png", true),
            ("subfolder/image.jpg", false),
            ("deep/nested/folder/image.png", false),
        ];

        for (path, should_exist) in legitimate_paths {
            let response = serve_asset(State(config.clone()), Path(path.to_string())).await;

            if should_exist {
                assert_eq!(
                    response.status(),
                    StatusCode::OK,
                    "Failed for existing path: {}",
                    path
                );
            } else {
                // Should be 404 (not found), not 403 (forbidden)
                assert_eq!(
                    response.status(),
                    StatusCode::NOT_FOUND,
                    "Wrong status for non-existent path: {}",
                    path
                );
            }
        }
    }
}
