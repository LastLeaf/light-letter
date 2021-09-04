use std::path::{Path, PathBuf};
use diesel::{r2d2, pg::PgConnection};
use regex::Regex;

use crate::{db, RpcError};

pub struct SiteState {
    pub initialization_time: chrono::DateTime<chrono::Utc>,
    pub host: String,
    pub host_aliases: Vec<String>,
    pub dir: PathBuf,
    pub theme_mod_name: String,
    pub theme_dir: PathBuf,
    db_pool: Option<db::DbPool>,
    pub config: crate::SiteConfig,
}

impl SiteState {
    pub fn from_sites_config(config: &crate::SitesConfig, sites_root: &Path) -> Vec<Self> {
        let sites_root = sites_root.to_owned();

        // create one db pool for one site
        let pools = db::Db::new(config).into_pools();

        // check each site
        config.site.iter().zip(pools).map(|(site_config, db_pool)| {

            // validate site name
            lazy_static! {
                static ref NAME_RE: Regex = Regex::new(r#"^[-_0-9a-zA-Z]+$"#).unwrap();
            }
            if !NAME_RE.is_match(site_config.name.as_str()) {
                panic!(r#"Illegal site name "{}" (site name should only contains letters, numbers, dashes, and underlines)."#, site_config.name)
            }

            // create dir structure
            let dir = sites_root.join("sites").join(site_config.name.as_str());
            fs_extra::dir::create_all(dir.as_path(), false).unwrap();
            let site_type = site_config.r#type.clone();
            match site_type.as_str() {
                "blog" => {
                    fs_extra::dir::create_all(dir.join("files").as_path(), false).unwrap();
                },
                "static" => {
                    fs_extra::dir::create_all(dir.join("static").as_path(), false).unwrap();
                },
                _ => panic!(r#"Unrecognized site type "{}"."#, &site_type)
            };

            // find theme dir
            let theme_mod_name = site_config.theme.clone().unwrap_or(String::new()).replace('-', "_").to_string();
            let theme_dir = match site_type.as_str() {
                "blog" => {
                    if let Some(theme) = &site_config.theme {
                        if let Some(theme_dir) = config.resource.themes.get(theme) {
                            PathBuf::from(theme_dir)
                        } else {
                            panic!("Theme {:?} is not defined for site {:?}", theme, site_config.name);
                        }
                    } else {
                        panic!("Theme is not specified for site {:?}", site_config.name);
                    }
                },
                _ => PathBuf::new(),
            };

            // pick out host name and alias
            debug!(r#"Serve site {} for host "{}", aliases {:?}"#, site_config.name, site_config.host, site_config.alias.as_ref().unwrap_or(&vec![]));
            let host = site_config.host.clone();
            let host_aliases = site_config.alias.as_ref().unwrap_or(&vec![]).iter().map(|x| x.clone()).collect();

            let site_state = SiteState {
                initialization_time: chrono::Utc::now(),
                host,
                host_aliases,
                dir,
                theme_mod_name,
                theme_dir,
                db_pool,
                config: site_config.clone(),
            };
            site_state
        }).collect()
    }

    pub(crate) fn db(&self) -> Result<r2d2::PooledConnection<r2d2::ConnectionManager<PgConnection>>, RpcError> {
        match self.db_pool.as_ref().and_then(|x| match x.pool.get() {
            Ok(x) => Some(x),
            Err(_) => None,
        }) {
            None => Err(RpcError::InternalError("Fail connecting to database".into())),
            Some(x) => Ok(x),
        }
    }
}
