#[macro_use] extern crate log;

use wasm_bindgen::prelude::*;
use light_letter_web::*;

mod not_found;
mod pages;

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

#[derive(Default)]
pub struct Theme {
    // empty
}

theme!(Theme);

routes! {
    not_found::NotFound,
    "/" => pages::index::Index<_>;
    "/about" => pages::index::About<_>;
}

stylesheets! {
    pages::index::Index<_>;
}
