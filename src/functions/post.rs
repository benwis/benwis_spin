use crate::{errors::BenwisAppError, models::NewPost};
use crate::models::Post;
use cfg_if::cfg_if;
use chrono::Duration;
use indexmap::IndexMap;
use leptos::{expect_context, ServerFnError};
use leptos_spin_macro::server;
cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::functions::con;
    use leptos_spin::{redirect, ResponseOptions};
    use std::sync::Arc;
    use serde_json::json;
    use spin_sdk::sqlite::Connection;
    use chrono::prelude::*;
}}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(GetPosts, "/api", "GetJson")]
pub async fn get_posts(num: Option<usize>) -> Result<Vec<Post>, ServerFnError<BenwisAppError>> {
    let con = con()?;
    let posts = Post::get_posts(&con)?;

    // Set Cache-Control headers
    let res = expect_context::<ResponseOptions>();
    res.append_header("Cache-Control", "private, max-age=3600".as_bytes());
    Ok(posts)
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(GetPost, "/api", "GetJson")]
pub async fn get_post(slug: String) -> Result<Option<Post>, ServerFnError<BenwisAppError>> {
    let con = con()?;

    let post = Post::get_post(&slug, &con)?;
    // Set Cache-Control headers
    let res = expect_context::<ResponseOptions>();
    res.append_header("Cache-Control", "private, max-age=3600".as_bytes());
    Ok(post)
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(AddPost, "/api")]
pub async fn add_post(slug: String, title: String, author_id: String, excerpt: String, content: String, hero: String, hero_alt: String, hero_caption: String, tags: String,  preview: String, published: String) -> Result<bool, ServerFnError<BenwisAppError>> {
    let con = con()?;

    let hero = match hero.is_empty(){
        true => None,
        false => Some(hero),
    };
    let hero_alt = match hero_alt.is_empty(){
        true => None,
        false => Some(hero_alt),
    };
    let hero_caption = match hero_caption.is_empty(){
        true => None,
        false => Some(hero_caption),
    };

    let excerpt = match excerpt.is_empty(){
        true => None,
        false => Some(excerpt),
    };
    let author_id = author_id.parse().map_err(|_| BenwisAppError::BadRequest("Invalid Author ID".to_string()))?;
    let tags = serde_json::from_str(&tags).map_err(|e| BenwisAppError::BadRequest(e.to_string()))?;

    let preview: bool = match preview.parse::<i32>().map_err(|e| BenwisAppError::BadRequest(e.to_string()))?{
    0 => false,
    1 => true,
    _ => return Err(BenwisAppError::BadRequest("Invalid number for bool conversion".to_string()).into())
    };
    let published: bool = match published.parse::<i32>().map_err(|e| BenwisAppError::BadRequest(e.to_string()))?{
    0 => false,
    1 => true,
    _ => return Err(BenwisAppError::BadRequest("Invalid number for bool conversion".to_string()).into())
    };
    let new_post = NewPost{
        slug,
        title,
        excerpt, 
        content, 
        created_at: Default::default(),
        updated_at: Default::default(),
        author_id, 
        preview, 
        published: published.into(), 
        hero, 
        hero_caption, 
        hero_alt, 
        tags, 
    };
    // Get user id from session cookie


    let post = Post::add_post(new_post, &con).await?;
    Ok(post)
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(UpdatePost, "/api")]
pub async fn update_post(id: i64, slug: String, title: String, author_id: String, excerpt: String, content: String, hero: String, hero_alt: String, hero_caption: String,toc: String,created_at: String, updated_at: String,  tags: String,  preview: String, published: String) -> Result<(), ServerFnError<BenwisAppError>> {
    let con = con()?;

    let hero = match hero.is_empty(){
        true => None,
        false => Some(hero),
    };
    let hero_alt = match hero_alt.is_empty(){
        true => None,
        false => Some(hero_alt),
    };
    let hero_caption = match hero_caption.is_empty(){
        true => None,
        false => Some(hero_caption),
    };

    let excerpt = match excerpt.is_empty(){
        true => None,
        false => Some(excerpt),
    };
    let author_id = author_id.parse().map_err(|_| BenwisAppError::BadRequest("Invalid Author ID".to_string()))?;
    let tags = serde_json::from_str(&tags).map_err(|e| BenwisAppError::BadRequest(e.to_string()))?;

    let toc = match toc.is_empty(){
        true => None,
        false => Some(toc),
    };
    let preview: bool = match preview.parse::<i32>().map_err(|e| BenwisAppError::BadRequest(e.to_string()))?{
    0 => false,
    1 => true,
    _ => return Err(BenwisAppError::BadRequest("Invalid number for bool conversion".to_string()).into())
    };
    let published: bool = match published.parse::<i32>().map_err(|e| BenwisAppError::BadRequest(e.to_string()))?{
    0 => false,
    1 => true,
    _ => return Err(BenwisAppError::BadRequest("Invalid number for bool conversion".to_string()).into())
    };
    let created_at: DateTime<Utc> = match DateTime::parse_from_rfc3339(&created_at){
        Ok(d) => d.into(),
        Err(e) => return Err(BenwisAppError::BadRequest("Invalid Date Format. Use RFC3339".to_string()).into())
    }; 

    let updated_at: DateTime<Utc> = match DateTime::parse_from_rfc3339(&updated_at){
        Ok(d) => d.into(),
        Err(e) => return Err(BenwisAppError::BadRequest("Invalid Date Format. Use RFC3339".to_string()).into())
    }; 
    let updated_post = Post{
    id,
    title,
    slug,
    author_id,
    excerpt,
    content,
    hero,
    hero_alt,
    hero_caption,
    tags,
    preview,
    published, 
    toc, 
    created_at, 
    updated_at,
    };

    let post = Post::update_post(updated_post, &con).await?;
    Ok(())
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(DeletePost, "/api")]
pub async fn delete_post(id: i64) -> Result<(), ServerFnError<BenwisAppError>> {
    let con = con()?;

    let post = Post::delete_post(id, &con).await?;
    Ok(())
}
