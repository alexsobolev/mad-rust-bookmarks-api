use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bookmark {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub url: String,
    pub title: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub read: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBookmarkRequest {
    #[validate(url(message = "Must be a valid URL"))]
    #[validate(length(min = 1, message = "URL is required"))]
    pub url: String,

    #[validate(length(min = 1, max = 200, message = "Title must be 1-200 characters"))]
    pub title: String,

    #[validate(length(max = 10, message = "Maximum 10 tags allowed"))]
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateBookmarkRequest {
    #[validate(url(message = "Must be a valid URL"))]
    pub url: Option<String>,

    #[validate(length(min = 1, max = 200, message = "Title must be 1-200 characters"))]
    pub title: Option<String>,

    #[validate(length(max = 10, message = "Maximum 10 tags allowed"))]
    pub tags: Option<Vec<String>>,

    pub read: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub tag: Option<String>,
    #[serde(default)]
    pub unread_only: bool,
    #[serde(default)]
    pub page: u32,
    #[serde(default = "default_size")]
    pub size: u32,
}

fn default_size() -> u32 {
    20
}
