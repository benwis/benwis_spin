use crate::functions::post::AddPost;
#[cfg(not(feature = "ssr"))]
use crate::js;
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn PostEditor() -> impl IntoView {
    let add_post = create_server_action::<AddPost>();
    let (content, write_content) = create_signal(String::new());

    view! {
      <Meta property="og:title" content="Add Post"/>
      <Title text="Add Post"/>
      <Meta name="description" content="Add a post"/>
      <Meta property="og:description" content="Add a post"/>

      <div class="grid min-h-full w-full grid-cols-2">
        <div class="w-full h-full px-8">
          <ActionForm action=add_post class="w-full text-black dark:text-white">
            <input type="hidden" name="author_id" value=1/>
            <label for="content" class="hidden">
              "Content:"
            </label>
            <textarea
              type="text"
              id="content"
              rows=50
              class="w-full text-black border border-gray-500"
              name="content"
              on:input=move |ev| {
                  cfg_if! {
                      if #[cfg(not(feature = "ssr"))] { let new_value =
                      event_target_value(& ev); let output =
                      js::process_markdown_to_html(new_value.into()); match output
                      { Ok(o) => write_content.set(o.content), Err(e) =>
                      leptos::logging::log!("{}", e) } }
                  }
              }
            >
            </textarea>

            <p class="text-right flex w-full justify-between">
              <button
                type="submit"
                class="rounded bg-blue-500 py-2 px-4 text-white hover:bg-blue-600 focus:bg-blue-400"
              >
                "Add Post"
              </button>
            </p>
          </ActionForm>
        </div>
        <section class="border-solid border-gray-500 border-4 ">
          <div
            class="prose text-black prose lg:prose-xl dark:prose-invert dark:text-white text-base p-4"
            inner_html=move || content.get()
          ></div>
        </section>
        >
      </div>
    }
}
