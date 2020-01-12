use std::io::Write;
use actix_web::{web, HttpRequest, HttpResponse};
use light_letter_web::{ReqInfo, PrerenderResult, prerender_maomi_component, get_css_str};

use super::SiteState;

thread_local! {
    static CSS_STR: String = get_css_str();
    static JS_STR: &'static str = include_str!("../../../light-letter-web/pkg/light_letter_web.js"); // TODO change to be able to use with cargo
    static WASM_STR: &'static [u8] = include_bytes!("../../../light-letter-web/pkg/light_letter_web_bg.wasm"); // TODO change to be able to use with cargo
}

fn http_request_info(req: &HttpRequest) -> ReqInfo {
    ReqInfo {
        path: req.path().into(),
        query: req.query_string().into(),
    }
}

fn render_page_component(prerendered: PrerenderResult) -> HttpResponse {
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

    if prerendered.is_ok { HttpResponse::Ok() } else { HttpResponse::NotFound() }
        .content_type("text/html")
        .body(html)
}

pub(crate) async fn page(site_state: web::Data<SiteState>, req: HttpRequest) -> HttpResponse {
    let req_info = http_request_info(&req);
    let prerendered = prerender_maomi_component(req_info);
    render_page_component(prerendered)
}

pub(crate) async fn rpc(site_state: web::Data<SiteState>, req: HttpRequest) -> HttpResponse {
    unimplemented!()
}

pub(crate) fn static_resource(req: HttpRequest) -> HttpResponse {
    let res: &str = req.match_info().query("res");
    match res {
        "light_letter_web.css" => CSS_STR.with(|s| HttpResponse::Ok().content_type("text/css").body(s)),
        "light_letter_web.js" => JS_STR.with(|s| HttpResponse::Ok().content_type("text/javascript").body(*s)),
        "light_letter_web_bg.wasm" => WASM_STR.with(|s| HttpResponse::Ok().content_type("application/wasm").body(*s)),
        _ => HttpResponse::NotFound().body(""),
    }
}
