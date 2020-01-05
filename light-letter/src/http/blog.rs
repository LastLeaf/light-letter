use std::io::Write;
use actix_web::{web, HttpRequest, HttpResponse};
use light_letter_web::{ReqInfo, PrerenderResult, prerender_maomi_component};

use super::SiteState;

fn http_request_info(req: &HttpRequest) -> ReqInfo {
    ReqInfo {
        path: "".into(),
        query: "".into(),
    }
}

fn render_page_component(prerendered: PrerenderResult) -> HttpResponse {
    let title = &prerendered.title;
    let style_links = "";
    let script_links = "";

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

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

pub(crate) async fn page(site_state: web::Data<SiteState>, req: HttpRequest) -> HttpResponse {
    let path: std::path::PathBuf = req.match_info().query("path").parse().unwrap();
    let req_info = http_request_info(&req);
    let prerendered = prerender_maomi_component(req_info);
    render_page_component(prerendered)
}

pub(crate) async fn rpc(site_state: web::Data<SiteState>, req: HttpRequest) -> HttpResponse {
    let path: std::path::PathBuf = req.match_info().query("path").parse().unwrap();
    unimplemented!()
}
