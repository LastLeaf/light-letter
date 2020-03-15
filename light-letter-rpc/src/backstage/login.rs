use sha2::{Sha256, Digest};
#[cfg(not(target_arch = "wasm32"))]
use diesel::prelude::*;

use crate::RpcError;
#[cfg(not(target_arch = "wasm32"))]
use crate::SiteState;
#[cfg(not(target_arch = "wasm32"))]
use crate::models::User;
#[cfg(not(target_arch = "wasm32"))]
use crate::schema::users::dsl::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginReq {
    pub account: String,
    pub pwd: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum LoginResp {
    Success,
    NoSuchAccount,
    WrongPassword,
}

const PWD_SALT: &'static str = "_light_letter";

#[cfg(not(target_arch = "wasm32"))]
pub async fn login(site_state: &'static SiteState, req: LoginReq) -> Result<LoginResp, RpcError> {
    let db = site_state.db()?;
    let p: Option<String> = users
        .select(pwd)
        .filter(id.eq(&req.account))
        .first(&db)
        .optional()
        .map_err(|x| RpcError::InternalError(x.to_string()))?;
    let ret = match p {
        None => LoginResp::NoSuchAccount,
        Some(p) => {
            let mut s = Sha256::new();
            s.input((req.pwd + PWD_SALT).as_bytes());
            let s = format!("{:x?}", s.result());
            if s != p {
                LoginResp::WrongPassword
            } else {
                LoginResp::Success
            }
        },
    };
    Ok(ret)
}
