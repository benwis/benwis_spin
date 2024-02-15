use crate::components::DarkModeToggle;
use crate::providers::AuthContext;
use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    view! {
      <nav class="top-nav bg-white dark:bg-gray-900 text-gray-700 dark:text-white">
        <div class="text-2xl">
          <a href="/">
            <h3 class="text-2xl dark:text-white text-base">"BENWIS"</h3>
          </a>
        </div>
        <input id="menu-toggle" type="checkbox"/>
        <label class="menu-button-container" for="menu-toggle">
          <div class="menu-button"></div>
        </label>
        <ul class="menu items-center z-10">
          <li class="items-center">
            <a href="/posts">"Blog"</a>
          </li>
          <li class="items-center">
            <a href="/about">"About Me"</a>
          </li>
          <li class="items-center">
            <a href="/portfolio">"Portfolio"</a>
          </li>
          <Transition fallback=move || ()>
            {move || {
                let user = move || match auth_context.user.get() {
                    Some(Ok(Some(user))) => Some(user),
                    Some(Ok(None)) => None,
                    Some(Err(_)) => None,
                    None => None,
                };
                view! {
                  <Show
                    when=move || user().is_none()
                    fallback=|| {
                        view! {
                          <li class="items-center">
                            <a href="/signup">"Signup"</a>
                          </li>
                        }
                    }
                  >

                    {|| ()}
                  </Show>
                  <Show
                    when=move || user().is_some()
                    fallback=|| {
                        view! {
                          <li class="items-center">
                            <a href="/login">"Login"</a>
                          </li>
                        }
                    }
                  >

                    <li class="items-center">
                      <a href="/logout">"Logout"</a>
                    </li>
                  </Show>
                }
            }}

          </Transition>
          <DarkModeToggle/>
        </ul>
      </nav>
    }
}
