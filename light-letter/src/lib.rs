#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tokio::runtime::{Runtime, Builder};

mod themes;
mod http;

/// The server builder object
pub struct SitesServerBuilder {
    sites_root: PathBuf,
    tokio_runtime: Runtime,
    http_server: http::Server,
    close_handler: http::CloseHandler,
}

impl SitesServerBuilder {
    /// Get the sites root dir
    pub fn sites_root(&self) -> &Path {
        &self.sites_root
    }

    /// Get the sites root dir
    pub fn addrs(&self) -> &Vec<SocketAddr> {
        self.http_server.addrs()
    }

    /// Start service
    pub fn run(self) {
        let Self {mut tokio_runtime, http_server, ..} = self;
        tokio_runtime.block_on(http_server.run_async());
    }

    /// Start service and execute another fn in separated thread
    pub fn run_with_thread<F>(self, f: F) where F: FnOnce(SitesServer) + Send + 'static {
        let Self {sites_root, mut tokio_runtime, http_server, close_handler} = self;
        std::thread::spawn(move || {
            f(SitesServer {
                sites_root,
                close_handler,
            });
        });
        tokio_runtime.block_on(http_server.run_async());
    }

    /// Start service in another thread
    pub fn run_in_thread(self) -> SitesServer {
        let Self {sites_root, mut tokio_runtime, http_server, close_handler} = self;
        std::thread::spawn(move || {
            tokio_runtime.block_on(http_server.run_async());
        });
        SitesServer {
            sites_root,
            close_handler,
        }
    }
}

/// The server object
pub struct SitesServer {
    sites_root: PathBuf,
    close_handler: http::CloseHandler,
}

impl SitesServer {
    /// Create a new server
    pub fn new(sites_root: PathBuf) -> SitesServerBuilder {
        info!("Initializing light-letter in {}", sites_root.as_path().to_str().unwrap());
        let sites_config = light_letter_rpc::sites_config::read_sites_config(&sites_root);
        themes::init(&sites_config);
        let tokio_runtime = Builder::new()
            .threaded_scheduler()
            .enable_all()
            .thread_name("light-letter")
            .build()
            .unwrap();
        let (http_server, close_handler) = http::Server::new_with_close_handler(&sites_root, &sites_config);
        SitesServerBuilder {
            sites_root,
            tokio_runtime,
            http_server,
            close_handler,
        }
    }

    /// Get the sites root dir
    pub fn sites_root(&self) -> &Path {
        &self.sites_root
    }

    /// Stop server
    pub fn stop(self) {
        self.close_handler.close();
    }
}

/// Create a new server and start it immediately
pub fn start(sites_root: PathBuf) {
    let s = SitesServer::new(sites_root);
    s.run();
}
