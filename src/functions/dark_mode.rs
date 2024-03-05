use leptos::{use_context, ServerFnError, expect_context};
use leptos_spin_macro::server;

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(ToggleDarkMode, "/api")]
pub async fn toggle_dark_mode(prefers_dark: bool) -> Result<bool, ServerFnError> {
    use leptos_spin::{ResponseOptions, RequestParts, redirect};
    let req = expect_context::<RequestParts>();
    use std::borrow::Cow;

    //let referrer:String = req.headers().iter().find(|(k,v)| k=="Referer").map(|(k, v)| v).ok_or_else(ServerFnError::Request("No Referer header provided!".to_string()))?;

    let referrer: Vec<(&String, String)> = req
        .headers()
        .iter()
        .filter(|(k, _v)| k == "referer")
        .map(|(k, v)| (k, String::from_utf8_lossy(v).to_string()))
        .collect();
    let referrer = referrer.first().map(|(k, v)| v);
    let Some(referrer) = referrer else {
        return Err(ServerFnError::new("No Referer Provided".to_string()))
    };
    println!("REFERRER: {referrer}");
    let res_options =
        use_context::<ResponseOptions>().expect("to have leptos_spin::ResponseOptions provided");
    let cookie = format!("darkmode={};Path=/",prefers_dark );
    res_options.insert_header("Set-Cookie", cookie.as_bytes());
    redirect(referrer);
    Ok(prefers_dark)
}
