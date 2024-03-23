use crate::{errors::BenwisAppError, models::NewPost};
use crate::models::Post;
use cfg_if::cfg_if;
use chrono::Duration;
use indexmap::IndexMap;
use leptos::{expect_context, ServerFnError};
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, DisplayFromStr};
use super::user::get_user;
use chrono::{DateTime, Utc};
use leptos::server;

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


#[serde_as]
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
struct AddPostParams {
    slug: String,
    title: String,
    #[serde_as(as = "DisplayFromStr")]
    author_id: i64,
    created_at: DateTime<Utc>,
    #[serde(default)]
    excerpt: Option<String>,
    raw_content: String,
    content: String,
    #[serde(default)]
    toc: Option<String>,
    #[serde(default)]
    hero: Option<String>,
    #[serde(default)]
    hero_alt: Option<String>,
    #[serde(default)]
    hero_caption: Option<String>,
    tags: Vec<String>,
    #[serde_as(as = "DisplayFromStr")]
    preview: bool,
    #[serde_as(as = "DisplayFromStr")]
    published: bool,


}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(AddPost, "/api")]
pub async fn add_post(slug: String, title: String, author_id: String, created_at_pretty: String, excerpt: String, raw_content: String, content: String,toc: String, hero: String, hero_alt: String, hero_caption: String, tags: String,  preview: String, published: String) -> Result<bool, ServerFnError<BenwisAppError>> {
    
//pub async fn add_post(post: AddPostParams) -> Result<bool, ServerFnError<BenwisAppError>> {
    let con = con()?;


    // Check if User is logged in
    let user = match get_user().await {
    Ok(Some(u)) => u,
    Ok(None) => return Err(BenwisAppError::AuthError.into()),
    Err(e) => return Err(BenwisAppError::ServerError(e.to_string()).into()),
    };

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

    let toc = match toc.is_empty(){
        true => None,
        false => Some(toc),
    };
    let excerpt = match excerpt.is_empty(){
        true => None,
        false => Some(excerpt),
    };
    let author_id = author_id.parse().map_err(|_| BenwisAppError::BadRequest("Invalid Author ID".to_string()))?;
    let processed_tags: Vec<String> = {
        if tags.is_empty(){
            Default::default()
        } 
        else{
            serde_json::from_str(&tags).map_err(|e| BenwisAppError::BadRequest(e.to_string()))?
        }
    };
    let preview: bool = match preview.as_ref(){
    "true" => true,
    "false" => false,
    _ => return Err(BenwisAppError::BadRequest("Invalid string for bool conversion".to_string()).into())
    };

    let published: bool = match published.as_ref(){
    "true" => true,
    "false" => false,
    _ => return Err(BenwisAppError::BadRequest("Invalid string for bool conversion".to_string()).into())
    };
    println!("CREATED AT: {created_at_pretty}");
    let created_at: DateTime<Utc> = {
        if !created_at_pretty.is_empty() {
            match DateTime::parse_from_rfc3339(&created_at_pretty){
                Ok(d) => d.into(),
                Err(e) => return Err(BenwisAppError::BadRequest("Invalid Date Format. Use RFC3339".to_string()).into())
            }
        } else{
            Utc::now()
        }
    }; 

    //let new_post = NewPost{
    //    slug: post.slug,
    //    toc: post.toc,
    //    title: post.title,
    //    excerpt: post.excerpt, 
    //    raw_content: post.raw_content,
    //    content: post.content, 
    //    created_at: post.created_at.timestamp(),
    //    updated_at: post.created_at.timestamp(),
    //    author_id: post.author_id, 
    //    preview: post.preview, 
    //    published: post.published, 
    //    hero: post.hero, 
    //    hero_caption: post.hero_caption, 
    //    hero_alt: post.hero_alt, 
    //    tags: post.tags, 
    //};
    let new_post = NewPost{
        slug,
        toc,
        title,
        excerpt, 
        raw_content,
        content, 
        created_at: created_at.timestamp(),
        updated_at: created_at.timestamp(),
        author_id, 
        preview, 
        published, 
        hero, 
        hero_caption, 
        hero_alt, 
        tags:processed_tags, 
    };
    //TODO: Get user id from session cookie


    let post = Post::add_post(new_post, &con).await?;
    Ok(post)
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(UpdatePost, "/api")]
pub async fn update_post(id: i64, slug: String, title: String, author_id: String, excerpt: String, raw_content: String, content: String, hero: String, hero_alt: String, hero_caption: String,toc: String,created_at_pretty: String, updated_at_pretty: String,  tags: String,  preview: String, published: String) -> Result<(), ServerFnError<BenwisAppError>> {
    let con = con()?;


    // Check if User is logged in
    let user = match get_user().await {
    Ok(Some(u)) => u,
    Ok(None) => return Err(BenwisAppError::AuthError.into()),
    Err(e) => return Err(BenwisAppError::ServerError(e.to_string()).into()),
    };

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
    
    let processed_tags: Vec<String> = {
        if tags.is_empty(){
            Default::default()
        } 
        else{
            serde_json::from_str(&tags).map_err(|e| BenwisAppError::BadRequest(e.to_string()))?
        }
    };
    
    let toc = match toc.is_empty(){
        true => None,
        false => Some(toc),
    };

    let preview: bool = match preview.as_ref(){
    "true" => true,
    "false" => false,
    _ => return Err(BenwisAppError::BadRequest("Invalid string for bool conversion".to_string()).into())
    };

    let published: bool = match published.as_ref(){
    "true" => true,
    "false" => false,
    _ => return Err(BenwisAppError::BadRequest("Invalid string for bool conversion".to_string()).into())
    };
    let created_at: DateTime<Utc> = match DateTime::parse_from_rfc3339(&created_at_pretty){
        Ok(d) => d.into(),
        Err(e) => return Err(BenwisAppError::BadRequest("Invalid Date Format. Use RFC3339".to_string()).into())
    }; 

    let updated_at: DateTime<Utc> = match DateTime::parse_from_rfc3339(&updated_at_pretty){
        Ok(d) => d.into(),
        Err(e) => return Err(BenwisAppError::BadRequest("Invalid Date Format. Use RFC3339".to_string()).into())
    }; 
    let updated_post = Post{
    id,
    title,
    slug,
    author_id,
    excerpt,
    raw_content,
    content,
    hero,
    hero_alt,
    hero_caption,
    tags: processed_tags,
    preview,
    published, 
    toc, 
    created_at, 
    updated_at,
    };

    Post::update_post(updated_post, &con).await?;
    Ok(())
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(DeletePost, "/api")]
pub async fn delete_post(id: i64) -> Result<(), ServerFnError<BenwisAppError>> {
    let con = con()?;

    // Check if User is logged in
    let user = match get_user().await {
    Ok(Some(u)) => u,
    Ok(None) => return Err(BenwisAppError::AuthError.into()),
    Err(e) => return Err(BenwisAppError::ServerError(e.to_string()).into()),
    };
    Post::delete_post(id, &con).await?;
    Ok(())
}
