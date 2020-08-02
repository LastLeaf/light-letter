#![cfg(not(target_arch = "wasm32"))]

use diesel::prelude::*;

use crate::{SiteState, RpcError};
use crate::schema::config::dsl::*;

pub fn get<T: serde::de::DeserializeOwned>(site_state: &'static SiteState, config_key: &str) -> Result<Option<T>, RpcError> {
    let db = site_state.db()?;
    let ret: Option<String> = config
        .select(value)
        .filter(key.eq(config_key))
        .first(&db)
        .optional()
        .map_err(|x| RpcError::InternalError(x.to_string()))?;
    Ok(match ret {
        None => None,
        Some(x) => serde_json::from_reader(x.as_bytes()).ok(),
    })
}

pub fn set<T: serde::Serialize>(site_state: &'static SiteState, config_key: &str, config_value: &T) -> Result<(), RpcError> {
    let db = site_state.db()?;
    let config_value = serde_json::to_string(config_value).map_err(|x| RpcError::InternalError(x.to_string()))?;
    diesel::insert_into(config)
        .values((
            key.eq(config_key),
            value.eq(&config_value),
        ))
        .on_conflict(key)
        .do_update()
        .set(value.eq(&config_value))
        .execute(&db)
        .map_err(|x| RpcError::InternalError(x.to_string()))?;
    Ok(())
}
