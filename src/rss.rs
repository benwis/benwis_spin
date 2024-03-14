use crate::models::Post;
use xml::escape::escape_str_pcdata;
use spin_sdk::http::{OutgoingResponse, Headers, ResponseOutparam};
use spin_sdk::sqlite::Connection;
use futures_util::sink::SinkExt;
use std::sync::Arc;

pub struct RssEntry {
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub pub_date: String,
    pub author: String,
    pub guid: String,
}

impl From<Post> for RssEntry {
    fn from(post: Post) -> Self {
        let full_url = format!("https://benw.is/posts/{}", post.slug);
        Self {
            title: post.title,
            link: full_url.clone(),
            description: post.excerpt,
            pub_date: post.created_at.to_rfc2822(),
            author: "benwis".to_string(),
            guid: full_url,
        }
    }
}

impl RssEntry {
    // Converts an RSSEntry to a String containing the rss item tags
    pub fn to_item(&self) -> String {
        format!(
r#"
        <item>
            <title><![CDATA[{}]]></title>
            <description><![CDATA[{}]]></description>
            <pubDate>{}</pubDate>
            <link>{}</link>
            <guid>{}</guid>
        </item>
      "#,
            self.title,
            self.description.clone().unwrap_or_default(),
            self.pub_date,
            self.guid,
            self.guid
        )
    }
}

pub fn generate_rss(
    title: &str,
    description: &str,
    link: &str,
    posts: &[Post],
) -> String {
    let rss_entries = posts
        .iter()
        .filter(|p| p.published)
        .cloned()
        .map(|p| p.into())
        .map(|r: RssEntry| r.to_item())
        .collect::<String>();

    // It's possible to insert XML injection attacks that might affect the RSS readers
    // if untrusted people can put in title, description, or link. To solve that,
    // we can escape these inputs
    let safe_title = escape_str_pcdata(title);
    let safe_description = escape_str_pcdata(description);

    format!(
r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
    <channel>
        <title>{safe_title}</title>
        <description>{safe_description}</description>
        <link>{link}</link>
        <language>en-us</language>
        <ttl>60</ttl>
        <atom:link href="https://benw.is/rss.xml" rel="self" type="application/rss+xml" />
        {}
    </channel>
</rss>   
     "#,
        rss_entries
    )
}
pub async fn rss_page(resp_out: ResponseOutparam,con: &Arc<Connection>) {
    // list of posts is loaded from the server in reaction to changes
    let posts = Post::get_posts(con).expect("Unable to get Posts");
    let rss = generate_rss(
        "benwis Blog",
        "The potentially misguided ramblings of a Rust developer flailing around on the web",
        "http://benw.is",
        &posts,
    );

    let headers = Headers::new(&[("Cache-Control".to_string(), "private, max-age=3600".as_bytes().to_vec())]);
  let og = OutgoingResponse::new(200, &headers);
            let mut ogbod = og.take_body();
            resp_out.set(og);
            ogbod.send(rss.into()).await.unwrap();
}
