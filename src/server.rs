use leptos::provide_context;
use leptos_spin::server_fn::register_explicit;
use leptos_spin::{render_best_match_to_stream_with_context, RouteTable};
use spin_sdk::http::{IncomingRequest, ResponseOutparam};
use spin_sdk::{http_component, sqlite::Connection};
use std::sync::Arc;

#[http_component]
async fn handle_benwis_leptos(req: IncomingRequest, resp_out: ResponseOutparam) {
    let mut conf = leptos::get_configuration(None).await.unwrap();
    conf.leptos_options.output_name = "benwis-leptos".to_owned();

    let app_router = crate::routes::AppRouter;

    let mut routes = RouteTable::build(app_router);
    routes.add_server_fn_prefix("/api").unwrap();

    let con = Arc::new(Connection::open("default").expect("Failed to open benwis_leptos db"));

    // Register server functions
    //register_explicit::<crate::functions::post::AddPost>();
    //register_explicit::<crate::functions::post::EditPost>();
    //register_explicit::<crate::functions::post::UpdatePost>();
    //register_explicit::<crate::functions::post::DeletePost>();
    register_explicit::<crate::functions::post::GetPost>();
    register_explicit::<crate::functions::post::GetPosts>();
    register_explicit::<crate::functions::dark_mode::ToggleDarkMode>();
    render_best_match_to_stream_with_context(
        req,
        resp_out,
        &routes,
        app_router,
        move || {
            provide_context(con.clone());
        },
        &conf.leptos_options,
    )
    .await
}
