use leptos::{prelude::*, task};
use leptos_router::components::Router;

use crate::{
    DynamicRoutes,
    components::{
        account::core::{self, CurrentUserInfo},
        annoframe::Frame,
        widget::{Footer, Header},
    },
};

#[cfg(feature = "ssr")]
use {axum::http::request::Parts, leptos_meta::MetaTags};

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="alert alert-warning my-4">
            <h4 class="alert-heading">"404 Not Found"</h4>
            <p class="mb-0">"The requested route does not exist or has not mounted a PageView."</p>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    leptos_meta::provide_meta_context();
    let user = use_context::<RwSignal<Option<CurrentUserInfo>>>().unwrap_or_else(|| RwSignal::new(None));
    provide_context(user);

    let user_res = Resource::new(|| (), |_| async move { core::get_me_action().await.ok() });

    Effect::new(move |_| {
        let Some(res) = user_res.get() else { return };
        if res.is_some() {
            user.set(res);
        } else if user.with_untracked(|u| u.is_none()) {
            task::spawn_local(async move {
                let Ok(guest) = core::guest_action().await else { return };
                if user.with_untracked(|u| u.is_none()) {
                    user.set(Some(guest));
                }
            });
        }
    });

    view! {
        <Router>
            <Header />
            <Frame>
                <main id="main-content" class="container-fluid px-3 px-md-4 px-xl-5 my-4 flex-grow-1">
                    <DynamicRoutes />
                </main>
            </Frame>
            <Footer />
        </Router>
    }
}

#[cfg(feature = "ssr")]
const WIDGET_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/widget_style.css"));
#[cfg(feature = "ssr")]
const ANNOFRAME_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/annoframe_style.css"));

#[cfg(feature = "ssr")]
#[component]
pub fn Shell() -> impl IntoView {
    let user = use_context::<Parts>().as_ref().and_then(|p| core::get_current_user(&p.headers));
    provide_context(RwSignal::new(user));

    let wasm_init = format!("import init, {{ hydrate }} from '/pkg/htmlvm.js?v={0}'; init('/pkg/htmlvm_bg.wasm?v={0}').then(hydrate).catch(console.error);", env!("BUILD_TIME"),);

    view! {
        <!DOCTYPE html>
        <html lang="en" data-bs-theme="dark">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"HTMLVM"</title>
                <MetaTags />
                <link rel="stylesheet" href="/static/node_modules/bootstrap/dist/css/bootstrap.min.css" />
                <style>{WIDGET_CSS}{ANNOFRAME_CSS}</style>
            </head>
            <body class="d-flex flex-column min-vh-100">
                <App />
                <script src="/static/node_modules/bootstrap/dist/js/bootstrap.bundle.min.js"></script>
                <script type="module">{wasm_init}</script>
            </body>
        </html>
    }
}
