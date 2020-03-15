use std::fmt;

use crate::SiteState;

#[derive(Debug)]
pub(crate) struct RpcError {
    message: String
}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RpcError {}

pub(crate) async fn rpc_route(path: String, site_state: &'static SiteState, req: String) -> Result<String, RpcError> {
    // TODO
    unimplemented!()
}
