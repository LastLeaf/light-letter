use maomi::prelude::*;
use light_letter_rpc::backstage::login::*;

use crate::PageMetaData;
use super::*;

#[derive(Default, serde::Deserialize)]
pub struct Query {
    username: String,
}

template!(xml<B: Backend> for<B> Login<B> ~LOGIN {
    <HintArea<_> mark="hint" />
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
    <Button<_>
        @press={|mut s, _| s.login() }
    >
        "Login"
    </Button>
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
        Box::pin(async { (args.query.username, meta_data) })
    }
    fn apply_prerendered_data(&mut self, data: &Self::PrerenderedData) {
        self.account = data.clone();
        self.ctx.update();
    }
}
impl<B: Backend> Login<B> {
    fn login(&mut self) {
        let req = LoginReq {
            account: self.account.clone(),
            pwd: self.pwd.clone(),
        };
        self.ctx.tick_with_component_rc(|s| {
            crate::run_client_async(async move {
                match crate::client_request_channel().request("/backstage/login", &req).await {
                    Err(e) => error!("{}", e),
                    Ok(resp) => {
                        let mut s = s.borrow_mut();
                        match resp {
                            LoginResp::Success => crate::route_to("/backstage/home", ""),
                            LoginResp::NoSuchAccount => s.marked_component::<HintArea<_>>("hint").unwrap().borrow_mut_with(&mut s).show_error("No such account"),
                            LoginResp::WrongPassword => s.marked_component::<HintArea<_>>("hint").unwrap().borrow_mut_with(&mut s).show_error("Wrong password"),
                        }
                    }
                }
            });
        });
    }
}
