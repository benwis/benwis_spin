//use crate::functions::dark_mode::ToggleDarkMode;
use leptos::*;
//use leptos_use::{use_color_mode, ColorMode, UseColorModeReturn};

use crate::functions::dark_mode::ToggleDarkMode;

#[cfg(not(feature = "ssr"))]
fn initial_prefers_dark() -> bool {
    use wasm_bindgen::JsCast;
    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    cookie.contains("darkmode=true")
}

#[cfg(feature = "ssr")]
fn initial_prefers_dark() -> bool {
    use_context::<leptos_spin::RequestParts>().is_some_and(|req| {
        req.headers()
            .iter()
            .filter(|(k, v)| k.contains("Set-Cookie"))
            .any(|(k, v)| String::from_utf8_lossy(v).contains("darkmode=true"))
    })
}

#[derive(Clone)]
pub struct ColorScheme {
    pub action: Action<ToggleDarkMode, Result<bool, ServerFnError>>,
    pub prefers_dark: Signal<bool>,
}

//#[derive(Clone, Debug)]
//pub struct ColorScheme {
//    pub read: Signal<ColorMode>,
//    pub write: WriteSignal<ColorMode>,
//}
//pub fn provide_color_scheme() -> ColorScheme {
//    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();
//    let cs = ColorScheme {
//        read: mode,
//        write: set_mode,
//    };
//    provide_context(cs.clone());
//    cs
//}
pub fn provide_color_scheme() -> Signal<bool> {
    // let color_scheme_signal = create_rw_signal( false);
    // provide_context( ColorScheme(color_scheme_signal));

    let initial = initial_prefers_dark();

    let toggle_dark_mode_action = create_server_action::<ToggleDarkMode>();
    // input is `Some(value)` when pending, and `None` if not pending
    let input = toggle_dark_mode_action.input();
    // value contains most recently-returned value
    let value = toggle_dark_mode_action.value();

    // NOTE: if you're following along the with video, this was implemented
    // incorrectly at the time I made it, due to a bug in <ActionForm/> that
    // was not resetting input. This is how it should have been implemented
    // all along, which would also have fixed the bug at 49:24!
    let prefers_dark_fn = move || {
        match (input.get(), value.get()) {
            // if there's some current input, use that optimistically
            (Some(submission), _) => submission.prefers_dark,
            // otherwise, if there was a previous value confirmed by server, use that
            (_, Some(Ok(value))) => value,
            // otherwise, use the initial value
            _ => initial,
        }
    };
    let prefers_dark = Signal::derive(prefers_dark_fn);

    provide_context(ColorScheme {
        action: toggle_dark_mode_action,
        prefers_dark,
    });
    prefers_dark
}
