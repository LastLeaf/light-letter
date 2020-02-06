use maomi::prelude::*;

template!(xml<B: Backend> for<B> HelloWorld<B> ~HELLO_WORLD {
    <input
        r#type="button"
        value={ &self.title }
        @click={ |mut s, _| s.tap() }
    ></input>
});
skin!(HELLO_WORLD = r#"
    .hello-world {
        text-align: center;
    }
"#);
pub struct HelloWorld<B: Backend> {
    ctx: ComponentContext<B, Self>,
    title: String,
}
impl<B: Backend> Component<B> for HelloWorld<B> {
    fn new(ctx: ComponentContext<B, Self>) -> Self {
        Self {
            ctx,
            title: "SSR required...".into()
        }
    }
}
impl<B: Backend> PrerenderableComponent<B> for HelloWorld<B> {
    type PrerenderedData = String;
    fn get_prerendered_data(&self) -> std::pin::Pin<Box<dyn futures::Future<Output = Self::PrerenderedData>>> {
        Box::pin(futures::future::ready("Hello world from SSR!".into()))
    }
    fn apply_prerendered_data(&mut self, data: &Self::PrerenderedData) {
        self.title = data.clone();
        self.ctx.update();
    }
}
impl<B: Backend> HelloWorld<B> {
    fn tap(self: &mut Self) {
        self.title = "Hello world again!".into();
        self.ctx.update();
    }
}
