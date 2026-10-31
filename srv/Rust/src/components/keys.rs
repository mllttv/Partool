use leptos::prelude::ServerFnError;
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::RwLock,
};

use super::var;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingConfig {
    pub jwt: String,
    pub bekdor_activate: i64,
    pub account_activate: i64,
    pub var_GCS_activate: i64,
    pub api_total_mlimit: usize,
    pub api_ip_mlimit: usize,
    pub api_faildelay: u64,
    pub jwt_expired: u64,
    pub status_record_limit: u64,
    pub var_routes_limit: u64,
    pub var_filesize_limit: u64,
    pub mail_smtp_from: String,
    pub mail_smtp_usr: String,
    pub mail_smtp_pass: String,
}

static SETTING: RwLock<Option<SettingConfig>> = RwLock::new(None);

pub fn dir() -> PathBuf {
    var::dir().join("keys")
}

fn atomic_write(dst: &Path, content: impl AsRef<[u8]>) -> io::Result<()> {
    let tmp = dst.with_extension(format!("tmp.{}", rand::rng().random::<u32>()));
    fs::write(&tmp, content)?;
    fs::rename(tmp, dst)
}

fn ensure_and_load() -> SettingConfig {
    let var_keys = dir();
    let _ = fs::create_dir_all(&var_keys);
    let def_dir = ["keys", "Rust/keys", "../Rust/keys", "srv/Rust/keys"]
        .into_iter()
        .find(|p| Path::new(p).join("setting.sample.yaml").exists())
        .map(PathBuf::from)
        .expect("Default keys directory not found");

    for e in fs::read_dir(&def_dir).into_iter().flatten().flatten().filter(|e| e.path().is_file()) {
        let name = e.file_name();
        let target = if name == "setting.sample.yaml" { "setting.yaml" } else { name.to_str().unwrap_or("") };
        let dst = var_keys.join(target);
        if !dst.exists() {
            let _ = fs::read(e.path()).map(|bytes| atomic_write(&dst, bytes));
        }
    }

    let setting_path = var_keys.join("setting.yaml");
    let content = fs::read_to_string(&setting_path).unwrap_or_else(|e| panic!("Failed to read setting.yaml at {setting_path:?}: {e}"));
    let mut config: SettingConfig = serde_yml::from_str(&content).unwrap_or_else(|e| panic!("Failed to parse setting.yaml at {setting_path:?}: {e}"));

    let def_jwt = fs::read_to_string(def_dir.join("setting.sample.yaml"))
        .ok()
        .and_then(|c| serde_yml::from_str::<SettingConfig>(&c).ok())
        .map(|c| c.jwt)
        .unwrap_or_else(|| "makhnovschyna".into());

    if config.jwt == def_jwt || config.jwt == "makhnovschyna" {
        config.jwt = rand::rng().random::<[u8; 32]>().iter().map(|b| format!("{b:02x}")).collect();
        if let Ok(ser) = serde_yml::to_string(&config) {
            let _ = atomic_write(&setting_path, ser);
        }
    }
    config
}

pub fn init() {
    *SETTING.write().unwrap() = Some(ensure_and_load());
}

pub fn setting() -> SettingConfig {
    SETTING.read().unwrap().clone().expect("keys::init must be called before setting")
}

pub fn update_setting(modify: impl FnOnce(&mut SettingConfig)) -> io::Result<()> {
    let mut s = setting();
    modify(&mut s);
    let content = serde_yml::to_string(&s).map_err(io::Error::other)?;
    atomic_write(&dir().join("setting.yaml"), content)?;
    *SETTING.write().unwrap() = Some(s);
    Ok(())
}

pub fn mail_credentials() -> Option<(String, String, String)> {
    None
}

pub async fn check_api_mlimit() -> Result<(), ServerFnError> {
    Ok(())
}
