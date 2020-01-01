use std::sync::Once;
use std::io::Write;
use actix_web::{web, Scope, HttpRequest, HttpResponse};
use wasm_bindgen::prelude::*;
use maomi::prelude::*;
use maomi::Context;
use maomi::backend::Dom;

pub mod backstage;

thread_local! {
    static CONTEXT: std::cell::RefCell<Option<(Context<Dom>, Vec<u8>)>> = std::cell::RefCell::new(None);
}

fn init_logger() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        console_log::init_with_level(log::Level::Debug).unwrap();
    });
}

fn init_prerendered<'a, C: PrerenderableComponent<'a, maomi::backend::Dom>>(prerendered_data: &str) {
    init_logger();
    let prerendered_data = base64::decode(&prerendered_data).unwrap();
    let context = maomi::Context::new_prerendered::<C>(maomi::backend::Dom::new_prerendering("maomi-prerendered"), &prerendered_data);
    CONTEXT.with(|c| {
        *c.borrow_mut() = Some((context, prerendered_data));
    })
}

#[wasm_bindgen]
pub fn load_maomi_component(root_comp_name: &str, data: &str) {
    match root_comp_name {
        "hello_world" => init_prerendered::<backstage::login::HelloWorld>(data),
        _ => panic!()
    }
}

pub(crate) async fn page(req: HttpRequest) -> HttpResponse {
    let root_comp_name = "";
    let title = "TEST";
    let style_links = "";
    let script_links = "";

    let (context, prerendered_data) = maomi::context::Context::prerender::<backstage::login::HelloWorld>(maomi::backend::Empty::new());
    let root_component = context.root_component::<backstage::login::HelloWorld>().unwrap();
    let root_component = root_component.borrow();
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
        r#"{script_links}<script>__load_maomi_component__("{root_comp_name}", "{prerendered_data}")</script></body></html>"#,
        root_comp_name = root_comp_name,
        prerendered_data = base64::encode(&prerendered_data),
        script_links = script_links,
    ).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

pub(crate) fn route(scope: Scope) -> Scope {
    let scope = scope.route("/backstage", web::get().to(page));
    scope
}
