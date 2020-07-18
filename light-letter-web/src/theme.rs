use crate::*;

pub trait Theme {
    fn prerender_maomi_component(&self, req_info: ReqInfo, request_channel: RequestChannel) -> PrerenderResult;
    fn get_css_str(&self) -> &'static str;
}
