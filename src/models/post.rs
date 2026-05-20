use bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author: String,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
