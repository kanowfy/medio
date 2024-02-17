use std::error::Error;

use time_interface::{TimeSender, Time};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;
use wasmcloud_interface_messaging::{MessagingSender, Messaging, PubMessage};
use wasmcloud_interface_sqldb::*;

use crate::article::{CreateArticleRequest, ArticleDto, UpdateArticleRequest};
use minicbor::{decode, Encode, Decode};

const TABLE_NAME: &str = "articles";
const TOPIC: &str = "medio/newsletter";

#[derive(Encode, Decode)]
struct ArticleModel {
    #[n(0)]
    id: u64,
    #[n(1)]
    author_id: u64,
    #[n(2)]
    title: String,
    #[n(3)]
    content: String,
    #[n(4)]
    created_at: u64,
    #[n(5)]
    updated_at: u64
}

pub(crate) async fn get_all_articles(ctx: &Context) -> Result<Vec<ArticleDto>, Box<dyn Error>> {
    let resp = SqlDbSender::new().query(
        ctx,
        &format!(
            "SELECT id, author_id, title, content, created_at, updated_at FROM {} ORDER BY created_at DESC",
            TABLE_NAME
        ).into()
    ).await?;

    if resp.num_rows == 0 {
        return Ok(Vec::new());
    } else {
        let rows: Vec<ArticleModel> = decode(&resp.rows)?;
        let rows: Vec<ArticleDto> = rows.into_iter().map(|r| r.into()).collect();
        Ok(rows)
    }
}

pub(crate) async fn get_articles_for_author(ctx: &Context, author_id: &u64) -> Result<Vec<ArticleDto>, Box<dyn Error>> {
    let resp = SqlDbSender::new().query(
        ctx,
        &format!(
            "SELECT id, author_id, title, content, created_at, updated_at FROM {} WHERE author_id = {} ORDER BY created_at DESC",
            TABLE_NAME, author_id
        ).into()
    ).await?;

    if resp.num_rows == 0 {
        return Ok(Vec::new());
    } else {
        let rows: Vec<ArticleModel> = decode(&resp.rows)?;
        let rows: Vec<ArticleDto> = rows.into_iter().map(|r| r.into()).collect();
        Ok(rows)
    }

}

async fn get_one_article_model(ctx: &Context, id: &u64) -> Result<ArticleModel, SqlDbError> {
    let resp = SqlDbSender::new().query(
        ctx,
        &format!(
            "SELECT id, author_id, title, content, created_at, updated_at FROM {} WHERE id = {}",
            TABLE_NAME, id
        ).into()
    ).await?;

    if resp.num_rows == 0 {
        return Err(SqlDbError::new("notFound", "article id not found".to_string()))
    }

    let mut rows: Vec<ArticleModel> = decode(&resp.rows)?;
    Ok(rows.remove(0))
} 

pub(crate) async fn get_one_article(ctx: &Context, id: &u64) -> Result<ArticleDto, SqlDbError> {
    match get_one_article_model(ctx, id).await {
        Ok(article) => Ok(article.into()),
        Err(e) => Err(e),
    }
}

pub(crate) async fn create_article(ctx: &Context, article: &CreateArticleRequest) -> Result<ArticleDto, Box<dyn Error>>{
    // create unix timestamp now
    let now = get_current_time(ctx).await?;
    info!("Adding to db");
    let resp = SqlDbSender::new().query(
        ctx,
        &format!(
            "INSERT INTO {} (author_id, title, content, created_at, updated_at) VALUES ({}, '{}', '{}', {}, {}) RETURNING *",
            TABLE_NAME, article.author_id, article.title, article.content, now, now
        ).into()
    ).await?;
    info!("Add to db successfully");

    let mut rows: Vec<ArticleModel> = decode(&resp.rows)?;
    let inserted = rows.remove(0).into();

    info!("Publishing message");
    // currently, if the article is inserted into the database but message publishing fails, the article still
    // remains in the database and but we responds with an error
    //TODO: add transaction support for article creation and message publishing to be atomic
    if let Err(e) = publish_message(ctx, &inserted).await {
        return Err(format!("publish message failed: {:?}", e).into())
    }
    info!("Publish message successful");
    Ok(inserted)
}

pub(crate) async fn update_article(ctx: &Context, update: &UpdateArticleRequest) -> Result<(), Box<dyn Error>> {
    let article = match get_one_article_model(ctx, &update.target_id).await {
        Ok(article) => article,
        Err(e) => return Err(format!("{}", e).into())
    };

    let article = ArticleModel {
        id: article.id,
        author_id: article.author_id,
        title: update.title.clone().unwrap_or(article.title),
        content: update.content.clone().unwrap_or(article.content),
        created_at: article.created_at,
        updated_at: get_current_time(ctx).await?
    };

    let _resp = SqlDbSender::new().execute(
        ctx,
        &format!(
            "UPDATE {} SET title = '{}', content = '{}', updated_at = {} WHERE id = {}",
            TABLE_NAME, article.title, article.content, article.updated_at, article.id
        ).into()
    ).await?;

    Ok(())
}

pub(crate) async fn delete_article(ctx: &Context, id: &u64) -> Result<(), Box<dyn Error>> {
    let resp = SqlDbSender::new().execute(
        ctx,
        &format!("DELETE FROM {} WHERE id = {}",
        TABLE_NAME, id).into()
    ).await?;

    if resp.rows_affected != 0 {
        return Err("article not found".into())
    }

    Ok(())
}

async fn publish_message(ctx: &Context, article: &ArticleDto) -> RpcResult<()> {
    let provider = MessagingSender::new();
    let encoded = bincode::serialize(article).unwrap();
    provider.publish(
        ctx,
        &PubMessage {
            subject: TOPIC.to_string(),
            reply_to: None,
            body: encoded
        }
    ).await
}

async fn get_current_time(ctx: &Context) -> RpcResult<u64> {
    let provider = TimeSender::new();
    provider.now(ctx).await
}

impl From<ArticleModel> for ArticleDto {
    fn from(a: ArticleModel) -> ArticleDto {
        ArticleDto {
            id: a.id,
            author_id: a.author_id,
            title: a.title,
            content: a.content,
            created_at: a.created_at,
            updated_at: match a.updated_at {
                0 => None,
                t => Some(t)
            }
        }
    }
}