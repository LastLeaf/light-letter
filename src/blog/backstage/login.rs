use maomi::prelude::*;

template!(xml for HelloWorld {
    <div style="display: inline">
        {&self.a}
        <slot />
    </div>
});
pub struct HelloWorld {
    pub a: String,
}
impl<B: Backend> Component<B> for HelloWorld {
    fn new(_ctx: ComponentContext<B, Self>) -> Self {
        Self {
            a: "Hello world!".into()
        }
    }
}
impl<B: Backend> PrerenderableComponent<'_, B> for HelloWorld {
    type PrerenderedData = String;
    fn get_prerendered_data(&self) -> std::pin::Pin<Box<dyn futures::Future<Output = Self::PrerenderedData>>> {
        Box::pin(futures::future::ready("PRERENDER".into()))
    }
    fn apply_prerendered_data(&mut self, data: &Self::PrerenderedData) {
        self.a = data.clone();
    }
}
