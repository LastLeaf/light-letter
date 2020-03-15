use std::io::Write;
use hyper::{Body, Request, Response};
use futures::future::FutureExt;
use futures::stream::StreamExt;
use light_letter_web::{ReqInfo, PrerenderResult, prerender_maomi_component, get_css_str, RequestChannel, RequestError};

use super::{SiteState, res_utils, error::Error};

thread_local! {
    static CSS_STR: &'static [u8] = get_css_str().as_bytes();
    static JS_STR: &'static [u8] = include_str!("../static/light_letter_web.js").as_bytes();
}

fn http_request_info(req: &http::request::Parts) -> ReqInfo {
    ReqInfo {
        path: req.uri.path().into(),
        query: req.uri.query().unwrap_or_default().into(),
    }
}

fn render_page_component(req: &http::request::Parts, prerendered: PrerenderResult) -> Response<Body> {
    let title = &prerendered.title;
    let style_links = r#"<link rel="stylesheet" href="/static/light_letter_web.css">"#;
    let script_links = r#"<script src="/static/light_letter_web.js"></script>"#;

    let root_component = prerendered.node_rc.borrow();
    let mut html: Vec<u8> = vec![];
    write!(
        html,
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>{title}</title>{style_links}</head><body>"#,
        title = title,
        style_links = style_links,
    ).unwrap();
    root_component.to_html_with_id(&mut html, "maomi-prerendered").unwrap();
    write!(
        html,
        r#"{script_links}<script>__light_letter__.load_maomi_component(location.pathname, "{prerendered_data}")</script></body></html>"#,
        prerendered_data = base64::encode(&prerendered.prerendered_data),
        script_links = script_links,
    ).unwrap();

    if prerendered.is_ok { res_utils::html_ok(req, html.into()) } else { res_utils::not_found(req) }
}

pub(crate) async fn page(site_state: &'static SiteState, req: http::request::Parts) -> Response<Body> {
    if req.method != "GET" {
        return Error::forbidden("Invalid Method").response();
    }
    let req_info = http_request_info(&req);
    let request_channel = RequestChannel::new(move |path, data| {
        Box::pin(light_letter_rpc::rpc_route(path.to_string(), site_state, data).map(|x| x.map_err(|x| RequestError::Custom(x.to_string()))))
    });
    let prerendered = prerender_maomi_component(req_info, request_channel);
    render_page_component(&req, prerendered)
}

pub(crate) async fn rpc(site_state: &'static SiteState, req: http::request::Parts, req_body: Body, sub_path: String) -> Response<Body> {
    if req.method != "POST" {
        return Error::forbidden("Invalid Method").response();
    }
    // TODO check referrer
    let sub_path = sub_path.to_owned();
    let mut body: Vec<u8> = vec![];
    req_body.for_each(|x| {
        let a: &[u8] = &x.unwrap_or_default();
        body.extend_from_slice(a);
        futures::future::ready(())
    }).await;
    match light_letter_rpc::rpc_route(sub_path, site_state, std::str::from_utf8(&body).unwrap_or_default().to_owned()).await {
        Ok(r) => {
            res_utils::html_ok(&req, std::borrow::Cow::Owned(r.into_bytes()))
        },
        Err(e) => {
            res_utils::internal_server_error(&req, e.to_string())
        },
    }
}

pub(crate) fn static_resource(req: &http::request::Parts, sub_path: &str, modified: &chrono::DateTime<chrono::Utc>) -> Response<Body> {
    match sub_path {
        "light_letter_web.css" => CSS_STR.with(|s| res_utils::cache_ok(req, modified, "text/css", (*s).into())),
        "light_letter_web.js" => JS_STR.with(|s| res_utils::cache_ok(req, modified, "text/javascript", (*s).into())),
        // "light_letter_web_bg.wasm" => WASM_STR.with(|s| res_utils::cache_ok(req, modified, "application/wasm", (*s).into())),
        _ => res_utils::not_found(req),
    }
}
