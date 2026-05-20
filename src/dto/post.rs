use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub published: Option<bool>,
}
