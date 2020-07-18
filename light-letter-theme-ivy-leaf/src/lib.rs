#[macro_use] extern crate log;

use wasm_bindgen::prelude::*;
use light_letter_web::*;

mod pages;

pub struct Theme {
    // empty
}

impl Theme {
    pub fn new() -> Self {
        Self {
            // empty
        }
    }
}

impl light_letter_web::Theme for Theme {
    fn prerender_maomi_component(&self, req_info: ReqInfo, request_channel: RequestChannel) -> PrerenderResult {
        routes::prerender_maomi_component(req_info, request_channel)
    }
    fn get_css_str(&self) -> &'static str {
        stylesheets::get_css_str()
    }
}

#[wasm_bindgen]
pub fn load_maomi_component(path: &str, data: &str) {
    routes::load_maomi_component(path, data)
}

routes! {
    light_letter_web::not_found::NotFound,
    "/" => pages::index::Index<_>;
}

stylesheets! {
    pages::index::Index<_>;
}
