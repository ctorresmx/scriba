use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PostAttributes {
    pub title: String,
    pub date: String,
    pub author: String,
    pub tags: Vec<String>,
    pub status: PostStatus,
    pub excerpt: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PostStatus {
    Draft,
    Published,
    Scheduled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedPost {
    pub attributes: PostAttributes,
    pub content: String,
    pub slug: String,
    pub url: String,            // Generated URL like YYYY/MM/DD slug
    pub formatted_date: String, // Formatted date like YYYY/MM/DD
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlogConfig {
    pub name: String,
    pub title: String,
    pub copyright: String,
    pub posts_dir: String,
    pub favicon_text: String,
}
