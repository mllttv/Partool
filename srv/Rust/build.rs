use quote::{format_ident, quote};
use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() -> Result<(), Box<dyn Error>> {
    for d in ["src/routes", "src/components", "build.rs"] {
        println!("cargo:rerun-if-changed={d}");
    }
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let build_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    println!("cargo:rustc-env=BUILD_TIME={build_time}");

    let (mut stack, mut files) = (vec![PathBuf::from("src/routes")], vec![]);
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(dir) else { continue };
        for p in entries.flatten().map(|e| e.path()) {
            let Some(name) = p.file_name().and_then(|n| n.to_str()) else { continue };
            if p.is_dir() {
                stack.push(p);
            } else if name.ends_with(".rs") && name != "mod.rs" && !name.starts_with(['_', '.']) {
                files.push(p);
            }
        }
    }
    files.sort();

    let (mut mods, mut tags, mut items, mut crons) = (vec![], vec![], vec![], vec![]);
    for path in files {
        let rel = path.strip_prefix("src/routes")?.to_string_lossy().replace('\\', "/");
        let (route, mod_id) = (format!("/{rel}"), format_ident!("route_{}", rel.replace(['/', '.', '-'], "_")));
        let abs_path = path.canonicalize()?.display().to_string();

        let content = fs::read_to_string(&path)?;
        let has_page_view = content.contains("pub fn PageView");
        let cfg = if has_page_view || content.contains("#[component]") { quote!() } else { quote!(#[cfg(feature = "ssr")]) };
        mods.push(quote! { #cfg #[path = #abs_path] pub mod #mod_id; });

        if path.file_name().is_some_and(|n| n == "cron.rs") {
            crons.push(quote! { #mod_id::run().await; });
        } else if has_page_view {
            items.push(route.clone());
            if route == "/index.rs" {
                tags.push(quote! { <Route path=path!("/") view=#mod_id::PageView/> });
            }
            if let Some(svc) = route.strip_suffix("/ent.rs") {
                let svc_slash = format!("{svc}/");
                tags.push(quote! { <Route path=path!(#svc) view=#mod_id::PageView/> });
                tags.push(quote! { <Route path=path!(#svc_slash) view=#mod_id::PageView/> });
            }
            tags.push(quote! { <Route path=path!(#route) view=#mod_id::PageView/> });
        }
    }

    let code = quote! {
        #[allow(unused_imports)]
        use leptos::prelude::*;
        use leptos_router::{components::{Route, Routes}, path};

        #(#mods)*

        #[component]
        pub fn DynamicRoutes() -> impl IntoView {
            view! {
                <Routes fallback=crate::app::NotFound>
                    #(#tags)*
                </Routes>
            }
        }

        pub fn get_available_routes() -> &'static [&'static str] { &[ #(#items),* ] }

        #[cfg(feature = "ssr")]
        pub async fn run_all_crons() {
            if crate::components::keys::setting().account_activate != 0 {
                crate::components::account::cron::run();
            }
            #(#crons)*
        }
    };
    fs::write(out_dir.join("dynamic_routes.rs"), code.to_string())?;

    let sass = if Command::new("dart-sass").arg("--version").output().is_ok() { "dart-sass" } else { "sass" };
    for name in ["bekdor", "widget", "annoframe"] {
        let (src, out) = (format!("src/components/{name}/style.sass"), out_dir.join(format!("{name}_style.css")));
        if Path::new(&src).exists() && !Command::new(sass).args([src.as_str(), out.to_str().ok_or("Invalid path")?, "--no-source-map", "--style=compressed"]).status()?.success() {
            return Err(format!("Sass compile failed for {src}").into());
        }
    }
    Ok(())
}
