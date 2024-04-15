use crate::functions::post::get_post;
use crate::models::post;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct PostParams {
    pub slug: Option<String>,
}

#[component]
pub fn Post() -> impl IntoView {
    let params = use_params::<PostParams>();
    let post = create_blocking_resource(
        move || params.get().map(|params| params.slug).ok().unwrap(),
        move |slug| get_post(slug.unwrap_or_default()),
    );
    view! {
        <Transition fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || {
                post.get()
                    .map(|p| {
                        match p {
                            Ok(Some(post)) => view! { <PostContent post=post/> }.into_view(),
                            Ok(None) => view! { <p>"Post Not Found"</p> }.into_view(),
                            Err(e) => view! { <p>"Error:" {e.to_string()}</p> }.into_view(),
                        }
                    })
            }}

        </Transition>
    }
}

#[component]
pub fn PostContent(post: post::Post) -> impl IntoView {
    view! {
        <Meta property="og:title" content=post.title.clone()/>
        <Meta property="og:description" content=post.excerpt.clone().unwrap_or_default()/>
        <Meta property="og:site_name" content="benw.is"/>
        <Meta property="og:locale" content="en-us"/>
        <Meta property="og:type" content="article"/>
        <Meta
            property="og:image"
            content=post.hero.clone().unwrap_or("https://benw.is/img/ben_catcarbon.png".to_string())
        />
        <Meta property="og:image:type" content="image/png"/>
        <Meta property="og:url" content=format!("https://benw.is/posts/{}", post.slug.clone())/>
        <Meta name="twitter:title" content=post.title.clone()/>
        <Meta name="twitter:site" content="@iambenwis"/>
        <Title text=post.title.clone()/>
        <Meta
            name="twitter:card"
            content=if post.hero.is_some() { "summary_large_image" } else { "summary" }
        />

        <Meta
            name="twitter:image"
            content=post.hero.clone().unwrap_or("https://benw.is/img/ben_catcarbon.png".to_string())
        />
        <Meta name="twitter:description" content=post.excerpt.clone().unwrap_or_default()/>
        <Meta name="description" content=post.excerpt.clone().unwrap_or_default()/>
        <Link rel="canonical" href=format!("https://benw.is/posts/{}", post.slug.clone())/>

        {(post.preview || post.published)
            .then(|| {
                view! {
                    <div id="page__header">
                        <h1 id="page__heading">{post.title}</h1>
                        <p id="page__meta">{post.created_at.to_string()}</p>
                    </div>
                    <hr/>
                    <div id="page__layout">
                        <div id="page__body">
                            <div id="page__toc">
                                <div id="page__toc-sticky">
                                    <div class="page__sidebar-section">
                                        <h2 class="page__sidebar-section-heading">Contents</h2>

                                        {post.toc}
                                    </div>
                                </div>
                            </div>
                            <main id="page__content" class="content">

                                <div class="post__hero">
                                    <img id="post__image" src=post.hero alt=post.hero_alt/>
                                    <caption>{post.hero_caption}</caption>
                                </div>
                                <hr/>
                                <div
                                    id="page__content"
                                    class="content"
                                    inner_html=post.content
                                ></div>
                            </main>
                        </div>
                        // Insert Sidebar

                        <div id="page__sidebar">
                            <div id="page__sidebar-sticky">
                                <div id="page__sidebar-layout">
                                    <div class="page__sidebar-section">
                                        <h2 class="page__sidebar-section-heading">Previous</h2>
                                        <a class="post-card" href="#">

                                            <img
                                                class="post-card__image"
                                                src="https://placehold.co/600x400"
                                            />
                                            <div class="post-card__text">
                                                <h2 class="post-card__heading">This is a post heading</h2>
                                                <p class="post-card__meta">Post time/date, etc</p>
                                                <p class="post-card__excerpt">
                                                    This is a brief description or summary of the
                                                    post that should
                                                    entice
                                                    people to click in.
                                                </p>
                                            </div>
                                        </a>
                                    </div>
                                    <div class="page__sidebar-section">
                                        <h2 class="page__sidebar-section-heading">Next</h2>
                                        <a class="post-card" href="#">

                                            <img
                                                class="post-card__image"
                                                src="https://placehold.co/600x400"
                                            />
                                            <div class="post-card__text">
                                                <h2 class="post-card__heading">This is a post heading</h2>
                                                <p class="post-card__meta">Post time/date, etc</p>
                                                <p class="post-card__excerpt">
                                                    This is a brief description or summary of the
                                                    post that should
                                                    entice
                                                    people to click in.
                                                </p>
                                            </div>
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            })}
    }
}
