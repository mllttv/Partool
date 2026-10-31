// Dummy implementation for static file server

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use {
    axum::http::HeaderMap,
    rusqlite::Connection,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CurrentUserInfo {
    pub is_logged_in: bool,
    pub username: String,
    pub role: i64,
    pub token: String,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub role: i64,
    pub iat: usize,
    pub st_iat: Option<usize>,
}

#[cfg(feature = "ssr")]
#[allow(dead_code)]
impl Claims {
    pub fn user_id(&self) -> Result<i64, ServerFnError> {
        Ok(0)
    }

    pub fn require_registered(&self) -> Result<i64, ServerFnError> {
        Err(ServerFnError::new("Account service is disabled"))
    }

    pub fn ensure_active(&self) -> Result<(), ServerFnError> {
        Ok(())
    }

    pub(crate) fn issue(&self, _headers: &HeaderMap) -> Result<CurrentUserInfo, ServerFnError> {
        Ok(CurrentUserInfo::default())
    }
}

#[cfg(feature = "ssr")]
pub fn is_https(_headers: &HeaderMap) -> bool {
    false
}

#[cfg(feature = "ssr")]
#[allow(dead_code)]
pub(crate) fn check_access(_role: Option<i64>, _msg: &'static str) -> Result<(), ServerFnError> {
    Ok(())
}

#[server(prefix = "/api", endpoint = "account_guest")]
pub async fn guest_action() -> Result<CurrentUserInfo, ServerFnError> {
    Ok(CurrentUserInfo::default())
}

#[server(prefix = "/api", endpoint = "account_login")]
pub async fn login_action(_username: String, _password: String) -> Result<CurrentUserInfo, ServerFnError> {
    Err(ServerFnError::new("Account service is disabled"))
}

#[server(prefix = "/api", endpoint = "account_register")]
pub async fn register_action(_username: Option<String>, _password: Option<String>, _mail: Option<String>) -> Result<CurrentUserInfo, ServerFnError> {
    Err(ServerFnError::new("Account service is disabled"))
}

#[server(prefix = "/api", endpoint = "account_update")]
pub async fn update_user_action(_username: Option<String>, _mail: Option<String>, _password: Option<String>) -> Result<CurrentUserInfo, ServerFnError> {
    Err(ServerFnError::new("Account service is disabled"))
}

#[server(prefix = "/api", endpoint = "account_me")]
pub async fn get_me_action() -> Result<CurrentUserInfo, ServerFnError> {
    Ok(CurrentUserInfo::default())
}

#[server(prefix = "/api", endpoint = "account_logout")]
pub async fn logout_action() -> Result<CurrentUserInfo, ServerFnError> {
    Ok(CurrentUserInfo::default())
}

#[server(prefix = "/api", endpoint = "account_delete")]
pub async fn delete_user_action() -> Result<(), ServerFnError> {
    Err(ServerFnError::new("Account service is disabled"))
}

#[cfg(feature = "ssr")]
pub fn jwt_encode<T: Serialize>(_claims: &T) -> Result<String, ServerFnError> {
    Ok(String::new())
}

#[cfg(feature = "ssr")]
pub fn jwt_decode<T: for<'de> Deserialize<'de> + Default>(_headers: &HeaderMap, _target: &str) -> Result<T, ServerFnError> {
    Ok(T::default())
}

#[cfg(feature = "ssr")]
pub fn get_current_user(_headers: &HeaderMap) -> Option<CurrentUserInfo> {
    None
}

#[cfg(feature = "ssr")]
pub fn init_db(_conn: &Connection) -> rusqlite::Result<()> {
    Ok(())
}
