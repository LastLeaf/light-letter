use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct SitesConfig {
    pub(crate) net: NetConfig,
    pub(crate) site: Vec<SiteConfig>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct NetConfig {
    pub(crate) port: u16,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SiteConfig {
    pub(crate) name: String,
    pub(crate) host: String,
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
