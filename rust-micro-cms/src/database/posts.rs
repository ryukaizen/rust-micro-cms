use crate::models::{NewPost, Post, Posts, UpdatePost};
use anyhow::Result;
use sqlx::sqlite::SqlitePool;

pub async fn create_posts_table(pool: &SqlitePool) -> Result<()> {
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            date TEXT NOT NULL,
            body TEXT NOT NULL,
            archived BOOLEAN NOT NULL DEFAULT FALSE,
            draft BOOLEAN NOT NULL DEFAULT TRUE,
            author_id INTEGER NOT NULL,
            FOREIGN KEY (author_id) REFERENCES author(id)
        )"
    )
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn insert_new_post(pool: &SqlitePool, post: &NewPost) -> Result<Post> {
    let inserted_post = sqlx::query_as!(
        Post,
        "INSERT INTO posts (title, date, body, author_id) VALUES (?, ?, ?, ?) RETURNING *",
        post.title,
        post.date,
        post.body,
        post.author_id
    )
    .fetch_one(pool)
    .await?;

    Ok(inserted_post)
}

pub async fn fetch_all_posts(pool: &SqlitePool) -> Result<Posts> {
    let posts = sqlx::query_as!(Post, "SELECT * FROM posts")
        .fetch_all(pool)
        .await?;

    Ok(Posts { posts })
}

pub async fn fetch_post_by_id(pool: &SqlitePool, post_id: i32) -> Result<Post> {
    let post = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1", post_id)
        .fetch_one(pool)
        .await?;

    Ok(post)
}

pub async fn fetch_all_posts_for_author(pool: &SqlitePool, author_id: i32) -> Result<Posts> {
    let posts = sqlx::query_as!(
        Post,
        "SELECT * FROM posts WHERE author_id = $1 AND archived = FALSE",
        author_id
    )
    .fetch_all(pool)
    .await?;

    Ok(Posts { posts })
}

pub async fn update_post(pool: &SqlitePool, post: &UpdatePost) -> Result<Post> {
    let updated_post = sqlx::query!(
        "UPDATE posts SET title = $1, date = $2, body = $3, archived = $4, draft = $5, author_id = $6 WHERE id = $7 RETURNING *",
        post.title,
        post.date,
        post.body,
        post.archived,
        post.draft,
        post.author_id,
        post.id
    )
    .fetch_one(pool)
    .await?;

    Ok(Post {
        id: updated_post.id.expect("Could not find id"),
        title: updated_post.title,
        date: updated_post.date,
        body: updated_post.body,
        archived: updated_post.archived,
        draft: updated_post.draft,
        author_id: updated_post.author_id,
    })
}

pub async fn delete_post(pool: &SqlitePool, post_id: i32) -> Result<()> {
    sqlx::query!("DELETE FROM posts WHERE id = $1", post_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn toggle_post_draft(pool: &SqlitePool, post_id: i32) -> Result<()> {
    let current_status = sqlx::query!("SELECT draft FROM posts WHERE id = $1", post_id)
        .fetch_one(pool)
        .await?
        .draft;

    let new_status = !current_status;
    sqlx::query!(
        "UPDATE posts SET draft = $1 WHERE id = $2",
        new_status,
        post_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn toggle_post_active(pool: &SqlitePool, post_id: i32) -> Result<()> {
    let current_status = sqlx::query! {
        "SELECT archived FROM posts WHERE id = $1",
        post_id
    }
    .fetch_one(pool)
    .await?
    .archived;

    let new_status = !current_status;
    sqlx::query!(
        "UPDATE posts SET archived = $1 WHERE id = $2",
        new_status,
        post_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
