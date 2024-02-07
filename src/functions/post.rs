use crate::errors::BenwisAppError;
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
pub async fn add_post(slug: String) -> Result<bool, ServerFnError<BenwisAppError>> {
    let con = con()?;

    let post = Post::add_post(&slug, &con)?;
    Ok(post)
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(UpdatePost, "/api")]
pub async fn update_post(slug: String) -> Result<bool, ServerFnError<BenwisAppError>> {
    let con = con()?;

    let post = Post::update_post(&slug, &con)?;
    Ok(post)
}
