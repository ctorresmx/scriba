use crate::models::BlogConfig;
use std::env;

pub fn get_blog_header_title(page_header: Option<&str>) -> String {
    let title = get_blog_config().title;
    match page_header {
        Some(header) => format!("{} - {}", header, title),
        None => title,
    }
}

pub fn get_blog_config() -> BlogConfig {
    BlogConfig {
        name: env::var("BLOG_NAME").unwrap_or_else(|_| "Scriba".to_string()),
        title: env::var("BLOG_TITLE").unwrap_or_else(|_| "Scriba".to_string()),
        copyright: env::var("BLOG_COPYRIGHT").unwrap_or_else(|_| "Scriba".to_string()),
        posts_dir: env::var("BLOG_POSTS_DIR").unwrap_or_else(|_| "./posts".to_string()),
        favicon_text: env::var("BLOG_FAVICON_TEXT").unwrap_or_else(|_| "Scr".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_env;

    #[test]
    fn test_get_blog_config_default_values() {
        let config = get_blog_config();

        assert_eq!(config.name, "Scriba");
        assert_eq!(config.title, "Scriba");
        assert_eq!(config.copyright, "Scriba");
        assert_eq!(config.posts_dir, "./posts");
        assert_eq!(config.favicon_text, "Scr");
    }

    #[test]
    fn test_get_blog_config_return_set_values() {
        temp_env::with_vars(
            [
                ("BLOG_NAME", Some("Test Blog")),
                ("BLOG_TITLE", Some("Test Title")),
                ("BLOG_COPYRIGHT", Some("Test Copyright")),
                ("BLOG_POSTS_DIR", Some("/custom/articles")),
                ("BLOG_FAVICON_TEXT", Some("TB")),
            ],
            || {
                let config = get_blog_config();

                assert_eq!(config.name, "Test Blog");
                assert_eq!(config.title, "Test Title");
                assert_eq!(config.copyright, "Test Copyright");
                assert_eq!(config.posts_dir, "/custom/articles");
                assert_eq!(config.favicon_text, "TB");
            },
        );
    }

    #[test]
    fn test_get_blog_header_title_no_page_header() {
        temp_env::with_var("BLOG_TITLE", Some("My Blog"), || {
            let title = get_blog_header_title(None);

            assert_eq!(title, "My Blog");
        });
    }

    #[test]
    fn test_get_blog_header_title_with_page_header() {
        temp_env::with_var("BLOG_TITLE", Some("My Blog"), || {
            let title = get_blog_header_title(Some("About"));

            assert_eq!(title, "About - My Blog");
        });
    }

    #[test]
    fn test_get_blog_header_title_with_default_values() {
        temp_env::with_var("BLOG_TITLE", None::<&str>, || {
            let title = get_blog_header_title(None);

            assert_eq!(title, "Scriba");
        });
    }

    #[test]
    fn test_get_blog_header_title_with_default_env_custom_header() {
        temp_env::with_var("BLOG_TITLE", None::<&str>, || {
            let title = get_blog_header_title(Some("Contact"));

            assert_eq!(title, "Contact - Scriba");
        });
    }
}
