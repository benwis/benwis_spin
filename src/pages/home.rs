use leptos::{
    component, create_resource, create_server_action,
     view, IntoView, ServerFnError, SignalGet
};
use leptos_router::*;
use leptos_spin_macro::server;
/// Renders the home page of your application.
#[component]
pub fn Home() -> impl IntoView {
    // Creates a reactive value to update the button
    let increment_count = create_server_action::<UpdateCount>();

    let count = create_resource(
        move || {
            (
                increment_count.version().get(),
                // clear.version().get(),
            )
        },
        |_| get_count(),
    );

    view! {
      <picture class="img">
        <source
          srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
          media="(prefers-color-scheme: dark)"
        />
        <img
          src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
          alt="Leptos Logo"
          height="200"
          width="400"
        />
      </picture>

      <h1>"Welcome to Leptos"</h1>

      <ActionForm action=increment_count>
        <button>"Click Me: " {move || count.get()}</button>
      </ActionForm>
    }
}

#[server(UpdateCount, "/api")]
pub async fn update_count() -> Result<(), ServerFnError> {
    use leptos::server_fn::error::NoCustomError;
    use leptos::expect_context;
    println!("Upated count");

    let store = spin_sdk::key_value::Store::open_default()?;

    let count: u64 = store
        .get_json("spin_test_count")
        .map_err(|e| ServerFnError::ServerError::<NoCustomError>(e.to_string()))?
        .unwrap_or_default();

    let updated_count = count + 1;

    store
        .set_json("spin_test_count", &updated_count)
        .map_err(|e| ServerFnError::ServerError::<NoCustomError>(e.to_string()))?;

    let res_options = expect_context::<leptos_spin::ResponseOptions>();
    res_options.append_header("foo", &"bar".as_bytes());
    leptos_spin::redirect("/foobar");
    Ok(())
}

#[server(GetCount, "/api")]
pub async fn get_count() -> Result<u64, ServerFnError> {
    use leptos::server_fn::error::NoCustomError;
    let store = spin_sdk::key_value::Store::open_default()?;

    let stored_count: u64 = store
        .get_json("spin_test_count")
        .map_err(|e| ServerFnError::ServerError::<NoCustomError>(e.to_string()))?
        .ok_or_else(|| {
            ServerFnError::ServerError::<NoCustomError>("Failed to get count".to_string())
        })?;

    println!("Got stored {stored_count}");

    Ok(stored_count)
}
