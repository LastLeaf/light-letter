use maomi::prelude::*;

use crate::PageMetaData;
use super::*;

#[derive(Default, serde::Deserialize)]
pub struct Query {
    username: String,
}

template!(xml<B: Backend> for<B> Login<B> ~LOGIN {
    <TextInput<_>
        value={ &self.account }
        placeholder="Account"
        @update={ |mut s, value: &str| {
            s.account = value.to_string();
            s.ctx.update();
        } }
    />
    <TextInput<_>
        value={ &self.pwd }
        placeholder="Password"
        @update={ |mut s, value: &str| {
            s.pwd = value.to_string();
            s.ctx.update();
        } }
    />
});
skin!(LOGIN = r#"

"#);
pub struct Login<B: Backend> {
    ctx: ComponentContext<B, Self>,
    account: String,
    pwd: String,
}
impl<B: Backend> Component<B> for Login<B> {
    fn new(ctx: ComponentContext<B, Self>) -> Self {
        Self {
            ctx,
            account: String::new(),
            pwd: String::new(),
        }
    }
}
impl<B: Backend> PrerenderableComponent<B> for Login<B> {
    type Args = crate::ReqArgs<Query>;
    type PrerenderedData = String;
    type MetaData = PageMetaData;
    fn get_prerendered_data(&self, args: Self::Args) -> std::pin::Pin<Box<dyn futures::Future<Output = (Self::PrerenderedData, Self::MetaData)>>> {
        let meta_data = PageMetaData {
            title: "Login".into(),
        };
        Box::pin(futures::future::ready((args.query.username, meta_data)))
    }
    fn apply_prerendered_data(&mut self, data: &Self::PrerenderedData) {
        self.account = data.clone();
        self.ctx.update();
    }
}
impl<B: Backend> Login<B> {
    // empty
}
