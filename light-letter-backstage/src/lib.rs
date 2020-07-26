#[macro_use] extern crate log;

use wasm_bindgen::prelude::*;
use light_letter_web::*;
pub use light_letter_web::request::{client_request_channel, RequestChannel, RequestError};

mod components;
pub mod not_found;
mod backstage;
pub(crate) use routes::route_to;
pub use routes::{prerender_maomi_component};
pub use stylesheets::get_css_str;

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
    "/backstage" => backstage::login::Login<_>;
}

stylesheets! {
    // basic components shares one style sheet
    components::input::TextInput<_>;
    backstage::login::Login<_>;
}
