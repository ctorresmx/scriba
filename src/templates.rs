use crate::models::ParsedPost;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub header_title: String,
    pub blog_title: String,
    pub copyright: String,
    pub current_year: String,
    pub posts: Vec<ParsedPost>,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub header_title: String,
    pub blog_title: String,
    pub copyright: String,
    pub current_year: String,
    pub post: ParsedPost,
}
