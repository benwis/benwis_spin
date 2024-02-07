use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::errors::BenwisAppError;
    use std::sync::Arc;
    use spin_sdk::sqlite::{Connection, Value};
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostFrontmatter {
    title: String,
    slug: String,
    hero: Option<String>,
    hero_alt: Option<String>,
    hero_caption: Option<String>,
    excerpt: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    published: bool,
    preview: bool,
    tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawPost {
    pub slug: String,
    pub content: String,
    pub frontmatter: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub published: bool,
    pub preview: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub slug: String,
    pub hero: Option<String>,
    pub hero_caption: Option<String>,
    pub hero_alt: Option<String>,
    pub excerpt: Option<String>,
    pub content: String,
    pub toc: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published: bool,
    pub preview: bool,
    pub tags: Vec<String>,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    impl Post {
    pub fn get_posts(con: &Arc<Connection>) -> Result<Vec<Post>, BenwisAppError>{
        let rowset = con.execute("SELECT * from posts", &[]).map_err( |_| BenwisAppError::InternalServerError)?;
        let mut posts:Vec<Post> = rowset.rows().map(|row| {
        Post{
            slug: row.get::<&str>("slug").unwrap().to_owned(),
            title: row.get::<&str>("title").unwrap().to_owned(),
            excerpt: row.get::<&str>("title").map(str::to_string),
            toc: row.get::<&str>("toc").map(str::to_string),
            content: row.get::<&str>("content").unwrap().to_owned(),
            created_at: DateTime::from_timestamp(row.get::<i64>("created_at").unwrap_or(0), 0).expect("Failed to create time"),
            updated_at: DateTime::from_timestamp(row.get::<i64>("updated_at").unwrap_or(0), 0).expect("Failed to create time"),
            hero: row.get::<&str>("hero").map(str::to_string),
            hero_alt: row.get::<&str>("hero_alt").map(str::to_string),
            hero_caption: row.get::<&str>("hero_caption").map(str::to_string),
            tags: serde_json::from_str(row.get::<&str>("tags").unwrap_or_default()).unwrap_or_default(),
            preview: row.get::<bool>("preview").unwrap_or_default(),
            published: row.get::<bool>("published").unwrap_or_default(),
        }
        }).collect();

        posts.sort_unstable_by(|a, b| b.created_at.partial_cmp(&a.created_at).unwrap());
        Ok(posts)
    }
    pub fn get_post(slug: &str, con: &Arc<Connection>) -> Result<Option<Post>, BenwisAppError>{
        let rowset = con.execute("SELECT * FROM posts WHERE slug = ? ", &[Value::Text(slug.to_owned())]).map_err(|_| BenwisAppError::NotFound)?;
        let post = rowset.rows().nth(0).map(|row| {
        Post{
            slug: row.get::<&str>("slug").unwrap().to_owned(),
            title: row.get::<&str>("title").unwrap().to_owned(),
            excerpt: row.get::<&str>("title").map(str::to_string),
            toc: row.get::<&str>("toc").map(str::to_string),
            content: row.get::<&str>("content").unwrap().to_owned(),
            created_at: DateTime::from_timestamp(row.get::<i64>("created_at").unwrap_or(0), 0).expect("Failed to create time"),
            updated_at: DateTime::from_timestamp(row.get::<i64>("updated_at").unwrap_or(0), 0).expect("Failed to create time"),
            hero: row.get::<&str>("hero").map(str::to_string),
            hero_alt: row.get::<&str>("hero_alt").map(str::to_string),
            hero_caption: row.get::<&str>("hero_caption").map(str::to_string),
            tags: serde_json::from_str(row.get::<&str>("tags").unwrap_or_default()).unwrap_or_default(),
            preview: row.get::<bool>("preview").unwrap_or_default(),
            published: row.get::<bool>("published").unwrap_or_default(),
        }
       });
       Ok(post)
    }
pub async fn add_post(  title: String,
    slug: String,
    created_at_pretty: String,
    excerpt: String,
    content: String,
    published: String,
    preview: String,
    con: &Arc<Connection>) -> Result<bool, BenwisAppError>{

    let published = published.parse::<bool>().unwrap();
    let preview = preview.parse::<bool>().unwrap();

    let user = super::user::get_user(cx).await?;
    let slug = match slug.is_empty() {
        true => slugify(&title),
        false => slug,
    };
}
}
}
    }
