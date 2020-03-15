#[macro_use] extern crate log;

use std::collections::HashMap;
#[allow(unused_imports)] use wasm_bindgen::prelude::*;
use maomi::prelude::*;
use maomi::backend::{Dom, Empty};

mod components;
mod not_found;
mod backstage;
#[macro_use] mod routing;
pub use routing::{ReqInfo, ReqArgs, PrerenderResult, PageMetaData};
pub(crate) use routing::route_to;
pub use routes::{prerender_maomi_component, get_css_str, load_maomi_component};
mod request;
pub(crate) use request::client_request_channel;
pub use request::{RequestChannel, RequestError};

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
    routing::history_state_init();
}

fn run_client_async<F: futures::Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

mod routes {
    use super::*;
    use super::routing::*;

    routes! {
        "/backstage" => backstage::login::Login<_>;
    }

    stylesheets! {
        // basic components shares one style sheet
        components::input::TextInput<_>;
        backstage::login::Login<_>;
    }
}
