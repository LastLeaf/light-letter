#[macro_use] extern crate log;
pub extern crate std;
pub extern crate lazy_static;
pub extern crate base64;
pub extern crate serde;

use std::collections::HashMap;
use maomi::prelude::*;
use maomi::backend::{Empty};

mod routing;
pub mod request;
pub use request::*;
mod theme;
pub use theme::Theme;

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
    pub request_channel: request::RequestChannel,
}

#[derive(Clone)]
pub struct PrerenderResult {
    pub node_rc: ComponentNodeRc<Empty>,
    pub prerendered_data: Vec<u8>,
    pub title: String,
    pub is_ok: bool,
}

#[derive(Debug, Clone)]
pub struct PageMetaData {
    pub title: String,
}

/// Define a theme for a type.
/// An instance is created with `Default` for each thread.
#[macro_export]
macro_rules! theme {
    ($name:ty) => {
        impl $crate::Theme for $name {
            fn prerender_maomi_component(&self, req_info: ReqInfo, request_channel: RequestChannel) -> PrerenderResult {
                routes::prerender_maomi_component(req_info, request_channel)
            }
            fn get_css_str(&self) -> &'static str {
                stylesheets::get_css_str()
            }
        }

        #[no_mangle]
        pub extern "C" fn load_theme() -> $crate::std::rc::Rc<dyn $crate::Theme> {
            $crate::std::rc::Rc::new(<$name>::default())
        }
        
        #[wasm_bindgen]
        pub fn load_maomi_component(path: &str, data: &str) {
            routes::load_maomi_component(path, data)
        }
    }
}
