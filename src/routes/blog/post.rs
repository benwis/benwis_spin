use crate::functions::post::get_post_with_siblings;
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
    let triad = create_blocking_resource(
        move || params.get().map(|params| params.slug).ok().unwrap(),
        move |slug| get_post_with_siblings(slug.unwrap_or_default()),
    );
    view! {
        <Transition fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || {
                triad
                    .get()
                    .map(|t| {
                        match t {
                            Ok(Some(triad)) => view! { <PostContent triad=triad/> }.into_view(),
                            Ok(None) => view! { <p>"Post Not Found"</p> }.into_view(),
                            Err(e) => view! { <p>"Error:" {e.to_string()}</p> }.into_view(),
                        }
                    })
            }}

        </Transition>
    }
}

#[component]
pub fn PostContent(triad: post::PostTriad) -> impl IntoView {
    let triad_store = store_value(triad.clone());
    view! {
        <Meta property="og:title" content=triad.post.title.clone()/>
        <Meta property="og:description" content=triad.post.excerpt.clone().unwrap_or_default()/>
        <Meta property="og:site_name" content="benw.is"/>
        <Meta property="og:locale" content="en-us"/>
        <Meta property="og:type" content="article"/>
        <Meta
            property="og:image"
            content=triad
                .post
                .hero
                .clone()
                .unwrap_or("https://benw.is/img/ben_catcarbon.png".to_string())
        />
        <Meta property="og:image:type" content="image/png"/>
        <Meta
            property="og:url"
            content=format!("https://benw.is/posts/{}", triad.post.slug.clone())
        />
        <Meta name="twitter:title" content=triad.post.title.clone()/>
        <Meta name="twitter:site" content="@iambenwis"/>
        <Title text=triad.post.title.clone()/>
        <Meta
            name="twitter:card"
            content=if triad.post.hero.is_some() { "summary_large_image" } else { "summary" }
        />

        <Meta
            name="twitter:image"
            content=triad
                .post
                .hero
                .clone()
                .unwrap_or("https://benw.is/img/ben_catcarbon.png".to_string())
        />
        <Meta name="twitter:description" content=triad.post.excerpt.clone().unwrap_or_default()/>
        <Meta name="description" content=triad.post.excerpt.clone().unwrap_or_default()/>
        <Link rel="canonical" href=format!("https://benw.is/posts/{}", triad.post.slug.clone())/>

        {(triad.post.preview || triad.post.published)
            .then(move || {
                view! {
                    <div id="page">
                        <div id="page__header">
                            <h1 id="page__heading">{triad.post.title}</h1>
                            <p id="page__meta">{triad.post.created_at.to_string()}</p>
                        </div>
                        <div id="page__layout">
                            <div id="page__body">
                                <div id="page__toc">
                                    <div id="page__toc-sticky">
                                        <div class="page__sidebar-section">
                                            <h2 class="page__sidebar-section-heading">Contents</h2>
                                            {triad.post.toc}
                                        </div>
                                    </div>
                                </div>
                                <main id="page__content" class="content">
                                    <div class="post__hero">
                                        <img
                                            id="post__image"
                                            src=triad.post.hero
                                            alt=triad.post.hero_alt
                                        />
                                        <caption>{triad.post.hero_caption}</caption>
                                    </div>
                                    <hr/>
                                    <div inner_html=triad.post.content></div>
                                </main>
                            </div>
                            <div id="page__sidebar">
                                <div id="page__sidebar-sticky">
                                    <Show when=move || {
                                        triad_store.get_value().previous.is_some()
                                    }>

                                        {
                                            let prev = triad.previous.clone().unwrap_or_default();
                                            view! {
                                                <div class="page__sidebar-section">
                                                    <h2 class="page__sidebar-section-heading">Previous</h2>
                                                    <a class="post-card" href="#">

                                                        <img
                                                            class="post-card__image"
                                                            src=prev.hero
                                                            alt=prev.hero_alt
                                                        />
                                                        <div class="post-card__text">
                                                            <h2 class="post-card__heading">{prev.title}</h2>
                                                            <p class="post-card__meta">{prev.created_at.to_string()}</p>
                                                            <p class="post-card__excerpt">{prev.excerpt}</p>
                                                        </div>
                                                    </a>
                                                </div>
                                            }
                                        }

                                    </Show>

                                    <Show when=move || {
                                        triad_store.get_value().next.is_some()
                                    }>

                                        {
                                            let next = triad.next.clone().unwrap_or_default();
                                            view! {
                                                <div class="page__sidebar-section">
                                                    <h2 class="page__sidebar-section-heading">Next</h2>
                                                    <a class="post-card" href="#">

                                                        <img
                                                            class="post-card__image"
                                                            src=next.hero
                                                            alt=next.hero_alt
                                                        />
                                                        <div class="post-card__text">
                                                            <h2 class="post-card__heading">{next.title}</h2>
                                                            <p class="post-card__meta">{next.created_at.to_string()}</p>
                                                            <p class="post-card__excerpt">{next.excerpt}</p>
                                                        </div>
                                                    </a>
                                                </div>
                                            }
                                        }

                                    </Show>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            })}
    }
}
