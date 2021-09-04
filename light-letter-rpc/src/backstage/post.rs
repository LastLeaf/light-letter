use validator::Validate;
#[cfg(not(target_arch = "wasm32"))]
use diesel::prelude::*;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::{RpcError, Session, LoginUser};
#[cfg(not(target_arch = "wasm32"))]
use crate::SiteState;
#[cfg(not(target_arch = "wasm32"))]
use crate::schema::{posts::dsl::*, post_tags::dsl::*};
#[cfg(not(target_arch = "wasm32"))]
use crate::models::Post;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Validate)]
pub struct PostListReq {
    #[validate(range(min = 0))]
    pub skip: usize,
    #[validate(range(max = 100))]
    pub count: usize,
    pub filter: PostListFilter,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum PostListFilter {
    None,
    Series(String),
    Category(String),
    Tag(String),
    Keyword(String),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum PostListResp {
    Success(Vec<PostMeta>),
    Fail,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct PostMeta {
    pub id: String,
    pub status: PostStatus,
    pub timestamp: u64,
    pub url: Option<String>,
    pub title: String,
    pub abstract_: String,
    pub series: Option<String>,
    pub category: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum PostStatus {
    Draft = 0,
    Published = 1,
    HiddenLink = 2,
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn list(site_state: &'static SiteState, req: PostListReq, _session: &mut Session) -> Result<PostListResp, RpcError> {
    req.validate().map_err(|x| RpcError::IllegalArgs(x.to_string()))?;
    let db = site_state.db()?;
    // TODO impl filters
    let p: Vec<Post> = posts
        .order_by(timestamp.desc())
        .offset(req.skip as i64)
        .limit(req.count as i64)
        .load::<Post>(&db)
        .map_err(|x| RpcError::InternalError(x.to_string()))?;
    let ret: Vec<_> = p.into_iter().map(|x| {
        PostMeta {
            id: x.id.to_string(),
            status: PostStatus::from_i32(x.status).unwrap_or(PostStatus::Draft),
            timestamp: x.timestamp.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
            url: x.url,
            title: x.title,
            abstract_: x.abstract_,
            series: x.series.map(|x| x.to_string()),
            category: x.category.to_string(),
        }
    }).collect();
    Ok(PostListResp::Success(ret))
}
