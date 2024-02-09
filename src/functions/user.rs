use crate::models::{SafeUser, User};
use leptos::*;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::functions::{con};
    }
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(GetUser, "/api")]
/// Get the current user if it exists by checking the user's session against the DB
pub async fn get_user() -> Result<Option<User>, ServerFnError> {

    let Some(req) = use_context::<leptos_spin::RequestParts>() else{
        return Err(ServerFnError::MissingArg("Missing Request".to_string()));
    };
    let con = con()?;

    // Redirect all non logged in users to Nedry!
    let Ok(identity) = identity(&req) else{
        leptos_spin::redirect("/nedry");
        return Err(ServerFnError::ServerError("Only users are allowed to post!".to_string()))
    };
    let id: i64 = identity.id().map_err(|e| ServerFnError::ServerError("User Not Found!".to_string()))?.parse().map_err(|_|ServerFnError::ServerError("Failed to convert String to Int".to_string()))?;

    let user = User::get(id, &con).await;
    Ok(user?)
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(GetSafeUser, "/api")]
/// Get the current user if it exists by checking the user's session against the DB
pub async fn get_safe_user() -> Result<Option<SafeUser>, ServerFnError> {
    let Some(req) = use_context::<leptos_spin::RequestParts>() else{
        return Err(ServerFnError::MissingArg("Missing Request".to_string()));
    };
    let con = con()?;

    // Redirect all non logged in users to Nedry!
    let Ok(identity) = identity(&req) else{
        leptos_spin::redirect("/nedry");
        return Err(ServerFnError::ServerError("Ah Ah Ah, You didn't say the magic word!".to_string()))
    };
    let id: i64 = identity.id().map_err(|e| ServerFnError::ServerError("User Not Found!".to_string()))?.parse().map_err(|_| ServerFnError::ServerError("Failed to convert String to Int".to_string()))?;

    let safe_user = SafeUser::get(id, &con).await;
    Ok(safe_user?)
}
