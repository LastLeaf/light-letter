use std::net::IpAddr;
use std::path::Path;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::Deserialize;

lazy_static! {
    static ref SECURE_RANDOM_STRING_ARC: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}

thread_local! {
    pub(crate) static SECURE_RANDOM_STRING: String = {
        SECURE_RANDOM_STRING_ARC.lock().unwrap().clone()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResourceConfig {
    pub backstage: String,
    pub themes: HashMap<String, String>,
    pub secure_random_string: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SitesConfig {
    pub resource: ResourceConfig,
    pub net: NetConfig,
    pub db: Option<DbConfig>,
    pub site: Vec<SiteConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NetConfig {
    pub ip: IpAddr,
    pub port: Vec<u16>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DbConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SiteConfig {
    pub name: String,
    pub r#type: String,
    pub database: Option<String>,
    pub host: String,
    pub alias: Option<Vec<String>>,
    pub theme: Option<String>,
}

pub fn read_sites_config(sites_root: &Path) -> SitesConfig {
    let config_file = sites_root.join("config.toml");
    let config_str = std::fs::read_to_string(&config_file).unwrap_or_else(|_| {
        panic!(r#"No config.toml found in "{}"."#, sites_root.to_str().unwrap_or_default())
    });
    let config: SitesConfig = toml::from_str(&config_str).unwrap_or_else(|e| {
        panic!("{}", e)
    });
    *SECURE_RANDOM_STRING_ARC.lock().unwrap() = config.resource.secure_random_string.clone().unwrap_or_default();
    config
}
