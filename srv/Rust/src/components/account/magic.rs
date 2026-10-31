// Dummy implementation for static file server

use leptos::prelude::*;

#[server(prefix = "/api", endpoint = "account_send_mail_magiclink")]
pub async fn send_link(_identifier: String) -> Result<(), ServerFnError> {
    Err(ServerFnError::new("Account service is disabled"))
}

#[server(prefix = "/api", endpoint = "account_magiclogin", input = server_fn::codec::GetUrl)]
pub async fn login(_token: String) -> Result<(), ServerFnError> {
    Err(ServerFnError::new("Account service is disabled"))
}
