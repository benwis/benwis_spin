
use cfg_if::cfg_if;
use chrono::Duration;
use crate::errors::BenwisAppError;
use indexmap::IndexMap;
use leptos::*;
use crate::models::Post;
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
    res.append_header(
        "Cache-Control",
        "private, max-age=3600".as_bytes(),
    );
    Ok(posts)
}

#[tracing::instrument(level = "info", fields(error), err)]
#[server(GetPost, "/api", "GetJson")]
pub async fn get_post(slug: String) -> Result<Option<Post>, ServerFnError<BenwisAppError>> {
    let con = con()?;

    let post = Post::get_post(&slug, &con)?;
    // Set Cache-Control headers
    let res = expect_context::<ResponseOptions>();
    res.append_header(
        "Cache-Control",
        "private, max-age=3600".as_bytes(),
    );
    Ok(post)
}
