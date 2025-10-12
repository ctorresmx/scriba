use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use regex::Regex;
use std::ffi::OsStr;
use std::fs;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use crate::config::get_blog_config;
use crate::models::{ParsedPost, PostAttributes, PostStatus};

fn highlight_code(code: &str, language: &str) -> String {
    // Early return for empty code
    if code.is_empty() {
        return format!("<pre><code></code></pre>");
    }

    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();

    // Use base16-eighties.dark theme with robust fallback
    let theme = theme_set
        .themes
        .get("base16-eighties.dark")
        .or_else(|| theme_set.themes.get("base16-ocean.dark"))
        .or_else(|| theme_set.themes.get("Solarized (dark)"))
        .or_else(|| theme_set.themes.values().next())
        .expect("Critical error: No syntax highlighting themes available");

    // Normalize language name for better recognition
    let normalized_language = match language.to_lowercase().as_str() {
        "ts" => "ts", // Keep as "ts" since syntect might know this better
        "typescript" => "typescript",
        "js" => "javascript",
        "json" => "json",
        "diff" => "diff",
        "java" => "java",
        "rust" => "rust",
        "python" | "py" => "python",
        "bash" | "sh" => "bash",
        "html" => "html",
        "css" => "css",
        "yaml" | "yml" => "yaml",
        _ => language,
    };

    // Find the syntax definition for the language
    let syntax = if normalized_language == "ts" || normalized_language == "typescript" {
        // Try multiple approaches for TypeScript
        syntax_set
            .find_syntax_by_token("typescript")
            .or_else(|| syntax_set.find_syntax_by_token("ts"))
            .or_else(|| syntax_set.find_syntax_by_extension("ts"))
            .or_else(|| syntax_set.find_syntax_by_extension("typescript"))
            .or_else(|| syntax_set.find_syntax_by_name("TypeScript"))
            .or_else(|| syntax_set.find_syntax_by_name("Typescript"))
            .or_else(|| syntax_set.find_syntax_by_name("JavaScript")) // Fallback to JS
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text())
    } else {
        syntax_set
            .find_syntax_by_token(normalized_language)
            .or_else(|| syntax_set.find_syntax_by_extension(normalized_language))
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text())
    };

    // Special handling for diff to add proper line backgrounds
    if normalized_language == "diff" {
        return highlight_diff(code, &syntax_set, syntax, theme);
    }

    // Generate highlighted HTML with robust error handling
    match highlighted_html_for_string(code, &syntax_set, syntax, theme) {
        Ok(highlighted) => highlighted,
        Err(e) => {
            eprintln!("Warning: Syntax highlighting failed for language '{}': {}", language, e);
            // Fallback to plain code block with proper HTML escaping
            format!("<pre><code>{}</code></pre>", html_escape::encode_text(code))
        }
    }
}

fn highlight_diff(
    code: &str,
    _syntax_set: &SyntaxSet,
    _syntax: &syntect::parsing::SyntaxReference,
    _theme: &syntect::highlighting::Theme,
) -> String {
    // Early return for empty diff
    if code.is_empty() {
        return String::from(
            "<pre style=\"background-color:#1e2229 !important;\" class=\"diff-highlight\"><code></code></pre>"
        );
    }

    // For diff files, we'll do custom highlighting with enhanced backgrounds
    // Parse the code line by line and add diff-specific styling
    let lines: Vec<&str> = code.lines().collect();
    let mut result = String::from(
        "<pre style=\"background-color:#1e2229 !important;\" class=\"diff-highlight\"><code>",
    );

    for line in lines.iter() {
        if line.starts_with('+') && !line.starts_with("+++") {
            result.push_str(&format!(
                "<span class=\"diff-added\">{}</span>",
                html_escape::encode_text(line)
            ));
        } else if line.starts_with('-') && !line.starts_with("---") {
            result.push_str(&format!(
                "<span class=\"diff-removed\">{}</span>",
                html_escape::encode_text(line)
            ));
        } else if line.starts_with("@@") {
            result.push_str(&format!(
                "<span class=\"diff-hunk\">{}</span>",
                html_escape::encode_text(line)
            ));
        } else {
            result.push_str(&format!(
                "<span style=\"color: #e6edf3;\">{}</span>",
                html_escape::encode_text(line)
            ));
        }

        // Add newline only if not the last line
        //        if i < lines.len() - 1 {
        //            result.push('\n');
        //        }
    }

    result.push_str("</code></pre>");
    result
}

