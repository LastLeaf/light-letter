use std::io::Write;
use hyper::{Body, Request, Response};
use light_letter_web::{ReqInfo, PrerenderResult, prerender_maomi_component, get_css_str};

use super::{SiteState, res_utils, error::Error};

thread_local! {
    static CSS_STR: &'static [u8] = get_css_str().as_bytes();
    static JS_STR: &'static [u8] = include_str!("../../../light-letter-web/pkg/light_letter_web.js").as_bytes(); // TODO change to be able to use with cargo
    static WASM_STR: &'static [u8] = include_bytes!("../../../light-letter-web/pkg/light_letter_web_bg.wasm"); // TODO change to be able to use with cargo
}

fn http_request_info(req: &Request<Body>) -> ReqInfo {
    ReqInfo {
        path: req.uri().path().into(),
        query: req.uri().query().unwrap_or_default().into(),
    }
}

fn render_page_component(req: &Request<Body>, prerendered: PrerenderResult) -> Response<Body> {
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
    root_component.to_html(&mut html).unwrap();
    write!(
        html,
        r#"{script_links}<script>__load_maomi_component__(location.path, "{prerendered_data}")</script></body></html>"#,
        prerendered_data = base64::encode(&prerendered.prerendered_data),
        script_links = script_links,
    ).unwrap();

    if prerendered.is_ok { res_utils::html_ok(req, html.into()) } else { res_utils::not_found(req) }
}

pub(crate) async fn page(site_state: &SiteState, req: &Request<Body>) -> Response<Body> {
    if req.method() != "GET" {
        return Error::forbidden("Invalid Method").response();
    }
    let req_info = http_request_info(&req);
    let prerendered = prerender_maomi_component(req_info);
    render_page_component(req, prerendered)
}

pub(crate) async fn rpc(site_state: &SiteState, req: &Request<Body>, sub_path: &str) -> Response<Body> {
    if req.method() != "POST" {
        return Error::forbidden("Invalid Method").response();
    }
    unimplemented!()
}

pub(crate) fn static_resource(req: &Request<Body>, sub_path: &str, modified: &chrono::DateTime<chrono::Utc>) -> Response<Body> {
    match sub_path {
        "light_letter_web.css" => CSS_STR.with(|s| res_utils::cache_ok(req, modified, "text/css", (*s).into())),
        "light_letter_web.js" => JS_STR.with(|s| res_utils::cache_ok(req, modified, "text/javascript", (*s).into())),
        "light_letter_web_bg.wasm" => WASM_STR.with(|s| res_utils::cache_ok(req, modified, "application/wasm", (*s).into())),
        _ => res_utils::not_found(req),
    }
}
