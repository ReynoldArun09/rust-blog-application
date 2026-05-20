use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CommentRequest {
    pub content: String,
}