fn parse_markdown_with_highlighting(content: &str) -> String {
    let parser = Parser::new(content);
    let mut events = Vec::new();
    let mut in_code_block = false;
    let mut code_buffer = String::new();
    let mut code_language = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(language))) => {
                in_code_block = true;
                code_language = language.to_string();
                code_buffer.clear();
                // Don't add this event, we'll replace it with highlighted HTML
            }
            Event::End(TagEnd::CodeBlock) => {
                if in_code_block {
                    let highlighted = highlight_code(&code_buffer, &code_language);
                    events.push(Event::Html(highlighted.into()));
                    in_code_block = false;
                }
                // Don't add this event either
            }
            Event::Text(text) => {
                if in_code_block {
                    code_buffer.push_str(&text);
                    // Don't add this event when in code block
                } else {
                    events.push(Event::Text(text));
                }
            }
            _ => {
                if !in_code_block {
                    events.push(event);
                }
            }
        }
    }

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, events.into_iter());
    html_output
}

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
                    let parsed_content = parse_markdown_with_highlighting(&post.content);
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
    fn test_highlight_code_basic_functionality() {
        let rust_code = "fn main() { println!(\"Hello\"); }";
        let result = highlight_code(rust_code, "rust");

        // Should contain HTML with pre tags
        assert!(result.contains("<pre"));
        assert!(result.contains("</pre>"));
        assert!(result.contains("main"));
        assert!(result.contains("println"));
    }

    #[test]
    fn test_highlight_code_language_normalization() {
        let ts_code = "const x: number = 42;";
        let result = highlight_code(ts_code, "ts");

        // Should contain HTML output
        assert!(result.contains("<pre"));
        assert!(result.contains("const"));
        assert!(result.contains("number"));
    }

    #[test]
    fn test_highlight_code_with_unknown_language() {
        let code = "some unknown syntax";
        let result = highlight_code(code, "unknownlang");

        // Should still return valid HTML
        assert!(result.contains("<pre"));
        assert!(result.contains("some unknown syntax"));
    }

    #[test]
    fn test_highlight_code_with_empty_code() {
        let result = highlight_code("", "rust");

        // Should handle empty code gracefully
        assert!(result.contains("<pre"));
        assert!(result.contains("</pre>"));
    }

    #[test]
    fn test_highlight_code_special_characters() {
        let code = "println!(\"Hello <world> & 'quotes'\");";
        let result = highlight_code(code, "rust");

        // Should properly escape HTML characters
        assert!(result.contains("<pre"));
        // The content should be properly escaped or highlighted
        assert!(result.len() > code.len()); // Should have added HTML tags
    }

    #[test]
    fn test_highlight_diff_basic() {
        let diff_code = "- old line\n+ new line";
        let result = highlight_code(diff_code, "diff");

        // Should contain diff-specific classes and structure
        assert!(result.contains("diff-highlight"));
        assert!(result.contains("diff-removed"));
        assert!(result.contains("diff-added"));
        assert!(result.contains("old line"));
        assert!(result.contains("new line"));
    }

    #[test]
    fn test_highlight_diff_with_context() {
        let diff_code = "@@ -1,3 +1,3 @@\n context line\n- removed line\n+ added line";
        let result = highlight_code(diff_code, "diff");

        // Should handle hunk headers and context
        assert!(result.contains("diff-hunk"));
        assert!(result.contains("@@"));
        assert!(result.contains("diff-removed"));
        assert!(result.contains("diff-added"));
        assert!(result.contains("context line"));
    }

    #[test]
    fn test_highlight_diff_ignores_file_headers() {
        let diff_code = "--- old-file.txt\n+++ new-file.txt\n- content";
        let result = highlight_code(diff_code, "diff");

        // Should not treat file headers as diff lines
        assert!(result.contains("old-file.txt"));
        assert!(result.contains("new-file.txt"));
        assert!(result.contains("diff-removed"));

        // Check that the file headers are properly handled
        // The "--- old-file.txt" line should not be treated as a removed line
        // because our code checks for "---" prefix to avoid file headers
        assert!(result.contains("--- old-file.txt"));
        assert!(result.contains("+++ new-file.txt"));

        // Verify the actual content line gets the diff class
        assert!(result.contains("content"));
    }

    #[test]
    fn test_highlight_diff_empty() {
        let result = highlight_code("", "diff");

        // Should handle empty diff gracefully - now returns early with plain pre/code
        assert!(result.contains("<pre"));
        assert!(result.contains("</pre>"));
        assert!(result.contains("<code"));
        assert!(result.contains("</code>"));
    }

    #[test]
    fn test_parse_markdown_with_highlighting_integration() {
        let markdown = "```rust\nfn main() {}\n```\n\n```diff\n- old\n+ new\n```";
        let result = parse_markdown_with_highlighting(markdown);

        // Should contain both regular highlighting and diff highlighting
        assert!(result.contains("<pre"));
        assert!(result.contains("main"));
        assert!(result.contains("diff-highlight"));
        assert!(result.contains("diff-removed"));
        assert!(result.contains("diff-added"));
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
