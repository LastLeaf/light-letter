use serde::Deserialize;
use maomi::prelude::*;

use crate::PageMetaData;

#[derive(Default, Deserialize)]
pub struct Query {
    // empty
}

template!(xml for NotFound {
    <div>
        "Not Found"
    </div>
});
pub struct NotFound { }
impl<B: Backend> Component<B> for NotFound {
    fn new(_ctx: ComponentContext<B, Self>) -> Self {
        Self { }
    }
}
impl<B: Backend> PrerenderableComponent<B> for NotFound {
    type Args = crate::ReqArgs<Query>;
    type PrerenderedData = ();
    type MetaData = PageMetaData;
    fn get_prerendered_data(&self, _args: Self::Args) -> std::pin::Pin<Box<dyn futures::Future<Output = (Self::PrerenderedData, Self::MetaData)>>> {
        let meta_data = PageMetaData {
            title: "Not Found".into(),
        };
        let prerendered_data = ();
        Box::pin(futures::future::ready((prerendered_data, meta_data)))
    }
    fn apply_prerendered_data(&mut self, _data: &Self::PrerenderedData) {
        // empty
    }
}
