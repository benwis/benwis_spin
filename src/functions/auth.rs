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
        let parsed_hash = PasswordHash::new(&password)?;
        Ok(argon2.verify_password(stored_password_hash.as_bytes(), &parsed_hash)?)
    }

    /// Verify the user is who they say they are
    pub fn auth_user(name: &str, password: &str, con: &Arc<Connection>){
    let user = User::get_from_username(name, &con);


    }
}
}

#[tracing::instrument(level = "info", fields(error), ret,err)]
#[server(Login, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let Some(req) = use_context::<leptos_spin::RequestParts>() else{
        return Ok(())
    };
    let con = con()?;

    let user = match User::get_from_username(&username, &con)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?{
        Some(u) => u,
        None => return Err(BenwisAppError::AuthError.into()),
        };

    match verify_password(&user.password, &password)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
    {
        Ok(_) => {
            // auth.login_user(user.id);
            // auth.remember_user(remember.is_some());
            Identity::login(&req.extensions(), user.id.to_string()).unwrap();
            leptos_spin::redirect( "/");
            Ok(())
        }
        Err(_) => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}

#[tracing::instrument(level = "info", fields(error), ret,err)]
#[server(Signup, "/api")]
pub async fn signup(
    username: String,
    display_name: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let Some(req) = use_context::<leptos_spin::RequestParts>() else{
        return Ok(())
    };
    let con = con()?;

    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords did not match.".to_string(),
        ));
    }
    // Don't want anyone signing up but me!
    if username != "benwis" {
        leptos_spin::redirect( "/nedry");
        return Ok(());
    }

    let password_hashed = hash_password(password.as_bytes()).unwrap();
    con.execute("INSERT INTO users (username, display_name, password) VALUES (?,?, ?)", &[Text(username.to_string()), Text(display_name.to_string()),Text(password_hashed.to_string())]).map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    let user = match User::get_from_username(&username, &con)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?{
        Some(u) => u,
        None => return Err(BenwisAppError::AuthError.into()),
        };

    Identity::login(&req.extensions(), user.id.to_string()).unwrap();

    // auth.login_user(user.id);
    // auth.remember_user(remember.is_some());

    leptos_spin::redirect( "/");

    Ok(())
}

#[tracing::instrument(level = "info", fields(error), ret,err)]
#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    let Some(req) = use_context::<leptos_spin::RequestParts>() else{
        return Ok(())
    };

    let identity = identity(&req)?;

    identity.logout();
    leptos_spin::redirect("/");

    Ok(())
}
