use crate::functions::auth::Signup;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Join(action: Action<Signup, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
      <Meta property="og:title" content="Signup"/>
      <Title text="Signup"/>
      <Meta name="description" content="Signup for the site"/>
      <Meta property="og:description" content="Signup for the site"/>
      <Meta
        property="og:image"
        content="https://benwis.imgix.net/pictureofMe.jpeg"
      />
       <div class="auth">
          <div class="auth__header">
              <h1 class="auth__heading">
                "Join"
              </h1>
          </div>
          <div class="auth__body content">
            <ActionForm id="auth__signup-form" action=action>
              <div>
                <label
                  for="email"
                >
                  "Username"
                </label>
                <input
                    id="username"
                    required
                    name="username"
                    type="text"
                    autoComplete="username"
                    aria-describedby="username-error"
                  />
              </div>
              <div>
                <label
                  for="email"
                >
                  "Displayed Name"
                </label>
                  <input
                    id="display_name"
                    required
                    name="display_name"
                    type="text"
                    autoComplete="display_name"
                    aria-describedby="display_name-error"
                  />
              </div>
              <div>
                <label
                  for="password"
                >
                  "Password"
                </label>
                <input
                  id="password"
                  name="password"
                  type="password"
                  autoComplete="new-password"
                  aria-describedby="password-error"
                />
              </div>
              <div>
                <label
                  for="password_confirmation"
                >
                  "Confirm Password"
                </label>
                  <input
                    id="password_confirmation"
                    name="password_confirmation"
                    type="password"
                    autoComplete="password_confirmation"
                    aria-describedby="password_confirmation_error"
                  />
              </div>
              <button
                type="submit"
              >
                "Create Account"
              </button>

              <div class="auth__form-aside">
                <div class="auth__form_aside_row">
                  "Already have an account?"
                  <a href="/signup">
                    "Log in"
                  </a>
                </div>
              </div>

            </ActionForm>
        </div>
      </div>
    }
}
