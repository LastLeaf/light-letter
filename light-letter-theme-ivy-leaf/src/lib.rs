use light_letter_web::{ReqInfo, PrerenderResult, RequestChannel, RequestError};

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
    fn name(&self) -> &'static str {
        "ivy-leaf"
    }
    fn prerender_maomi_component(&self, req_info: ReqInfo, request_channel: RequestChannel) -> PrerenderResult {
        // TODO
        unimplemented!()
    }
    fn get_css_str(&self) -> &'static str {
        // TODO
        unimplemented!()
    }
}
