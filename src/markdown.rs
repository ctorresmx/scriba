use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::Parser;
use regex::Regex;
use std::ffi::OsStr;
use std::fs;

use crate::config::get_blog_config;
use crate::models::{ParsedPost, PostAttributes, PostStatus};

pub fn generate_slug(title: &str) -> String {
    // Remove special characters (keep only a-z, 0-9, spaces, and hyphens)
    let binding = title.to_lowercase();
    let re_special = Regex::new(r"[^a-z0-9\s\-]").unwrap();
    let step1 = re_special.replace_all(&binding, "");

    // Replace one or more spaces with single hyphen
    let re_spaces = Regex::new(r"\s+").unwrap();
    let step2 = re_spaces.replace_all(&step1, "-");

    // Replace multiple consecutive hyphens with single hyphen
    let re_hyphens = Regex::new(r"-+").unwrap();
    let step3 = re_hyphens.replace_all(&step2, "-");

    // Remove leading and trailing hyphens
    let re_trim = Regex::new(r"^-+|-+$").unwrap();
    re_trim.replace_all(&step3, "").trim().to_string()
}

pub fn generate_url(date: &str, slug: &str) -> String {
    format!("/{}/{}", date, slug)
}

pub fn transform_date_format(date_str: &str) -> String {
    date_str.replace('-', "/")
}

