use std::borrow::Cow;

use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use argon2::{
        password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
        Argon2,
    };
    use crate::functions::{con};
    use crate::models::User;
    use crate::errors::BenwisAppError;
    use rand_core::OsRng;
    use spin_sdk::sqlite::{Connection, Value::{Text, Integer}};
    use std::sync::Arc;
    use async_session::{Session, SessionStore};
    use crate::session::{SqliteStore};
    use leptos_spin::ResponseOptions;
    use cookie::Cookie;
    use leptos_spin::RequestParts;
    
    /// Hash Argon2 password
    pub fn hash_password(password: &[u8]) -> Result<String, BenwisAppError> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2.hash_password(password, &salt)?.to_string();
        Ok(password_hash)
    }
    /// Verify Password
    pub fn verify_password(password: &str, stored_password_hash: &str) -> Result<(), BenwisAppError> {
        let argon2 = Argon2::default();
        // Verify password against PHC string
        let parsed_hash = PasswordHash::new(password)?;
        Ok(argon2.verify_password(stored_password_hash.as_bytes(), &parsed_hash)?)
    }

    /// Verify the user is who they say they are
    pub async fn auth_user(name: &str, password: &str, con: &Arc<Connection>) -> Result<User, BenwisAppError>{
        // Does the user exist
        let Ok(Some(user)) = User::get_from_username(name, con).await else{
            return Err(BenwisAppError::AuthError);
        };

        // Check that password is correct
        match verify_password(password, &user.password){
            Ok(_) => Ok(user),
            Err(_) => Err(BenwisAppError::AuthError),
        }
    }
    pub fn get_session_cookie_value(req_parts: &RequestParts)-> Result<Option<String>, BenwisAppError>{

    let cookies: Vec<(&String, Cow<'_, str>)> = req_parts
        .headers()
        .iter()
        .filter(|(k, _v)| k == "Cookie")
        .map(|(k, v)| (k, String::from_utf8_lossy(v)))
        .collect();
    let cookie_string = cookies.first().map(|(k, v)| v);
    let cookie_jar = match cookie_string {
        Some(c) => Cookie::split_parse_encoded(c.clone()),
        None => return Err(BenwisAppError::AuthError),
    };

    let mut session_val = None;
    for cookie in cookie_jar.into_iter(){
        if let Ok(c) = cookie {
            if c.name() == "benwis_session" {
                session_val = Some(c.clone().value().to_owned());
                break;
            }
        }
    };
    Ok(session_val)
    }

    pub async fn auth_session(req_parts: &RequestParts, con: &Arc<Connection>)-> Result<User, BenwisAppError>{
    
    let store = expect_context::<SqliteStore>();
    let session_val = match get_session_cookie_value(req_parts)?{
    Some(sv) => sv,
    None => return Err(BenwisAppError::AuthError),
    };

    let Some(session) = store.load_session(session_val).await? else{
        return Err(BenwisAppError::InternalServerError);
    }; 
    let Some(user_id) = session.get("user_id") else{
        return Err(BenwisAppError::AuthError);
    };

    let user = match User::get(user_id, con).await?{
    Some(u) => u,
    None => return Err(BenwisAppError::AuthError)
    };  
    Ok(user)
    }

    /// Create a new Session and store User id in it
    pub async fn create_session(user_id: i64)-> Result<String, BenwisAppError>{
        let mut session = Session::new();
        session.insert("user_id", user_id)?;

        let session_store = expect_context::<SqliteStore>();
        let cookie_value = session_store.store_session(session).await?.unwrap();
        Ok(cookie_value)
    }

    /// Destroy the Session if it exists
    pub async fn logout_session(cookie_value: &str)-> Result<(), BenwisAppError>{
        let store = expect_context::<SqliteStore>();
        let session = match store.load_session(cookie_value.to_string()).await?{
            Some(s) =>s,
            None => return Ok(())
        };
        store.destroy_session(session).await?;
        Ok(())
    }
}
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(Login, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let Some(req) = use_context::<leptos_spin::RequestParts>() else {
        return Ok(());
    };
    let con = con()?;

    let user = auth_user(&username, &password, &con).await?;
    let session_cookie = create_session(user.id).await?;

    let res_options = expect_context::<ResponseOptions>();
    res_options.insert_header(
        "Set-Cookie",
        format!("benwis_session={session_cookie}").as_bytes(),
    );

    leptos_spin::redirect("/");
    Ok(())
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(Signup, "/api")]
pub async fn signup(
    username: String,
    display_name: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let Some(req) = use_context::<leptos_spin::RequestParts>() else {
        return Ok(());
    };
    let con = con()?;

    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords did not match.".to_string(),
        ));
    }
    // Don't want anyone signing up but me!
    if username != "benwis" {
        leptos_spin::redirect("/nedry");
        return Ok(());
    }

    let password_hashed = hash_password(password.as_bytes()).unwrap();
    con.execute(
        "INSERT INTO users (username, display_name, password) VALUES (?,?, ?)",
        &[
            Text(username.to_string()),
            Text(display_name.to_string()),
            Text(password_hashed.to_string()),
        ],
    )
    .map_err(|e| ServerFnError::<BenwisAppError>::ServerError(e.to_string()))?;

    let user = match User::get_from_username(&username, &con)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
    {
        Some(u) => u,
        None => return Err(BenwisAppError::AuthError.into()),
    };

    leptos_spin::redirect("/");

    Ok(())
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    let Some(req) = use_context::<leptos_spin::RequestParts>() else {
        return Ok(());
    };
    let con = con()?;
    let Some(session) = get_session_cookie_value(&req)? else{
        return Ok(());
    };
    logout_session(&session).await?;
    leptos_spin::redirect("/");

    Ok(())
}
