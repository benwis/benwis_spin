use leptos::{use_context, ServerFnError, expect_context};
use leptos_spin_macro::server;

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(ToggleDarkMode, "/api")]
pub async fn toggle_dark_mode(prefers_dark: bool) -> Result<bool, ServerFnError> {
    use leptos_spin::{ResponseOptions, RequestParts, redirect};
    let req = expect_context::<RequestParts>();
    use std::borrow::Cow;

    let res_options =
        use_context::<ResponseOptions>().expect("to have leptos_spin::ResponseOptions provided");
    let cookie = format!("darkmode={};Path=/",prefers_dark );
    res_options.insert_header("Set-Cookie", cookie.as_bytes());
    Ok(prefers_dark)
}
