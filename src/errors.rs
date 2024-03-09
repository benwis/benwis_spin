use http::status::StatusCode;
//use leptos::ServerFnError;
//use leptos::{server_fn::server_fn_error, ServerFnError};
use cfg_if::cfg_if;
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use thiserror::Error;

#[derive(Debug, Clone, Error, Diagnostic, Serialize, Deserialize, EnumString)]
pub enum BenwisAppError {
    #[error("Not Found")]
    NotFound,
    #[error("Auth Error")]
    AuthError,
    #[error("DB Connection Not Found")]
    DBConnectionNotFound,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Server Error: {0}")]
    ServerError(String),
    #[error("TomlError: {0}")]
    TomlError(String),
    #[error("Argon2Error: {0}")]
    Argon2Error(String),
    #[error("CompilationError: {0}")]
    CompilationError(String),
    #[error("SessionError: {0}")]
    SessionError(String),
    #[error("JsonError: {0}")]
    JsonError(String),
    #[error("DBError: {0}")]
    DBError(String),
    #[error("Invalid Date or Time")]
    InvalidDateTime,
    #[error("Missing or Invalid Frontmatter")]
    MissingOrInvalidFrontmatter,
    #[error("Bad Request: {0}")]
    BadRequest(String),
}

impl BenwisAppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            BenwisAppError::NotFound => StatusCode::NOT_FOUND,
            BenwisAppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::Argon2Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::CompilationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::SessionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::JsonError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::DBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::DBConnectionNotFound => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::InvalidDateTime => StatusCode::BAD_REQUEST,
            BenwisAppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            BenwisAppError::AuthError => StatusCode::BAD_REQUEST,
            BenwisAppError::MissingOrInvalidFrontmatter => StatusCode::INTERNAL_SERVER_ERROR,
            BenwisAppError::TomlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

cfg_if! {
    if #[cfg(not(feature = "ssr"))] {
impl From<serde_wasm_bindgen::Error> for BenwisAppError {
    fn from(error: serde_wasm_bindgen::Error) -> Self {
        Self::CompilationError(error.to_string())
    }
}
}
}
//impl From<ServerFnError> for ServerFnError<BenwisAppError> {
//    fn from(err: BenwisAppError) -> Self {
//        server_fn_error!(err)
//    }
//}
//
cfg_if! {
    if #[cfg(feature = "ssr")] {
        impl From<argon2::password_hash::Error> for BenwisAppError {
            fn from(error: argon2::password_hash::Error) -> Self {
                Self::Argon2Error(error.to_string())
            }
        }
        impl From<spin_sdk::sqlite::Error> for BenwisAppError{
            fn from(error: spin_sdk::sqlite::Error) -> Self {
                Self::DBError(error.to_string())
            }
        }
        impl From<async_session::Error> for BenwisAppError{
            fn from(error: async_session::Error) -> Self {
                Self::SessionError(error.to_string())
            }
        }
        impl From<serde_json::Error> for BenwisAppError{
            fn from(error: serde_json::Error) -> Self {
                Self::JsonError(error.to_string())
            }
        }
    }
}
