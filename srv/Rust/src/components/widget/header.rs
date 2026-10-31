use leptos::prelude::*;
use leptos_router::hooks::use_location;

use super::header_user::HeaderUser;
use crate::get_available_routes;

fn resolve_service(path: &str) -> (&'static str, String, &'static str, String, &'static str) {
    let clean = path.trim_start_matches('/');
    match clean.split('/').next().unwrap_or("") {
        "" | "index.rs" => ("⚡", "HTMLVM".into(), "// FHS Monolith", "/".into(), "text-light"),
        "bekdor" => ("🛡️", "Bekdor".into(), "// Axum Console", "/bekdor".into(), "text-warning"),
        s => ("📦", s.to_uppercase(), "// FaaS Route", format!("/{s}/ent.rs"), "text-light"),
    }
}

#[component]
pub fn Header() -> impl IntoView {
    let location = use_location();
    let current_path = Signal::derive(move || location.pathname.get());
    let current_svc = Signal::derive(move || current_path.with(|p| p.trim_start_matches('/').split('/').next().unwrap_or("").to_string()));
    let (user_section, auth_modal) = HeaderUser();

    let menu_items = {
        let services: Vec<_> = get_available_routes()
            .iter()
            .filter_map(|&r| {
                let s = r.strip_prefix('/')?.strip_suffix("/ent.rs")?;
                (!s.is_empty() && !s.contains('/')).then_some((s, r))
            })
            .collect();

        if services.is_empty() {
            view! {
                <li>
                    <span class="dropdown-item text-secondary fst-italic small">"No extra services mounted"</span>
                </li>
            }
            .into_any()
        } else {
            services
                .into_iter()
                .map(|(svc, r)| {
                    view! {
                        <li>
                            <a class="dropdown-item d-flex align-items-center gap-2" class:active=move || current_svc.get() == svc href=r>
                                <span>"📦"</span>
                                <span class="font-monospace small text-uppercase">{svc}</span>
                            </a>
                        </li>
                    }
                })
                .collect_view()
                .into_any()
        }
    };

    let service_dropdown = view! {
        <div class="dropdown">
            <button
                class="btn btn-sm btn-outline-info dropdown-toggle d-flex align-items-center gap-1 shadow-none"
                type="button"
                id="serviceDropdown"
                data-bs-toggle="dropdown"
                aria-expanded="false"
                title="Switch Service"
            >
                <span class="fs-6">"⚡"</span>
            </button>
            <ul class="dropdown-menu dropdown-menu-dark shadow border-secondary mt-2" aria-labelledby="serviceDropdown">
                <li>
                    <h6 class="dropdown-header font-monospace small text-secondary">"SWITCH SERVICE"</h6>
                </li>
                <li>
                    <a
                        class="dropdown-item d-flex align-items-center gap-2"
                        class:active=move || matches!(current_path.get().as_str(), "/" | "/index.rs")
                        href="/"
                    >
                        <span>"⚡"</span>
                        <span>"HTMLVM Portal"</span>
                    </a>
                </li>
                <li>
                    <hr class="dropdown-divider border-secondary border-opacity-50" />
                </li>
                {menu_items}
            </ul>
        </div>
    };

    let brand_link = move || {
        let (icon, title, sub, href, cls) = resolve_service(&current_path.get());
        view! {
            <a class="navbar-brand d-flex align-items-center gap-2 m-0 text-decoration-none" href=href>
                <span class="fs-5">{icon}</span>
                <div class="d-flex align-items-baseline gap-1">
                    <span class=format!("fw-bold tracking-wide {cls}")>{title}</span>
                    <span class="small font-monospace text-info d-none d-sm-inline">{sub}</span>
                </div>
            </a>
        }
    };

    view! {
        <header class="header-navbar sticky-top">
            <nav class="navbar navbar-dark navbar-htmlvm py-2">
                <div class="container-fluid px-3 px-md-4 d-flex justify-content-between align-items-center flex-wrap gap-2">
                    <div class="d-flex align-items-center gap-3">
                        {service_dropdown} <div class="vr bg-secondary opacity-50 d-none d-sm-block my-1" /> {brand_link}
                    </div>
                    {user_section}
                </div>
            </nav>
        </header>
        {auth_modal}
    }
}
