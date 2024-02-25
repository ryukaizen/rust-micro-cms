use crate::db::DBConnection;
use crate::models::{Author, Authors};
use std::sync::Arc;
use anyhow::{Result, Error};

pub struct AuthorService {
    db: Arc<DBConnection>,
}

impl AuthorService {
    pub async fn create_author(&self, payload: &Author) -> Result<Author, Error> {
        self.db.insert_new_author(payload).await
    }

    pub async fn get_all_authors(&self) -> Result<Authors, Error> {
        self.db.fetch_all_authors().await
    }

    pub async fn get_author_by_id(&self, author_id: i32) -> Result<Option<Author>, Error> {
        self.db.fetch_author_by_id(author_id).await
    }
}
