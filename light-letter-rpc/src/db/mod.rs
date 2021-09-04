use diesel::{Connection, pg::PgConnection, RunQueryDsl, sql_types::Text, r2d2};
use url::Url;
use regex::Regex;
use diesel_derives::*;

#[derive(QueryableByName)]
pub struct Empty ();

#[derive(QueryableByName)]
pub struct PgDatabases {
    #[allow(dead_code)]
    #[sql_type = "Text"]
    datname: String,
}

embed_migrations!("./migrations");

fn pg_connection_string(host: &str, port: Option<u16>, db_name: &str, username: &str, password: &str) -> String {
    let mut u = Url::parse("postgresql://localhost/postgres").unwrap();
    u.set_host(Some(host)).unwrap();
    u.set_port(port).unwrap();
    u.set_path(&format!("/{}", db_name));
    u.set_username(username).unwrap();
    u.set_password(Some(password)).unwrap();
    u.as_str().to_owned()
}

pub(crate) struct DbPool {
    pub(crate) pool: r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
}

pub(crate) struct Db {
    pools: Vec<Option<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>>>,
}

impl Db {
    pub(crate) fn new(sites_config: &crate::SitesConfig) -> Self {
        let pools = match &sites_config.db {
            None => {
                sites_config.site.iter().map(|site_config| {
                    match site_config.r#type.as_str() {
                        "blog" => {
                            panic!("Blog sites need database.");
                        },
                        _ => {
                            None
                        }
                    }
                }).collect()
            },
            Some(db_config) => {
                sites_config.site.iter().map(|site_config| {

                    // get and check db name
                    let db_name = site_config.database.as_ref().map(|x| x.as_str()).unwrap_or(site_config.name.as_str());
                    lazy_static! {
                        static ref DB_NAME_RE: Regex = Regex::new(r#"^[_a-zA-Z][_a-zA-Z0-9]*$"#).unwrap();
                    }
                    if !DB_NAME_RE.is_match(db_name) {
                        panic!(r#"Illegal database name "{}" (database name should be ident-compatible)."#, db_name)
                    }

                    // try create database
                    let u = pg_connection_string(
                        db_config.host.as_ref().unwrap_or(&"localhost".into()),
                        db_config.port.clone(),
                        "postgres",
                        db_config.username.as_str(),
                        db_config.password.as_str(),
                    );
                    let conn = PgConnection::establish(u.as_str()).unwrap_or_else(|e| {
                        panic!("Cannot connect to database server ({})", e)
                    });
                    match diesel::sql_query(format!("SELECT datname FROM pg_database WHERE datname = '{}'", db_name)).load(&conn) {
                        Ok(r) => {
                            let r: Vec<PgDatabases> = r;
                            if r.len() == 0 {
                                match diesel::sql_query(format!("CREATE DATABASE {}", db_name)).load(&conn) {
                                    Ok(r) => {
                                        let _r: Vec<Empty> = r;
                                        debug!(r#"Database "{}" created for site "{}""#, db_name, site_config.name);
                                    },
                                    Err(e) => {
                                        panic!("Cannot create database ({})", e)
                                    }
                                }
                            } else {
                                debug!(r#"Database "{}" already exists for site "{}""#, db_name, site_config.name);
                            }
                        },
                        Err(_) => {
                            // if cannot query info (might be no permission), just continue
                            debug!(r#"Database "{}" not checkable for site "{}""#, db_name, site_config.name);
                        }
                    }

                    match site_config.r#type.as_str() {
                        "blog" => {
                            // init tables
                            let u = pg_connection_string(
                                db_config.host.as_ref().unwrap_or(&"localhost".into()),
                                db_config.port.clone(),
                                db_name,
                                db_config.username.as_str(),
                                db_config.password.as_str(),
                            );
                            let conn = PgConnection::establish(u.as_str()).unwrap_or_else(|e| {
                                panic!("Cannot connect to database ({})", e)
                            });
                            embedded_migrations::run(&conn).unwrap_or_else(|e| {
                                panic!("Cannot update database structure ({})", e)
                            });

                            // init connection pool
                            let connection_manager = r2d2::ConnectionManager::new(u);
                            let pool = r2d2::Pool::new(connection_manager).unwrap();
                            Some(pool)
                        },
                        _ => {
                            None
                        }
                    }

                }).collect()
            },
        };
        Self {
            pools
        }
    }

    pub(crate) fn into_pools(self) -> Vec<Option<DbPool>> {
        self.pools.into_iter().map(|x| {
            x.map(|x| {
                DbPool {
                    pool: x
                }
            })
        }).collect()
    }
}
