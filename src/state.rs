use crate::repository::BookmarkRepository;
use crate::service::BookmarkService;
use mongodb::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub bookmark_service: Arc<BookmarkService>,
}

impl AppState {
    pub async fn new(mongodb_uri: &str) -> Result<Self, mongodb::error::Error> {
        let client = Client::with_uri_str(mongodb_uri).await?;
        let db = client.database("bookmarks_db");

        let repository = Arc::new(BookmarkRepository::new(db));
        let service = Arc::new(BookmarkService::new(repository));

        Ok(Self {
            bookmark_service: service,
        })
    }
}
