#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

use std::net::SocketAddr;
use std::path::{Path, PathBuf};

mod sites_config;
use sites_config::*;
mod http;
use http::SiteState;
mod db;
mod schema;
mod models;

/// The server builder object
pub struct SitesServerBuilder {
    sites_root: PathBuf,
    sys: actix_rt::SystemRunner,
    http: http::Server,
}

impl SitesServerBuilder {
    /// Get the sites root dir
    pub fn sites_root(&self) -> &Path {
        &self.sites_root
    }

    /// Get the sites root dir
    pub fn addrs(&self) -> &Vec<SocketAddr> {
        self.http.addrs()
    }

    /// Start service
    pub fn run(self) {
        self.sys.run().unwrap();
    }

    /// Start service in another thread
    pub fn run_with_thread<F>(self, f: F) where F: FnOnce(SitesServer) + Send + 'static {
        let Self {sys, http, sites_root} = self;
        let current_system = actix_rt::System::current();
        std::thread::spawn(move || {
            f(SitesServer {
                sites_root,
                current_system,
                http,
            });
        });
        sys.run().unwrap();
    }
}

/// The server object
pub struct SitesServer {
    sites_root: PathBuf,
    current_system: actix_rt::System,
    http: http::Server,
}

impl SitesServer {
    /// Create a new server
    pub fn new(sites_root: PathBuf) -> SitesServerBuilder {
        info!("Initializing light-letter in {}", sites_root.as_path().to_str().unwrap());
        let config = sites_config::read_sites_config(&sites_root);
        db::Db::new(&config);
        let sys = actix_rt::System::new("light-letter");
        let http = http::Server::new(sites_root.as_path(), &config);
        SitesServerBuilder {
            sites_root,
            sys,
            http,
        }
    }

    /// Get the sites root dir
    pub fn sites_root(&self) -> &Path {
        &self.sites_root
    }

    /// Get the sites root dir
    pub fn addrs(&self) -> &Vec<SocketAddr> {
        self.http.addrs()
    }

    /// Stop server
    pub fn stop(&mut self, graceful: bool) {
        self.http.stop(graceful);
        self.current_system.stop();
    }
}

/// Create a new server and start it immediately
pub fn start(sites_root: PathBuf) {
    let s = SitesServer::new(sites_root);
    s.run();
}
