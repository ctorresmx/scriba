#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostAttributes {
    title: String,
    date: chrono::DateTime<chrono::Utc>,
    author: String,
    tags: Vec<String>,
    status: PostStatus,
    excerpt: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostStatus {
    Draft,
    Published,
    Scheduled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedPost {
    attributes: PostAttributes,
    content: String,
    slug: String,
    url: String,            // Generated URL like YYYY/MM/DD slug
    formatted_date: String, // Formatted date like YYYY/MM/DD
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlogConfig {
    name: String,
    title: String,
    copyright: String,
    posts_dir: String,
    favicon_text: String,
}
