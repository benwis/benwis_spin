use crate::functions::post::AddPost;
#[cfg(not(feature = "ssr"))]
use crate::js;
use crate::models::SafeUser;
use crate::routes::AuthContext;
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn AddPost() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");

    view! {
      <Meta property="og:title" content="Add Post"/>
      <Title text="Add Post"/>
      <Meta name="description" content="Add a post"/>
      <Meta property="og:description" content="Add a post"/>

      <Transition fallback=move || ()>
        {move || {
            let user = move || {
                match auth_context.user.get() {
                    Some(Ok(Some(user))) => Some(user),
                    Some(Ok(None)) => None,
                    Some(Err(_)) => None,
                    None => None,
                }
            };
            view! {
              <Show when=move || user().is_some() fallback=|| ().into_view()>
                <AddPostForm user=user()/>
              </Show>
            }
        }}

      </Transition>
    }
}

#[island]
pub fn AddPostForm(user: Option<SafeUser>) -> impl IntoView {
    let add_post = create_server_action::<AddPost>();
    let content = create_rw_signal(String::new());
    let toc = create_rw_signal::<Option<String>>(None);

    let show_post_metadata = create_rw_signal(false);
    view! {
      <ActionForm action=add_post class="w-full text-black dark:text-white">
        <div class="grid min-h-full w-full grid-cols-2">
          <section class="text-left flex-col w-full justify-between col-span-2 gap-4 dark:bg-gray-900 bg-slate-50 rounded mb-4 border-2 dark:border-yellow-400 border-gray-300 ">
            <div on:click=move |_e| {
                show_post_metadata.update(|b| *b = !*b);
            }>

              <Show
                when=move || show_post_metadata.get()
                fallback=move || {
                    view! {
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 448 512"
                        class="w-6 inline dark:stroke-white dark:fill-white px-2"
                      >
                        <path d="M256 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H48c-17.7 0-32 14.3-32 32s14.3 32 32 32H192V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H400c17.7 0 32-14.3 32-32s-14.3-32-32-32H256V80z"></path>
                      </svg>
                    }
                }
              >

                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 448 512"
                  class="inline w-6 dark:stroke-white dark:fill-white px-2"
                >
                  <path d="M432 256c0 17.7-14.3 32-32 32L48 288c-17.7 0-32-14.3-32-32s14.3-32 32-32l352 0c17.7 0 32 14.3 32 32z"></path>
                </svg>
              </Show>
              <h1 class="inline dark:text-white">Post Metadata</h1>
            </div>
            <Show
              when=move || show_post_metadata.get()
              fallback=|| ().into_view()
            >
              <div>
                <div class="relative my-2 mx-4">
                  <input
                    type="text"
                    id="title"
                    name="title"
                    class="block px-2.5 pb-2.5 pt-4 w-full text-sm text-gray-900 bg-transparent rounded-lg border-2 border-gray-300 appearance-none dark:text-white dark:border-yellow-400 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer"
                    placeholder=" "
                  />
                  <label
                    for="title"
                    class="absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] bg-white dark:bg-gray-900 px-2 peer-focus:px-2 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto start-1"
                  >
                    Post Title
                  </label>
                </div>

                <div class="relative mb-2 mx-4">
                  <input
                    type="text"
                    id="slug"
                    name="slug"
                    class="block px-2.5 pb-2.5 pt-4 w-full text-sm text-gray-900 bg-transparent  rounded-lg border-2 border-gray-300 appearance-none dark:text-white dark:border-yellow-400 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer"
                    placeholder=" "
                  />
                  <label
                    for="slug"
                    class="absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] bg-white dark:bg-gray-900 px-2 peer-focus:px-2 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto start-1"
                  >
                    Post Slug
                  </label>
                </div>

                <div class="relative mb-2 mx-4">
                  <input
                    type="text"
                    id="created_at_pretty"
                    name="created_at_pretty"
                    class="block px-2.5 pb-2.5 pt-4 w-full text-sm text-gray-900 bg-transparent  rounded-lg border-2 border-gray-300 appearance-none dark:text-white dark:border-yellow-400 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer"
                    placeholder="1970-01-01T00:00:00-00:00"
                  />
                  <label
                    for="created_at_pretty"
                    class="absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] bg-white dark:bg-gray-900 px-2 peer-focus:px-2 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto start-1"
                  >
                    Post Date
                  </label>
                </div>
                <div class="relative my-2 mx-4">
                  <textarea
                    type="text"
                    id="excerpt"
                    name="excerpt"
                    rows=10
                    class="block px-2.5 pb-2.5 pt-4 w-full text-sm text-gray-900 bg-transparent rounded-lg border-2 border-gray-300 appearance-none dark:text-white dark:border-yellow-400 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer"
                    placeholder=" "
                  ></textarea>
                  <label
                    for="excerpt"
                    class="absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] bg-white dark:bg-gray-900 px-2 peer-focus:px-2 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto start-1"
                  >
                    Excerpt
                  </label>
                </div>
                <div class="relative mb-2 mx-4">
                  <input
                    type="text"
                    id="hero"
                    name="hero"
                    class="block px-2.5 pb-2.5 pt-4 w-full text-sm text-gray-900 bg-transparent  rounded-lg border-2 border-gray-300 appearance-none dark:text-white dark:border-yellow-400 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer"
                    placeholder=" "
                  />
                  <label
                    for="hero"
                    class="absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] bg-white dark:bg-gray-900 px-2 peer-focus:px-2 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto start-1"
                  >
                    Hero
                  </label>
                </div>

                <div class="relative mb-2 mx-4">
                  <input
                    type="text"
                    id="hero_alt"
                    name="hero_alt"
                    class="block px-2.5 pb-2.5 pt-4 w-full text-sm text-gray-900 bg-transparent  rounded-lg border-2 border-gray-300 appearance-none dark:text-white dark:border-yellow-400 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer"
                    placeholder=" "
                  />
                  <label
                    for="hero_alt"
                    class="absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] bg-white dark:bg-gray-900 px-2 peer-focus:px-2 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto start-1"
                  >
                    Hero Alt
                  </label>
                </div>

                <div class="relative mb-2 mx-4">
                  <input
                    type="text"
                    id="hero_caption"
                    name="hero_caption"
                    class="block px-2.5 pb-2.5 pt-4 w-full text-sm text-gray-900 bg-transparent  rounded-lg border-2 border-gray-300 appearance-none dark:text-white dark:border-yellow-400 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer"
                    placeholder=" "
                  />
                  <label
                    for="hero_caption"
                    class="absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] bg-white dark:bg-gray-900 px-2 peer-focus:px-2 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto start-1"
                  >
                    Hero Caption
                  </label>
                </div>

                <div class="relative mb-2 mx-4">
                  <input
                    type="text"
                    id="tags"
                    name="tags"
                    class="block px-2.5 pb-2.5 pt-4 w-full text-sm text-gray-900 bg-transparent  rounded-lg border-2 border-gray-300 appearance-none dark:text-white dark:border-yellow-400 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer"
                    placeholder=" "
                  />
                  <label
                    for="tags"
                    class="absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] bg-white dark:bg-gray-900 px-2 peer-focus:px-2 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto start-1"
                  >
                    Post Tags
                  </label>
                </div>
                <div class="relative mb-2 mx-4">
                  <label
                    for="preview"
                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                  >
                    Preview
                  </label>
                  <select
                    id="preview"
                    name="preview"
                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                  >
                    <option selected value="false">
                      False
                    </option>
                    <option value="true">"True"</option>
                  </select>
                </div>

                <div class="relative mb-2 mx-4">
                  <label
                    for="published"
                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                  >
                    Published
                  </label>
                  <select
                    id="published"
                    name="published"
                    class=" bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                  >
                    <option selected value="false">
                      False
                    </option>
                    <option value="true">"True"</option>
                  </select>
                </div>
              </div>
            </Show>
          </section>
          <section class="flex flex-row-reverse w-full justify-between justify-items-end align-items-end col-span-2 bg-transparent rounded border-1 dark:border-yellow-400 border-gray-300">
            <button
              type="submit"
              class="rounded bg-yellow-400 py-2 px-4 m-2 text-black hover:bg-yellow-300 focus:bg-yellow-300"
            >
              "Add Post"
            </button>
          </section>
          <div class="w-full h-full pr-4">
            <input
              type="hidden"
              name="author_id"
              value=match user {
                  Some(u) => u.id,
                  None => -1,
              }
            />

            <input type="hidden" name="toc" id="toc" value=move || toc.get()/>
            <label for="content" class="hidden">
              "Content:"
            </label>
            <textarea
              type="text"
              id="raw_content"
              name="raw_content"
              rows=50
              class="w-full text-black border border-gray-500"
              on:input=move |ev| {
                  cfg_if! {
                      if #[cfg(not(feature = "ssr"))] { let new_value =
                      event_target_value(& ev); let output =
                      js::process_markdown_to_html_with_frontmatter(new_value
                      .into()); match output { Ok(o) => { content.set(o.content);
                      toc.set(o.toc); }, Err(e) => leptos::logging::log!("{}", e)
                      } }
                  }
              }
            >
            </textarea>
            <input
              type="hidden"
              name="content"
              id="content"
              value=move || content.get()
            />
          </div>
          <section class="shadow-md rounded">
            <div
              class="prose text-black prose dark:prose-invert dark:text-white text-base p-4 bg-slate-200 dark:bg-gray-800 w-full h-full rounded"
              inner_html=move || content.get()
            ></div>
          </section>
        </div>
      </ActionForm>
    }
}
