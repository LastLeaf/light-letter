#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
#[cfg(not(target_arch = "wasm32"))]
#[macro_use] extern crate diesel;
#[cfg(not(target_arch = "wasm32"))]
#[macro_use] extern crate diesel_migrations;
use std::fmt;

#[cfg(not(target_arch = "wasm32"))]
pub mod db;
#[cfg(not(target_arch = "wasm32"))]
mod schema;
#[cfg(not(target_arch = "wasm32"))]
mod models;
#[cfg(not(target_arch = "wasm32"))]
pub mod sites_config;
#[cfg(not(target_arch = "wasm32"))]
pub use sites_config::*;
#[cfg(not(target_arch = "wasm32"))]
pub mod site_state;
#[cfg(not(target_arch = "wasm32"))]
pub use site_state::*;
pub mod backstage;

#[derive(Debug)]
pub enum RpcError {
    InternalError(String),
    ParseError(String),
    NoSuchRoute,
}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RpcError::InternalError(x) => write!(f, "{}", x),
            RpcError::ParseError(x) => write!(f, "{}", x),
            RpcError::NoSuchRoute => write!(f, ""),
        }
    }
}

impl std::error::Error for RpcError {}

macro_rules! rpc_route {
    ($($route:expr => $func:expr;)*) => {
        #[cfg(not(target_arch = "wasm32"))]
        pub async fn rpc_route(path: String, site_state: &'static SiteState, req: String) -> Result<String, RpcError> {
            let ret = match path.as_str() {
                $($route => {
                    let req = serde_json::from_reader(req.as_str().as_bytes()).map_err(|x| RpcError::ParseError(x.to_string()))?;
                    let resp = ($func)(site_state, req).await?;
                    serde_json::to_string(&resp).map_err(|x| RpcError::ParseError(x.to_string()))
                },)*
                _ => {
                    Err(RpcError::NoSuchRoute)
                }
            };
            if ret.is_ok() {
                debug!("Rpc request to {:?} finished", path);
            } else {
                warn!("Rpc request to {:?} no route found", path);
            }
            ret
        }
    };
}

rpc_route! {
    "/backstage/login" => backstage::login::login;
}
