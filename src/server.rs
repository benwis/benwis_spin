use leptos::provide_context;
use leptos_spin::{
    render_best_match_to_stream_with_context, server_fn::register_explicit, RouteTable,
};
use spin_sdk::http::{IncomingRequest, ResponseOutparam};
use spin_sdk::{http_component, sqlite::Connection};
use std::sync::Arc;

#[http_component]
async fn handle_spin_test(req: IncomingRequest, resp_out: ResponseOutparam) {
    let mut conf = leptos::get_configuration(None).await.unwrap();
    conf.leptos_options.output_name = "benwis-leptos".to_owned();

    // Register server functions

    // register_explicit::<crate::pages::home::GetCount>();
    // register_explicit::<crate::pages::home::UpdateCount>();

    let app_router = crate::routes::AppRouter;

    let mut routes = RouteTable::build(app_router);
    routes.add_server_fn_prefix("/api").unwrap();

    let con = Arc::new(Connection::open("default").expect("Failed to open benwis_leptos db"));

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
