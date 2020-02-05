use std::net::IpAddr;
use std::path::Path;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct SitesConfig {
    pub(crate) net: NetConfig,
    pub(crate) db: Option<DbConfig>,
    pub(crate) site: Vec<SiteConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct NetConfig {
    pub(crate) ip: IpAddr,
    pub(crate) port: Vec<u16>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct DbConfig {
    pub(crate) host: Option<String>,
    pub(crate) port: Option<u16>,
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct SiteConfig {
    pub(crate) name: String,
    pub(crate) r#type: String,
    pub(crate) database: Option<String>,
    pub(crate) host: String,
    pub(crate) alias: Option<Vec<String>>,
}

pub(crate) fn read_sites_config(sites_root: &Path) -> SitesConfig {
    let config_file = sites_root.join("config.toml");
    let config_str = std::fs::read_to_string(&config_file).unwrap_or_else(|_| {
        panic!(r#"No config.toml found in "{}"."#, sites_root.to_str().unwrap_or_default())
    });
    let config = toml::from_str(&config_str).unwrap_or_else(|e| {
        panic!(format!("{}", e))
    });
    config
}
