use maomi::prelude::*;

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
    type PrerenderedData = ();
    fn get_prerendered_data(&self) -> std::pin::Pin<Box<dyn futures::Future<Output = Self::PrerenderedData>>> {
        Box::pin(futures::future::ready(()))
    }
    fn apply_prerendered_data(&mut self, data: &Self::PrerenderedData) {
        // empty
    }
}
