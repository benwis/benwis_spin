use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::errors::BenwisAppError;
    use std::sync::Arc;
    use spin_sdk::sqlite::{Connection, Value::{self, Text, Integer,Null}};
use slug::slugify;
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
pub struct NewPost {
    pub title: String,
    pub slug: String,
    pub author_id: i64,
    pub hero: Option<String>,
    pub hero_caption: Option<String>,
    pub toc: Option<String>,
    pub hero_alt: Option<String>,
    pub excerpt: Option<String>,
    pub raw_content: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub published: bool,
    pub preview: bool,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostTriad {
    pub previous: Option<Post>,
    pub post: Post,
    pub next: Option<Post>,
}

impl PostTriad {
    pub fn from_triad_posts(input: Vec<TriadPost>) -> PostTriad {
        let mut post = None;
        let mut previous = None;
        let mut next = None;
        for tp in input {
            if tp.which == "current" {
                post = Some(tp.into())
            } else if tp.which == "previous" {
                previous = Some(tp.into())
            } else if tp.which == "next" {
                next = Some(tp.into())
            } else {
                panic!("Impossible post triad");
            }
        }
        PostTriad {
            next,
            post: post.unwrap(),
            previous,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub author_id: i64,
    pub hero: Option<String>,
    pub hero_caption: Option<String>,
    pub hero_alt: Option<String>,
    pub excerpt: Option<String>,
    pub raw_content: String,
    pub content: String,
    pub toc: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published: bool,
    pub preview: bool,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriadPost {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub author_id: i64,
    pub hero: Option<String>,
    pub hero_caption: Option<String>,
    pub hero_alt: Option<String>,
    pub excerpt: Option<String>,
    pub raw_content: String,
    pub content: String,
    pub toc: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published: bool,
    pub preview: bool,
    pub tags: Vec<String>,
    pub which: String,
}
impl From<TriadPost> for Post {
    fn from(tp: TriadPost) -> Self {
        Self {
            id: tp.id,

            title: tp.title,
            slug: tp.slug,
            author_id: tp.author_id,
            hero: tp.hero,
            hero_caption: tp.hero_caption,
            hero_alt: tp.hero_alt,
            excerpt: tp.excerpt,
            raw_content: tp.raw_content,
            content: tp.content,
            toc: tp.toc,
            created_at: tp.created_at,
            updated_at: tp.updated_at,
            published: tp.published,
            preview: tp.preview,
            tags: tp.tags,
        }
    }
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    impl Post {
    pub fn get_posts(con: &Arc<Connection>) -> Result<Vec<Post>, BenwisAppError>{
        let rowset = con.execute("SELECT * from posts ORDER BY created_at DESC", &[]).map_err( |_| BenwisAppError::InternalServerError)?;
        let posts:Vec<Post> = rowset.rows().map(|row| {
        Post{

            id: row.get::<i64>("id").unwrap().to_owned(),
            author_id: row.get::<i64>("author_id").unwrap().to_owned(),
            slug: row.get::<&str>("slug").unwrap().to_owned(),
            title: row.get::<&str>("title").unwrap().to_owned(),
            excerpt: row.get::<&str>("excerpt").map(str::to_string),
            toc: row.get::<&str>("toc").map(str::to_string),
            raw_content: row.get::<&str>("raw_content").unwrap().to_owned(),
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

        //posts.sort_unstable_by(|a, b| b.created_at.partial_cmp(&a.created_at).unwrap());
        Ok(posts)
    }

    pub fn get_post_with_siblings(slug: &str, con: &Arc<Connection>) -> Result<Option<PostTriad>, BenwisAppError>{
        let rowset = con.execute("WITH ordered_posts AS (SELECT ROW_NUMBER() OVER (ORDER BY created_at DESC) as rn, * FROM posts),
     first_post AS (SELECT * FROM ordered_posts WHERE slug = ?)
SELECT 'current' as which, *
FROM first_post
UNION
SELECT 'previous', *
FROM ordered_posts
WHERE rn = (SELECT rn + 1 FROM first_post)
UNION
SELECT 'next', *
FROM ordered_posts
WHERE rn = (SELECT rn - 1 FROM first_post);", &[Value::Text(slug.to_owned())]).map_err( |_| BenwisAppError::InternalServerError)?;
        let posts: Vec<TriadPost> = rowset.rows().map(|row| {
        TriadPost{

            id: row.get::<i64>("id").unwrap().to_owned(),
            author_id: row.get::<i64>("author_id").unwrap().to_owned(),
            slug: row.get::<&str>("slug").unwrap().to_owned(),
            which: row.get::<&str>("which").unwrap().to_owned(),
            title: row.get::<&str>("title").unwrap().to_owned(),
            excerpt: row.get::<&str>("excerpt").map(str::to_string),
            toc: row.get::<&str>("toc").map(str::to_string),
            raw_content: row.get::<&str>("raw_content").unwrap().to_owned(),
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
        Ok(Some(PostTriad::from_triad_posts(posts)))
    }
    pub fn get_post(slug: &str, con: &Arc<Connection>) -> Result<Option<Post>, BenwisAppError>{
        let rowset = con.execute("SELECT * FROM posts WHERE slug = ? ", &[Value::Text(slug.to_owned())]).map_err(|_| BenwisAppError::NotFound)?;
        let post = rowset.rows().nth(0).map(|row| {
        Post{
            id: row.get::<i64>("id").unwrap().to_owned(),
            author_id: row.get::<i64>("author_id").unwrap().to_owned(),
            slug: row.get::<&str>("slug").unwrap().to_owned(),
            title: row.get::<&str>("title").unwrap().to_owned(),
            excerpt: row.get::<&str>("excerpt").map(str::to_string),
            toc: row.get::<&str>("toc").map(str::to_string),
            raw_content: row.get::<&str>("raw_content").unwrap().to_owned(),
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

    pub fn get_post_by_id(id: i64, con: &Arc<Connection>) -> Result<Option<Post>, BenwisAppError>{
        let rowset = con.execute("SELECT * FROM posts WHERE id = ? ", &[Value::Integer(id)]).map_err(|_| BenwisAppError::NotFound)?;
        let post = rowset.rows().nth(0).map(|row| {
        Post{
            id: row.get::<i64>("id").unwrap().to_owned(),
            author_id: row.get::<i64>("author_id").unwrap().to_owned(),
            slug: row.get::<&str>("slug").unwrap().to_owned(),
            title: row.get::<&str>("title").unwrap().to_owned(),
            excerpt: row.get::<&str>("excerpt").map(str::to_string),
            toc: row.get::<&str>("toc").map(str::to_string),
            raw_content: row.get::<&str>("raw_content").unwrap().to_owned(),
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
pub async fn add_post(
    post: NewPost,
    con: &Arc<Connection>) -> Result<bool, BenwisAppError>{

    // Forms send empty values as empty strings
    let slug = match post.slug.is_empty() {
        true => slugify(&post.title),
        false => post.slug,
    };

    let excerpt = match post.excerpt{
    Some(e) => Text(e),
    None => Null,
    };

    let hero = match post.hero{
    Some(e) => Text(e),
    None => Null,
    };

    let hero_alt = match post.hero_alt{
    Some(e) => Text(e),
    None => Null,
    };

    let hero_caption = match post.hero_caption{
    Some(e) => Text(e),
    None => Null,
    };

    let toc = match post.toc{
    Some(e) => Text(e),
    None => Null,
    };

    let Ok(tags) = serde_json::to_string(&post.tags) else{
    return Err(BenwisAppError::JsonError("Failed to serialize tags".to_string()));
    };


    con.execute("INSERT INTO posts (title, slug, excerpt, toc, raw_content, content, published, preview, author_id, hero, hero_alt, hero_caption, tags, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", &[Text(post.title), Text(slug), excerpt,toc, Text(post.raw_content), Text(post.content), Integer(post.published.into()), Integer(post.preview.into()), Integer(post.author_id), hero, hero_alt, hero_caption, Text(tags), Integer(post.created_at)])?;
    Ok(true)
}
    pub async fn delete_post(id: i64, con: &Arc<Connection>)-> Result<(), BenwisAppError>{
        con.execute("DELETE FROM posts WHERE id = ?",&[Value::Integer(id)])?;
        Ok(())
    }
    pub async fn update_post(post: Post, con: &Arc<Connection>)-> Result<(), BenwisAppError>{

    let excerpt = match post.excerpt{
    Some(e) => Text(e),
    None => Null,
    };

    let slug = match post.slug.is_empty() {
        true => slugify(&post.title),
        false => post.slug,
    };

    let hero = match post.hero{
    Some(e) => Text(e),
    None => Null,
    };

    let hero_alt = match post.hero_alt{
    Some(e) => Text(e),
    None => Null,
    };

    let hero_caption = match post.hero_caption{
    Some(e) => Text(e),
    None => Null,
    };

    let toc = match post.toc{
    Some(e) => Text(e),
    None => Null,
    };

    println!("created_at: {}", post.created_at);
    let created_at = post.created_at.timestamp();

    let Ok(tags) = serde_json::to_string(&post.tags) else{
    return Err(BenwisAppError::JsonError("Failed to serialize tags".to_string()));
    };

    con.execute("UPDATE posts SET title=?,slug=?,excerpt=?,raw_content=?,content=?,published=?,preview=?, author_id=?, hero=?, hero_alt=?, hero_caption=?, tags=?, toc=?, created_at=?  WHERE id = ?",&[Text(post.title), Text(slug), excerpt, Text(post.raw_content), Text(post.content), Integer(post.published.into()), Integer(post.preview.into()), Integer(post.author_id), hero, hero_alt, hero_caption, Text(tags), toc, Integer(created_at), Integer(post.id) ])?;
    Ok(())
    }
}
}
    }
