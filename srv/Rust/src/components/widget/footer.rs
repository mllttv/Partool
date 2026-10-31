use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-auto py-3 border-top border-secondary border-opacity-25 bg-black bg-opacity-50">
            <div class="container-fluid px-3 px-md-4 d-flex flex-column flex-md-row justify-content-between align-items-center gap-2 small text-secondary">
                <div class="d-flex align-items-center gap-2">
                    <span class="fw-bold text-info font-monospace">"HTMLVM"</span>
                    <span class="font-monospace small text-secondary">"v0.1.0"</span>
                    <span class="d-none d-sm-inline text-secondary">"— Monorepo Architecture for Static Assets & Dynamic FaaS"</span>
                </div>
                <div class="d-flex align-items-center gap-2">
                    <span class="badge badge-core font-monospace">"Native FHS"</span>
                    <span class="badge badge-user font-monospace">"Axum & Leptos SSR"</span>
                    <span class="badge badge-guest font-monospace">"UTC Engine"</span>
                </div>
            </div>
        </footer>
    }
}
