mod components;
mod error_template;
mod errors;
mod functions;
mod layouts;
mod models;
mod providers;
mod routes;
mod session;

#[cfg(feature = "ssr")]
mod server;

#[cfg(feature = "ssr")]
mod rss;

#[cfg(not(feature = "ssr"))]
mod js;

use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {

      console_error_panic_hook::set_once();
      leptos::leptos_dom::HydrationCtx::stop_hydrating();
    }
}
}
