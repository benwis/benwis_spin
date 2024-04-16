use crate::functions;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Login(action: Action<functions::auth::Login, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <Meta property="og:title" content="Login"/>
        <Title text="Login"/>
        <Meta name="description" content="Login to the site"/>
        <Meta property="og:description" content="Login to the site"/>
        <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg"/>
        <div class="auth">
            <div class="auth__header">
                <h1 class="auth__heading">"Login"</h1>
            </div>
            <div class="auth__body content">
                <ActionForm id="auth__login-form" action=action>
                    <div>
                        <label for="username" class="auth__login-form-label">
                            "Username"
                        </label>
                        <input
                            id="username"
                            required
                            name="username"
                            type="username"
                            aria-describedby="username-error"
                        />
                    </div>
                    <div>
                        <label for="password" class="auth__login-form-label">
                            "Password"
                        </label>
                        <input
                            id="password"
                            name="password"
                            type="password"
                            autoComplete="current-password"
                            aria-describedby="password-error"
                        />
                    </div>
                    <button type="submit" id="auth__login-form-submit">
                        "Log in"
                    </button>

                    <div id="auth__form-aside">
                        <div class="auth__login-form_aside_row">
                            <input id="remember" name="remember" type="checkbox"/>
                            <label for="remember">"Remember me"</label>
                        </div>
                        <div class="auth__form_aside_row">
                            "Don't have an account?" <a href="">"Sign up"</a>
                        </div>
                    </div>

                </ActionForm>
            </div>
        </div>
    }
}
