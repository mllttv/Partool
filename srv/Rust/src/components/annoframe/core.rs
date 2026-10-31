// Dummy implementation for static file server

use leptos::prelude::*;
use server_fn::codec::{ByteStream, GetUrl, Streaming};

#[server(prefix = "/api", endpoint = "annoframe_asset", input = GetUrl, output = Streaming)]
async fn get_asset(_path: String) -> Result<ByteStream, ServerFnError> {
    Ok(ByteStream::new(futures::stream::empty::<Result<Vec<u8>, ServerFnError>>()))
}

pub fn play(_path: &str) {
    // No-op for static file server
}

#[component]
pub fn Frame(children: Children) -> impl IntoView {
    view! {
        <div class="annoframe-content w-100 flex-grow-1 d-flex flex-column">
            {children()}
        </div>
    }
}
