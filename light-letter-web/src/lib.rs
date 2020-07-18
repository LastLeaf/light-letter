#[macro_use] extern crate log;

use std::collections::HashMap;
#[allow(unused_imports)] use wasm_bindgen::prelude::*;
use maomi::prelude::*;
use maomi::backend::{Empty};

mod components;
pub mod not_found;
mod backstage;
#[macro_use] mod routing;
pub(crate) use routes::route_to;
pub use routes::{prerender_maomi_component};
pub use stylesheets::get_css_str;
mod request;
pub use request::client_request_channel;
pub use request::{RequestChannel, RequestError};
mod theme;
pub use theme::Theme;

fn init_logger() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        console_error_panic_hook::set_once();
        console_log::init_with_level(log::Level::Debug).unwrap();
    });
}

#[wasm_bindgen(start)]
pub fn wasm_main() {
    init_logger();
}

pub fn run_client_async<F: futures::Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

#[derive(Debug, Clone)]
pub struct ReqInfo {
    pub path: String,
    pub query: String,
}

#[derive(Clone)]
pub struct ReqArgs<T: Default> {
    pub path_args: HashMap<&'static str, String>,
    pub query: T,
    pub request_channel: RequestChannel,
}

#[derive(Clone)]
pub struct PrerenderResult {
    pub node_rc: ComponentNodeRc<Empty>,
    pub prerendered_data: Vec<u8>,
    pub title: String,
    pub is_ok: bool,
}

pub struct PageMetaData {
    pub title: String,
}

#[cfg(not(feature = "wasm_export"))]
#[wasm_bindgen]
pub fn load_maomi_component(path: &str, data: &str) {
    routes::load_maomi_component(path, data)
}

routes! {
    not_found::NotFound,
    "/backstage" => backstage::login::Login<_>;
}

stylesheets! {
    // basic components shares one style sheet
    components::input::TextInput<_>;
    backstage::login::Login<_>;
}
