use crate::models::ParsedPost;
use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate {
    pub title: String,
    pub copyright: String,
    pub current_year: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub title: String,
    pub copyright: String,
    pub current_year: String,
    pub posts: Vec<ParsedPost>,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub title: String,
    pub copyright: String,
    pub current_year: String,
    pub post: ParsedPost,
}
