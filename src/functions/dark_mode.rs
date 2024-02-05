use leptos::*;

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(ToggleDarkMode, "/api")]
pub async fn toggle_dark_mode(prefers_dark: bool) -> Result<bool, ServerFnError> {
    use leptos_spin::ResponseOptions;

    let res_options =
        use_context::<ResponseOptions>().expect("to have leptos_spin::ResponseOptions provided");
    res_options.insert_header("Set-Cookie", "darkmode={prefer_dark}; Path=/".as_bytes());

    Ok(prefers_dark)
}
