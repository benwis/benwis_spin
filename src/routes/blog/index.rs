use crate::functions::post::{get_posts, AddPost, DeletePost, UpdatePost};
use crate::providers::AuthContext;
use leptos::*;
use leptos_meta::*;
use leptos_router::{ActionForm, A};

#[component]
pub fn Blog() -> impl IntoView {
    let add_post = create_server_multi_action::<AddPost>();
    let update_post = create_server_action::<UpdatePost>();
    let delete_post = create_server_action::<DeletePost>();
    // list of posts is loaded from the server in reaction to changes
    let posts = create_resource(
        move || {
            (
                add_post.version().get(),
                update_post.version().get(),
                delete_post.version().get(),
            )
        },
        move |_| get_posts(Some(5)),
    );

    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    view! {
        <Meta property="og:title" content="benwis Blog"/>
        <Title text="benwis Blog"/>
        <Meta
            name="description"
            content="The potentially misguided ramblings of a Rust developer flailing around on the web"
        />
        <Meta
            property="og:description"
            content="The potentially misguided ramblings of a Rust developer flailing around on the web"
        />
        <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg"/>

        <div id="archive">
            <div id="archive-background"></div>
            <div id="archive__header">
                <div id="archive__header-prev" class="archive__pagination">
                    <a class="archive__pagination-link" href="">
                        <svg
                            class="archive__pagination-icon"
                            clip-rule="evenodd"
                            fill-rule="evenodd"
                            stroke-linejoin="round"
                            stroke-miterlimit="2"
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="m9.474 5.209s-4.501 4.505-6.254 6.259c-.147.146-.22.338-.22.53s.073.384.22.53c1.752 1.754 6.252 6.257 6.252 6.257.145.145.336.217.527.217.191-.001.383-.074.53-.221.293-.293.294-.766.004-1.057l-4.976-4.976h14.692c.414 0 .75-.336.75-.75s-.336-.75-.75-.75h-14.692l4.978-4.979c.289-.289.287-.761-.006-1.054-.147-.147-.339-.221-.53-.221-.191-.001-.38.071-.525.215z"
                                fill-rule="nonzero"
                            ></path>
                        </svg>
                        1
                    </a>
                </div>
                <h1 id="archive__heading">Blog</h1>
                <div id="archive__header-next" class="archive__pagination">
                    <a href="" class="archive__pagination-link">
                        3
                        <svg
                            class="archive__pagination-icon"
                            clip-rule="evenodd"
                            fill-rule="evenodd"
                            stroke-linejoin="round"
                            stroke-miterlimit="2"
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="m14.523 18.787s4.501-4.505 6.255-6.26c.146-.146.219-.338.219-.53s-.073-.383-.219-.53c-1.753-1.754-6.255-6.258-6.255-6.258-.144-.145-.334-.217-.524-.217-.193 0-.385.074-.532.221-.293.292-.295.766-.004 1.056l4.978 4.978h-14.692c-.414 0-.75.336-.75.75s.336.75.75.75h14.692l-4.979 4.979c-.289.289-.286.762.006 1.054.148.148.341.222.533.222.19 0 .378-.072.522-.215z"
                                fill-rule="nonzero"
                            ></path>
                        </svg>
                    </a>
                </div>
            </div>

            <div id="archive__posts">

                <Transition fallback=move || {
                    view! { <p>"Loading..."</p> }
                }>
                    {move || {
                        let existing_posts = {
                            move || {
                                posts
                                    .get()
                                    .map(move |posts| match posts {
                                        Err(e) => {
                                            vec![
                                                view! { <pre class="error">"Error: " {e.to_string()}</pre> }
                                                    .into_any(),
                                            ]
                                        }
                                        Ok(posts) => {
                                            if posts.is_empty() {
                                                vec![
                                                    view! {
                                                        <p class="text-black dark:text-white">
                                                            "No posts were found."
                                                        </p>
                                                    }
                                                        .into_any(),
                                                ]
                                            } else {
                                                posts
                                                    .into_iter()
                                                    .filter(|post| { post.published })
                                                    .map(move |post| {
                                                        let post_slug: StoredValue<String> = store_value(
                                                            post.slug.clone(),
                                                        );
                                                        view! {
                                                            <a class="post-card" href=format!("/posts/{}", post.slug)>
                                                                <img class="post-card__image" src=post.hero/>
                                                                <div class="post-card__text">
                                                                    <h2 class="post-card__heading">{post.title}</h2>
                                                                    <p class="post-card__meta">{post.created_at.to_string()}</p>
                                                                    <p class="post-card__excerpt">{post.excerpt}</p>
                                                                </div>
                                                                <Transition fallback=move || ()>
                                                                    {move || {
                                                                        let user = move || match auth_context.user.get() {
                                                                            Some(Ok(Some(user))) => Some(user),
                                                                            Some(Ok(None)) => None,
                                                                            Some(Err(_)) => None,
                                                                            None => None,
                                                                        };
                                                                        view! {
                                                                            <Show when=move || user().is_some() fallback=|| ()>
                                                                                <A href=format!(
                                                                                    "{}/edit",
                                                                                    post_slug.get_value(),
                                                                                )>"Edit Post"</A>
                                                                                <ActionForm action=delete_post>
                                                                                    <input type="hidden" name="id" value=post.id/>
                                                                                    <input type="submit" value="Delete Post"/>
                                                                                </ActionForm>
                                                                            </Show>
                                                                        }
                                                                    }}

                                                                </Transition>
                                                            </a>
                                                        }
                                                            .into_any()
                                                    })
                                                    .collect::<Vec<_>>()
                                            }
                                        }
                                    })
                                    .unwrap_or_default()
                            }
                        };
                        view! { {existing_posts} }
                    }}

                </Transition>
            </div>
            <div id="archive__footer">
                <div id="archive__footer-prev" class="archive__pagination">
                    <a class="archive__pagination-link" href="">
                        <svg
                            class="archive__pagination-icon"
                            clip-rule="evenodd"
                            fill-rule="evenodd"
                            stroke-linejoin="round"
                            stroke-miterlimit="2"
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="m9.474 5.209s-4.501 4.505-6.254 6.259c-.147.146-.22.338-.22.53s.073.384.22.53c1.752 1.754 6.252 6.257 6.252 6.257.145.145.336.217.527.217.191-.001.383-.074.53-.221.293-.293.294-.766.004-1.057l-4.976-4.976h14.692c.414 0 .75-.336.75-.75s-.336-.75-.75-.75h-14.692l4.978-4.979c.289-.289.287-.761-.006-1.054-.147-.147-.339-.221-.53-.221-.191-.001-.38.071-.525.215z"
                                fill-rule="nonzero"
                            ></path>
                        </svg>

                    </a>
                </div>
                <div id="archive__footer-numbers">
                    <a href="" class="archive__pagination-page-link">
                        1
                    </a>
                    <a href="" class="archive__pagination-page-link">
                        2
                    </a>
                    <a href="" class="archive__pagination-page-link">
                        3
                    </a>
                    <a href="" class="archive__pagination-page-link">
                        4
                    </a>
                </div>
                <div id="archive__footer-next" class="archive__pagination">
                    <a href="" class="archive__pagination-link">

                        <svg
                            class="archive__pagination-icon"
                            clip-rule="evenodd"
                            fill-rule="evenodd"
                            stroke-linejoin="round"
                            stroke-miterlimit="2"
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="m14.523 18.787s4.501-4.505 6.255-6.26c.146-.146.219-.338.219-.53s-.073-.383-.219-.53c-1.753-1.754-6.255-6.258-6.255-6.258-.144-.145-.334-.217-.524-.217-.193 0-.385.074-.532.221-.293.292-.295.766-.004 1.056l4.978 4.978h-14.692c-.414 0-.75.336-.75.75s.336.75.75.75h14.692l-4.979 4.979c-.289.289-.286.762.006 1.054.148.148.341.222.533.222.19 0 .378-.072.522-.215z"
                                fill-rule="nonzero"
                            ></path>
                        </svg>
                    </a>
                </div>
            </div>

        </div>
    }
}
