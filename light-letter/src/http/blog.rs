use std::io::Write;
use hyper::{Body, Response};
use futures::future::FutureExt;
use futures::stream::StreamExt;
use light_letter_web::{ReqInfo, PrerenderResult, RequestChannel, RequestError};
use light_letter_web::{prerender_maomi_component, get_css_str};

use super::{SiteState, resource, res_utils, error::Error};

lazy_static! {
    static ref VERSION: u32 = rand::random();
}

fn http_request_info(req: &http::request::Parts) -> ReqInfo {
    ReqInfo {
        path: req.uri.path().into(),
        query: req.uri.query().unwrap_or_default().into(),
    }
}

fn render_page_component(
    req: &http::request::Parts,
    prerendered: PrerenderResult,
    style_url: &str,
    script_url: &str,
    wasm_url: &str,
) -> Response<Body> {
    let title = &prerendered.title;
    let root_component = prerendered.node_rc.borrow();
    let mut html: Vec<u8> = vec![];
    write!(
        html,
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{title}</title>{style_links}</head><body>"#,
        title = title,
        style_links = format_args!(r#"<link rel="stylesheet" href="/static/{}?v={}">"#, style_url, *VERSION),
    ).unwrap();
    root_component.to_html_with_id(&mut html, "maomi-prerendered").unwrap();
    write!(
        html,
        r#"{script_links}<script>var xhr=new XMLHttpRequest();xhr.responseType="arraybuffer";xhr.addEventListener("load", function(){{wasm_bindgen(xhr.response).then(function(){{wasm_bindgen.load_maomi_component(location.pathname,"{prerendered_data}")}})}});xhr.open("GET","{wasm_link}");xhr.send()</script></body></html>"#,
        prerendered_data = base64::encode(&prerendered.prerendered_data),
        script_links = format_args!(r#"<script src="/static/{}?v={}"></script>"#, script_url, *VERSION),
        wasm_link = format_args!(r#"/static/{}?v={}"#, wasm_url, *VERSION),
    ).unwrap();
    if prerendered.is_ok { res_utils::html_ok(req, html.into()) } else { res_utils::not_found(req) }
}

pub(crate) async fn backstage_page(site_state: &'static SiteState, req: http::request::Parts) -> Response<Body> {
    if req.method != "GET" {
        return Error::forbidden("Invalid Method").response();
    }
    let req_info = http_request_info(&req);
    let request_channel = RequestChannel::new(move |path, data| {
        Box::pin(light_letter_rpc::rpc_route(path.to_string(), site_state, data).map(|x| x.map_err(|x| RequestError::Custom(x.to_string()))))
    });
    let prerendered = prerender_maomi_component(req_info, request_channel);
    render_page_component(&req, prerendered, "light_letter_web.css", "light_letter_web.js", "light_letter_web_bg.wasm")
}

pub(crate) async fn page(site_state: &'static SiteState, req: http::request::Parts) -> Response<Body> {
    if req.method != "GET" {
        return Error::forbidden("Invalid Method").response();
    }
    let theme_mod_name = &site_state.theme_mod_name;
    let theme = crate::themes::get(theme_mod_name).expect("No such theme mod loaded");
    let req_info = http_request_info(&req);
    let request_channel = RequestChannel::new(move |path, data| {
        Box::pin(light_letter_rpc::rpc_route(path.to_string(), site_state, data).map(|x| x.map_err(|x| RequestError::Custom(x.to_string()))))
    });
    let prerendered = theme.prerender_maomi_component(req_info, request_channel);
    render_page_component(&req, prerendered, &format!("{}.css", theme_mod_name), &format!("{}.js", theme_mod_name), &format!("{}_bg.wasm", theme_mod_name))
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
        "light_letter_web.css" => res_utils::cache_ok(req, modified, "text/css", get_css_str().as_bytes().into()),
        "light_letter_web.js" => resource::get(|r| res_utils::cache_ok(req, modified, "text/javascript", r.web_js.into())),
        "light_letter_web_bg.wasm" => resource::get(|r| res_utils::cache_ok(req, modified, "application/wasm", r.web_wasm.into())),
        _ => {
            resource::get(|r| {
                if sub_path.ends_with(".css") {
                    let theme_mod_name = sub_path.get(..(sub_path.len() - 4)).unwrap_or("");
                    if let Some(theme) = crate::themes::get(theme_mod_name) {
                        return Some(res_utils::cache_ok(req, modified, "text/css", theme.get_css_str().as_bytes().into()));
                    }
                } else if sub_path.ends_with(".js") {
                    let theme_mod_name = sub_path.get(..(sub_path.len() - 3)).unwrap_or("");
                    if let Some(s) = r.theme_js.get(theme_mod_name) {
                        return Some(res_utils::cache_ok(req, modified, "text/javascript", (*s).into()));
                    }
                } else if sub_path.ends_with("_bg.wasm") {
                    let theme_mod_name = sub_path.get(..(sub_path.len() - 8)).unwrap_or("");
                    if let Some(s) = r.theme_wasm.get(theme_mod_name) {
                        return Some(res_utils::cache_ok(req, modified, "application/wasm", (*s).into()));
                    }
                }
                None
            }).unwrap_or_else(|| {
                res_utils::not_found(req)
            })
        },
    }
}
