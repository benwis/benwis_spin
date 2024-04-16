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
pub mod auth;
pub use auth::*;
pub use nedry::*;

use crate::error_template::ErrorTemplate;
use crate::layouts::{Default, Home};
use crate::providers::{provide_auth, provide_color_scheme, AuthContext};
use crate::routes::Blog;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn AppRouter() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_auth();
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    _ = provide_color_scheme();

    view! {
        <Stylesheet id="leptos" href="/benwis-leptos.css"/>

        // content for this welcome page
        <Router>
            <Routes>
                <Route
                    path=""
                    view=|| {
                        view! {
                            <Home>
                                <ErrorBoundary fallback=|errors| {
                                    view! { <ErrorTemplate errors=errors/> }
                                }>
                                    <Outlet/>
                                </ErrorBoundary>
                            </Home>
                        }
                    }
                >

                    <Route
                        path=""
                        view=move || {
                            view! { <Index/> }
                        }
                    />

                </Route>
                <Route
                    path="/*"
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
                        path="posts/add"
                        view=move || {
                            view! { <AddPost/> }
                        }
                    />

                    <Route
                        path="posts/:slug/edit"
                        view=move || {
                            view! { <EditPost/> }
                        }
                    />

                    <Route
                        path="nedry"
                        view=move || {
                            view! { <Nedry/> }
                        }
                    />

                    <Route
                        path="signup"
                        view=move || {
                            view! { <Join action=auth_context.signup/> }
                        }
                    />

                    <Route
                        path="login"
                        view=move || {
                            view! { <Login action=auth_context.login/> }
                        }
                    />

                    <Route
                        path="logout"
                        view=move || {
                            view! { <Logout action=auth_context.logout/> }
                        }
                    />

                    <Route path="/*any" view=NotFound/>
                </Route>
            </Routes>
        </Router>
    }
}
