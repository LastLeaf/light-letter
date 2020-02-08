use serde::Deserialize;
use maomi::prelude::*;

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
    fn get_prerendered_data(&self, _args: Self::Args) -> std::pin::Pin<Box<dyn futures::Future<Output = Self::PrerenderedData>>> {
        Box::pin(futures::future::ready(()))
    }
    fn apply_prerendered_data(&mut self, _data: &Self::PrerenderedData) {
        // empty
    }
}
