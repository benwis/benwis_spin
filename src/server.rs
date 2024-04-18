use http::Uri;
use leptos::provide_context;
use leptos_spin::server_fn::register_explicit;
use leptos_spin::{render_best_match_to_stream_with_context, RouteTable};
use spin_sdk::http::{IncomingRequest, ResponseOutparam};
use spin_sdk::{http_component, sqlite::Connection};
use std::sync::Arc;

use crate::rss::rss_page;
use crate::session::SqliteStore;

#[http_component]
async fn handle_benwis_leptos(req: IncomingRequest, resp_out: ResponseOutparam) {
    let mut conf = leptos::get_configuration(None).await.unwrap();
    conf.leptos_options.output_name = "benwis-leptos".to_owned();

    let app_router = crate::routes::AppRouter;

    let mut routes = RouteTable::build(app_router);
    routes.add_server_fn_prefix("/api").unwrap();

    let con = Arc::new(Connection::open("default").expect("Failed to open benwis_leptos db"));
    
    // Setup up Store for user sessions
    let store = SqliteStore::from_connection(con.clone());
    store.migrate().await.expect("Failed to migrate sessions!");

    // Register server functions
    register_explicit::<crate::functions::post::AddPost>();
    register_explicit::<crate::functions::post::UpdatePost>();
    register_explicit::<crate::functions::post::DeletePost>();
    register_explicit::<crate::functions::post::GetPost>();
    register_explicit::<crate::functions::post::GetPostCount>();
    register_explicit::<crate::functions::post::GetPostWithSiblings>();
    register_explicit::<crate::functions::auth::Login>();
    register_explicit::<crate::functions::auth::Logout>();
    register_explicit::<crate::functions::auth::Signup>();
    register_explicit::<crate::functions::user::GetUser>();
    register_explicit::<crate::functions::user::GetSafeUser>();
    register_explicit::<crate::functions::post::GetPosts>();

    register_explicit::<crate::functions::post::GetPostsPaginated>();
    register_explicit::<crate::functions::dark_mode::ToggleDarkMode>();
    
    // Render the rss.xml file page here because it's not handled by Leptos
    let req_uri = req.uri();
    let uri = req_uri.parse::<Uri>().expect("Invalid URI");
    let path = uri.path();

    if path == "/rss.xml"{
        return rss_page(resp_out, &con).await
    }
    render_best_match_to_stream_with_context(
        req,
        resp_out,
        &routes,
        app_router,
        move || {
            provide_context(con.clone());
            provide_context(store.clone());
        },
        &conf.leptos_options,
    )
    .await
}