pub fn get_all_posts() -> Vec<ParsedPost> {
    let blog_config = get_blog_config();

    if let Ok(post_files) = fs::read_dir(blog_config.posts_dir) {
        let matter = Matter::<YAML>::new();
        let mut posts: Vec<ParsedPost> = post_files
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension() == Some(OsStr::new("md")))
            .map(|file_path| fs::read_to_string(file_path.as_path()))
            .filter_map(Result::ok)
            .map(|file_content| matter.parse::<PostAttributes>(file_content.as_str()))
            .filter_map(Result::ok)
            .filter_map(|post| {
                post.data.map(|p| {
                    let slug = generate_slug(&p.title);
                    let date = transform_date_format(&p.date);
                    let content_parser = Parser::new(post.content.as_str());
                    let mut parsed_content = String::new();
                    pulldown_cmark::html::push_html(&mut parsed_content, content_parser);
                    ParsedPost {
                        attributes: p,
                        content: parsed_content,
                        slug: slug.clone(),
                        url: generate_url(&date, &slug.as_str()),
                        formatted_date: date,
                    }
                })
            })
            .filter(|post| post.attributes.status == PostStatus::Published)
            .collect();
        posts.sort_by(|a, b| b.formatted_date.cmp(&a.formatted_date));
        posts
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_slug_basic_conversion() {
        assert_eq!(generate_slug("Hello World"), "hello-world");
        assert_eq!(generate_slug("My First Blog Post"), "my-first-blog-post");
        assert_eq!(generate_slug("Testing 123"), "testing-123");
    }

    #[test]
    fn test_generate_slug_special_characters_removal() {
        assert_eq!(generate_slug("Hello, World!"), "hello-world");
        assert_eq!(generate_slug("C++ Programming"), "c-programming");
        assert_eq!(generate_slug("React & TypeScript"), "react-typescript");
        assert_eq!(generate_slug("@username: A Story"), "username-a-story");
    }

    #[test]
    fn test_generate_slug_multiple_spaces_and_hyphens() {
        assert_eq!(generate_slug("Hello   World"), "hello-world");
        assert_eq!(generate_slug("Hello -- World"), "hello-world");
        assert_eq!(generate_slug("Hello  --  World"), "hello-world");
        assert_eq!(generate_slug("---Hello World---"), "hello-world");
    }

    #[test]
    fn test_generate_slug_edge_cases() {
        assert_eq!(generate_slug(""), "");
        assert_eq!(generate_slug("   "), "");
        assert_eq!(generate_slug("123"), "123");
        assert_eq!(generate_slug("---"), "");
    }

    #[test]
    fn test_generate_url_basic() {
        assert_eq!(
            generate_url("2025/01/15", "hello-world"),
            "/2025/01/15/hello-world"
        );

        assert_eq!(
            generate_url("2025/12/31", "year-end-post"),
            "/2025/12/31/year-end-post"
        );

        assert_eq!(
            generate_url("2024/07/04", "independence-day"),
            "/2024/07/04/independence-day"
        );
    }

    #[test]
    fn test_format_date_basic() {
        assert_eq!(transform_date_format("2025-01-15"), "2025/01/15");
        assert_eq!(transform_date_format("2025-12-31"), "2025/12/31");
        assert_eq!(transform_date_format("2024-07-04"), "2024/07/04");
    }

    #[test]
    fn test_format_date_single_digit_months_and_days() {
        assert_eq!(transform_date_format("2025-01-01"), "2025/01/01");
        assert_eq!(transform_date_format("2025-01-09"), "2025/01/09");
        assert_eq!(transform_date_format("2025-09-01"), "2025/09/01");
        assert_eq!(transform_date_format("2025-02-05"), "2025/02/05");
    }

    #[test]
    fn test_format_date_leap_year() {
        assert_eq!(transform_date_format("2024-02-29"), "2024/02/29");
        assert_eq!(transform_date_format("2020-02-29"), "2020/02/29");
    }

    #[test]
    fn test_format_date_edge_cases() {
        // New Year's Day
        assert_eq!(transform_date_format("2025-01-01"), "2025/01/01");

        // New Year's Eve
        assert_eq!(transform_date_format("2025-12-31"), "2025/12/31");

        // Different centuries
        assert_eq!(transform_date_format("1999-12-31"), "1999/12/31");
        assert_eq!(transform_date_format("2000-01-01"), "2000/01/01");
    }

    #[test]
    fn test_format_date_various_years() {
        assert_eq!(transform_date_format("1990-06-15"), "1990/06/15");
        assert_eq!(transform_date_format("2010-03-20"), "2010/03/20");
        assert_eq!(transform_date_format("2030-11-08"), "2030/11/08");
    }

    #[test]
    fn test_format_date_all_months() {
        assert_eq!(transform_date_format("2025-01-15"), "2025/01/15");
        assert_eq!(transform_date_format("2025-02-15"), "2025/02/15");
        assert_eq!(transform_date_format("2025-03-15"), "2025/03/15");
        assert_eq!(transform_date_format("2025-04-15"), "2025/04/15");
        assert_eq!(transform_date_format("2025-05-15"), "2025/05/15");
        assert_eq!(transform_date_format("2025-06-15"), "2025/06/15");
        assert_eq!(transform_date_format("2025-07-15"), "2025/07/15");
        assert_eq!(transform_date_format("2025-08-15"), "2025/08/15");
        assert_eq!(transform_date_format("2025-09-15"), "2025/09/15");
        assert_eq!(transform_date_format("2025-10-15"), "2025/10/15");
        assert_eq!(transform_date_format("2025-11-15"), "2025/11/15");
        assert_eq!(transform_date_format("2025-12-15"), "2025/12/15");
    }

    #[test]
    fn test_transform_date_format() {
        assert_eq!(transform_date_format("2025-01-15"), "2025/01/15");
        assert_eq!(transform_date_format("2024-12-31"), "2024/12/31");
        assert_eq!(transform_date_format("2020-02-29"), "2020/02/29");
        assert_eq!(transform_date_format("1999-01-01"), "1999/01/01");
    }

    #[test]
    fn test_generate_slug_unicode() {
        assert_eq!(generate_slug("Hello 世界"), "hello");
        assert_eq!(generate_slug("Café Blog Post"), "caf-blog-post");
        assert_eq!(generate_slug("Résumé Tips"), "rsum-tips");
        assert_eq!(generate_slug("naïve approach"), "nave-approach");
    }

    #[test]
    fn test_generate_slug_numbers_and_letters() {
        assert_eq!(generate_slug("API v2.0"), "api-v20");
        assert_eq!(generate_slug("HTTP/2 Performance"), "http2-performance");
        assert_eq!(generate_slug("Node.js vs Deno"), "nodejs-vs-deno");
        assert_eq!(generate_slug("100% Coverage"), "100-coverage");
    }

    #[test]
    fn test_transform_date_format_edge_cases() {
        // Test with empty string
        assert_eq!(transform_date_format(""), "");

        // Test with malformed dates (still transforms hyphens)
        assert_eq!(transform_date_format("2025-1-5"), "2025/1/5");
        assert_eq!(transform_date_format("25-12-31"), "25/12/31");

        // Test with extra hyphens
        assert_eq!(transform_date_format("2025--01--15"), "2025//01//15");
    }

    #[test]
    fn test_generate_url_edge_cases() {
        // Test with empty strings
        assert_eq!(generate_url("", ""), "//");
        assert_eq!(generate_url("2025/01/15", ""), "/2025/01/15/");
        assert_eq!(generate_url("", "test-slug"), "//test-slug");

        // Test with special characters in slug (should be pre-sanitized)
        assert_eq!(
            generate_url("2025/01/15", "test-slug"),
            "/2025/01/15/test-slug"
        );
    }

    // Mock test data creation helpers
    fn create_test_post_content(title: &str, date: &str, status: &str, author: &str) -> String {
        format!(
            r#"---
title: "{}"
date: {}
author: "{}"
tags: ["test", "blog"]
status: {}
excerpt: "Test excerpt"
---

# {}

This is test content for the post."#,
            title, date, author, status, title
        )
    }

    #[test]
    fn test_post_parsing_integration() {
        use gray_matter::engine::YAML;
        use gray_matter::Matter;

        let matter = Matter::<YAML>::new();
        let content =
            create_test_post_content("Test Post", "2025-01-15", "published", "Test Author");

        let parsed = matter.parse::<PostAttributes>(&content).unwrap();

        assert!(parsed.data.is_some());
        let post_data = parsed.data.unwrap();

        assert_eq!(post_data.title, "Test Post");
        assert_eq!(post_data.date, "2025-01-15");
        assert_eq!(post_data.author, "Test Author");
        assert_eq!(post_data.status, PostStatus::Published);
        assert_eq!(post_data.tags, vec!["test", "blog"]);
        assert_eq!(post_data.excerpt, Some("Test excerpt".to_string()));

        // Test the generated fields
        let slug = generate_slug(&post_data.title);
        let formatted_date = transform_date_format(&post_data.date);
        let url = generate_url(&formatted_date, &slug);

        assert_eq!(slug, "test-post");
        assert_eq!(formatted_date, "2025/01/15");
        assert_eq!(url, "/2025/01/15/test-post");
    }

    #[test]
    fn test_post_status_parsing() {
        use gray_matter::engine::YAML;
        use gray_matter::Matter;

        let matter = Matter::<YAML>::new();

        // Test published status
        let published_content =
            create_test_post_content("Published Post", "2025-01-15", "published", "Author");
        let parsed = matter.parse::<PostAttributes>(&published_content).unwrap();
        assert_eq!(parsed.data.unwrap().status, PostStatus::Published);

        // Test draft status
        let draft_content = create_test_post_content("Draft Post", "2025-01-15", "draft", "Author");
        let parsed = matter.parse::<PostAttributes>(&draft_content).unwrap();
        assert_eq!(parsed.data.unwrap().status, PostStatus::Draft);

        // Test scheduled status
        let scheduled_content =
            create_test_post_content("Scheduled Post", "2025-01-15", "scheduled", "Author");
        let parsed = matter.parse::<PostAttributes>(&scheduled_content).unwrap();
        assert_eq!(parsed.data.unwrap().status, PostStatus::Scheduled);
    }

    #[test]
    fn test_complex_slug_scenarios() {
        // Test with mixed case and numbers
        assert_eq!(generate_slug("iOS 17.1 Update"), "ios-171-update");

        // Test with parentheses and brackets
        assert_eq!(
            generate_slug("React (Hooks) vs Vue [Composition]"),
            "react-hooks-vs-vue-composition"
        );

        // Test with quotes and apostrophes
        assert_eq!(
            generate_slug("Don't Use 'var' in JavaScript"),
            "dont-use-var-in-javascript"
        );

        // Test with URL-like content
        assert_eq!(
            generate_slug("https://example.com/api"),
            "httpsexamplecomapi"
        );

        // Test with very long title
        let long_title = "This is a very long title that might be used in a blog post and should be converted to a reasonable slug without issues";
        let slug = generate_slug(long_title);
        assert!(slug.len() > 0);
        assert!(!slug.starts_with('-'));
        assert!(!slug.ends_with('-'));
        assert!(!slug.contains("  "));
    }
}
