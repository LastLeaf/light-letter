use std::net::IpAddr;
use std::path::Path;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ResourceConfig {
    pub web: String,
    pub themes: HashMap<String, String>,
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
    let config = toml::from_str(&config_str).unwrap_or_else(|e| {
        panic!(format!("{}", e))
    });
    config
}
