use sha2::{Sha256, Digest};
use regex::Regex;
use validator::Validate;
#[cfg(not(target_arch = "wasm32"))]
use diesel::prelude::*;

use crate::{RpcError, Session, LoginUser};
#[cfg(not(target_arch = "wasm32"))]
use crate::SiteState;
#[cfg(not(target_arch = "wasm32"))]
use crate::schema::users::dsl::*;

const PWD_SALT: &'static str = "_light_letter";

fn normalize_id(account: &str) -> Option<String> {
    lazy_static! {
        static ref ID_RE: Regex = Regex::new(r#"^[-_0-9a-zA-Z]{4,32}$"#).unwrap();
    }
    if ID_RE.is_match(account) {
        Some(account.to_lowercase())
    } else {
        None
    }
}

fn hash_pwd(unique_salt: &str, pwd_str: &str) -> String {
    let mut s = Sha256::new();
    s.input((String::new() + unique_salt + pwd_str + PWD_SALT).as_bytes());
    format!("{:x}", s.result())
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Validate)]
pub struct LoginReq {
    #[validate(length(min = 4, max = 32))]
    pub account: String,
    #[validate(length(max = 64))]
    pub pwd: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum LoginResp {
    Success,
    IdIllegal,
    NoSuchAccount,
    WrongPassword,
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn login(site_state: &'static SiteState, req: LoginReq, session: &mut Session) -> Result<LoginResp, RpcError> {
    req.validate().map_err(|x| RpcError::IllegalArgs(x.to_string()))?;
    let db = site_state.db()?;
    if let Some(acc) = normalize_id(&req.account) {
        let p: Option<(String, String, String)> = users
            .select((id, name, pwd))
            .filter(id.eq(&acc))
            .first(&db)
            .optional()
            .map_err(|x| RpcError::InternalError(x.to_string()))?;
        let ret = match p {
            None => LoginResp::NoSuchAccount,
            Some((u_id, u_name, p)) => {
                if hash_pwd(&acc, &req.pwd) != p {
                    LoginResp::WrongPassword
                } else {
                    session.login_user = Some(LoginUser {
                        id: u_id,
                        name: u_name,
                    });
                    session.update();
                    LoginResp::Success
                }
            },
        };
        Ok(ret)
    } else {
        Ok(LoginResp::IdIllegal)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Validate)]
pub struct CurrentUserReq { }

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum CurrentUserResp {
    Logged(LoginUser),
    NoLogged,
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn current_user(_site_state: &'static SiteState, _req: CurrentUserReq, session: &mut Session) -> Result<CurrentUserResp, RpcError> {
    if let Some(login_user) = &session.login_user {
        Ok(CurrentUserResp::Logged(login_user.clone()))
    } else {
        Ok(CurrentUserResp::NoLogged)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Validate)]
pub struct RegisterReq {
    #[validate(length(min = 4, max = 32))]
    pub name: String,
    #[validate(length(min = 4, max = 32))]
    pub account: String,
    #[validate(length(max = 64))]
    pub pwd: String,
    #[validate(email, length(max = 64))]
    pub email: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum RegisterResp {
    Success,
    Denied,
    IdIllegal,
    IdUsed,
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn register(site_state: &'static SiteState, req: RegisterReq, _session: &mut Session) -> Result<RegisterResp, RpcError> {
    req.validate().map_err(|x| RpcError::IllegalArgs(x.to_string()))?;
    let db = site_state.db()?;
    let deny_register: bool = crate::config::get(site_state, "no_register")?.unwrap_or(false);
    if deny_register {
        return Ok(RegisterResp::Denied);
    }
    if let Some(acc) = normalize_id(&req.account) {
        let u: Option<String> = users
            .select(name)
            .filter(id.eq(&acc))
            .first(&db)
            .optional()
            .map_err(|x| RpcError::InternalError(x.to_string()))?;
        if u.is_some() {
            return Ok(RegisterResp::IdUsed);
        }
        diesel::insert_into(users)
            .values((
                id.eq(&acc),
                name.eq(&req.name),
                pwd.eq(hash_pwd(&acc, &req.pwd)),
                email.eq(&req.email),
            ))
            .execute(&db)
            .map_err(|x| RpcError::InternalError(x.to_string()))?;
        Ok(RegisterResp::Success)
    } else {
        Ok(RegisterResp::IdIllegal)
    }
}
