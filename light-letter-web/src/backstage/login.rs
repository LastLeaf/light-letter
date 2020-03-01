use maomi::prelude::*;

use crate::PageMetaData;

#[derive(Default, serde::Deserialize)]
pub struct Query {
    r#from: String,
}

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
    type Args = crate::ReqArgs<Query>;
    type PrerenderedData = String;
    type MetaData = PageMetaData;
    fn get_prerendered_data(&self, args: Self::Args) -> std::pin::Pin<Box<dyn futures::Future<Output = (Self::PrerenderedData, Self::MetaData)>>> {
        let meta_data = PageMetaData {
            title: format!("From {}", args.query.r#from),
        };
        let prerendered_data = format!("Hello world from {}!", args.query.from);
        Box::pin(futures::future::ready((prerendered_data, meta_data)))
    }
    fn apply_prerendered_data(&mut self, data: &Self::PrerenderedData) {
        self.title = data.clone();
        self.ctx.update();
    }
}
impl<B: Backend> HelloWorld<B> {
    fn tap(self: &mut Self) {
        self.ctx.tick_with_component_rc(|_| {
            crate::route_to("/backstage", "from=TEST");
        })
    }
}
