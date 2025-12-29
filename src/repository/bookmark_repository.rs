use crate::models::{Bookmark, UpdateBookmarkRequest};
use futures::TryStreamExt;
use mongodb::{
    Collection, Database,
    bson::{doc, oid::ObjectId},
    options::FindOptions,
};

pub struct BookmarkRepository {
    collection: Collection<Bookmark>,
}

impl BookmarkRepository {
    pub fn new(db: Database) -> Self {
        Self {
            collection: db.collection("bookmarks"),
        }
    }

    pub async fn find_all(&self) -> Result<Vec<Bookmark>, mongodb::error::Error> {
        let options = FindOptions::builder()
            .sort(doc! { "created_at": -1 })
            .build();
        let cursor = self.collection.find(doc! {}).with_options(options).await?;
        cursor.try_collect().await
    }

    pub async fn find_by_id(
        &self,
        id: &ObjectId,
    ) -> Result<Option<Bookmark>, mongodb::error::Error> {
        self.collection.find_one(doc! { "_id": id }).await
    }

    pub async fn create(&self, mut bookmark: Bookmark) -> Result<Bookmark, mongodb::error::Error> {
        let result = self.collection.insert_one(&bookmark).await?;
        bookmark.id = result.inserted_id.as_object_id();
        Ok(bookmark)
    }

    pub async fn update(
        &self,
        id: &ObjectId,
        request: UpdateBookmarkRequest,
    ) -> Result<Option<Bookmark>, mongodb::error::Error> {
        let mut set_doc = doc! {};

        if let Some(url) = request.url {
            set_doc.insert("url", url);
        }
        if let Some(title) = request.title {
            set_doc.insert("title", title);
        }
        if let Some(tags) = request.tags {
            set_doc.insert("tags", tags);
        }
        if let Some(read) = request.read {
            set_doc.insert("read", read);
        }

        if set_doc.is_empty() {
            return self.find_by_id(id).await;
        }

        self.collection
            .find_one_and_update(doc! { "_id": id }, doc! { "$set": set_doc })
            .await
    }

    pub async fn delete(&self, id: &ObjectId) -> Result<bool, mongodb::error::Error> {
        let result = self.collection.delete_one(doc! { "_id": id }).await?;
        Ok(result.deleted_count > 0)
    }

    pub async fn search(
        &self,
        tag: Option<&str>,
        unread_only: bool,
        page: u32,
        size: u32,
    ) -> Result<Vec<Bookmark>, mongodb::error::Error> {
        let mut filter = doc! {};

        if let Some(t) = tag {
            filter.insert("tags", t);
        }

        if unread_only {
            filter.insert("read", false);
        }

        let options = FindOptions::builder()
            .skip(Some((page * size) as u64))
            .limit(Some(size as i64))
            .sort(doc! { "created_at": -1 })
            .build();

        let cursor = self.collection.find(filter).with_options(options).await?;
        cursor.try_collect().await
    }
}
