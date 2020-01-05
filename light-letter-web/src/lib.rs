#[allow(unused_imports)] use wasm_bindgen::prelude::*;
use maomi::prelude::*;
use maomi::Context;
use maomi::backend::{Dom, Empty};
use maomi::node::ComponentNodeRc;

mod not_found;
mod backstage;

thread_local! {
    static CONTEXT: std::cell::RefCell<Option<(Context<Dom>, Vec<u8>)>> = std::cell::RefCell::new(None);
}

#[cfg(target_os = "wasm32")]
fn init_logger() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        console_log::init_with_level(log::Level::Debug).unwrap();
    });
}

#[cfg(target_os = "wasm32")]
fn init_prerendered<C: PrerenderableComponent<Dom>>(prerendered_data: &str) {
    init_logger();
    let prerendered_data = base64::decode(&prerendered_data).unwrap();
    let context = maomi::Context::new_prerendered::<C>(Dom::new_prerendering("maomi-prerendered"), &prerendered_data);
    CONTEXT.with(|c| {
        *c.borrow_mut() = Some((context, prerendered_data));
    })
}

pub struct ReqInfo {
    pub path: String,
    pub query: String,
}

pub struct PrerenderResult {
    pub node_rc: ComponentNodeRc<Empty>,
    pub prerendered_data: Vec<u8>,
    pub title: String
}

fn prerender<C: PrerenderableComponent<Empty>>(req_info: ReqInfo) -> PrerenderResult {
    let (context, prerendered_data) = maomi::context::Context::prerender::<C>(Empty::new());
    let comp = context.root_component::<C>().unwrap().into_node();
    PrerenderResult {
        node_rc: comp,
        prerendered_data,
        title: String::new(), // TODO
    }
}

// The entrance in non-ssr mode
#[cfg(target_os = "wasm32")]
#[wasm_bindgen]
pub fn client_render_maomi_component(path: &str, query: &str) {
    unimplemented!()
}

// Do ssr
pub fn prerender_maomi_component(req_info: ReqInfo) -> PrerenderResult {
    match req_info.path.as_str() {
        "hello_world" => prerender::<backstage::login::HelloWorld>(req_info),
        _ => prerender::<not_found::NotFound>(req_info)
    }
}

// Load ssr result
#[cfg(target_os = "wasm32")]
#[wasm_bindgen]
pub fn load_maomi_component(path: &str, data: &str) {
    match root_comp_name {
        "hello_world" => init_prerendered::<backstage::login::HelloWorld>(data),
        _ => init_prerendered::<not_found::NotFound>(data)
    }
}
