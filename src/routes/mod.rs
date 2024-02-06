pub mod blog;
pub use blog::*;
pub mod index;
pub use index::*;
pub mod about;
pub use about::*;
pub mod portfolio;
pub use portfolio::*;
pub mod nedry;
pub mod notfound;
pub use notfound::*;

pub use nedry::*;

use crate::error_template::ErrorTemplate;
use crate::layouts::Default;
use crate::providers::provide_color_scheme;
use crate::routes::Blog;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn AppRouter() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    _ = provide_color_scheme();

    view! {
      <Stylesheet id="leptos" href="pkg/pkg/benwis-leptos.css"/>

      // content for this welcome page
      <Router>
        <Routes>
          <Route
            path="minimal"
            view=move || {
                view! { <Index/> }
            }
          />

          <Route
            path=""
            view=|| {
                view! {
                  <Default>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorTemplate errors=errors/> }
                    }>
                      <Outlet/>
                    </ErrorBoundary>
                  </Default>
                }
            }
          >

            <Route
              path=""
              view=move || {
                  view! { <Index/> }
              }
            />

            <Route
              path="about"
              view=move || {
                  view! { <About/> }
              }
            />

            <Route
              path="portfolio"
              view=move || {
                  view! { <Portfolio/> }
              }
            />

            <Route
              path="posts"
              view=move || {
                  view! { <Blog/> }
              }
            />

            <Route
              path="posts/:slug"
              view=move || {
                  view! { <Post/> }
              }

              ssr=SsrMode::Async
            />

            <Route
              path="nedry"
              view=move || {
                  view! { <Nedry/> }
              }
            />

            <Route path="/*any" view=NotFound/>
          </Route>
        </Routes>
      </Router>
    }
}
