use crate::error::AppError;
use crate::models::{Bookmark, CreateBookmarkRequest, UpdateBookmarkRequest};
use crate::repository::BookmarkRepository;
use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

pub struct BookmarkService {
    repository: Arc<BookmarkRepository>,
}

impl BookmarkService {
    pub fn new(repository: Arc<BookmarkRepository>) -> Self {
        Self { repository }
    }

    pub async fn list(
        &self,
        tag: Option<&str>,
        unread_only: bool,
        page: u32,
        size: u32,
    ) -> Result<Vec<Bookmark>, AppError> {
        self.repository
            .search(tag, unread_only, page, size)
            .await
            .map_err(Into::into)
    }

    pub async fn get(&self, id: &str) -> Result<Bookmark, AppError> {
        let oid = ObjectId::parse_str(id).map_err(|_| AppError::InvalidId)?;
        self.repository
            .find_by_id(&oid)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn create(&self, request: CreateBookmarkRequest) -> Result<Bookmark, AppError> {
        let bookmark = Bookmark {
            id: None,
            url: request.url,
            title: request.title,
            tags: request.tags,
            created_at: Utc::now(),
            read: false,
        };
        self.repository.create(bookmark).await.map_err(Into::into)
    }

    pub async fn update(
        &self,
        id: &str,
        request: UpdateBookmarkRequest,
    ) -> Result<Bookmark, AppError> {
        let oid = ObjectId::parse_str(id).map_err(|_| AppError::InvalidId)?;
        self.repository
            .update(&oid, request)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let oid = ObjectId::parse_str(id).map_err(|_| AppError::InvalidId)?;
        let deleted = self.repository.delete(&oid).await?;
        if deleted {
            Ok(())
        } else {
            Err(AppError::NotFound)
        }
    }
}
