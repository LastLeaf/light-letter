use maomi::prelude::*;
use light_letter_rpc::session::LoginUser;
use light_letter_rpc::backstage::login::*;

use crate::PageMetaData;
use super::*;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Query { }

template!(xml<B: Backend> for<B> Home<B> ~HOME {
    <if { self.current_user.is_some() }>
        <div>"Hi, " <span>{ &self.current_user.as_ref().unwrap().name }</span> </div>
    </if>
});
skin!(HOME = r#"

"#);
pub struct Home<B: Backend> {
    ctx: ComponentContext<B, Self>,
    current_user: Option<LoginUser>,
}
impl<B: Backend> Component<B> for Home<B> {
    fn new(ctx: ComponentContext<B, Self>) -> Self {
        Self {
            ctx,
            current_user: None,
        }
    }
    fn attached(&mut self) {
        if self.current_user.is_none() {
            let query: login::Query = Default::default();
            crate::redirect_to("/backstage/login", query);
        }
    }
}
impl<B: Backend> PrerenderableComponent<B> for Home<B> {
    type Args = crate::ReqArgs<Query>;
    type PrerenderedData = Option<LoginUser>;
    type MetaData = PageMetaData;
    fn get_prerendered_data(&self, args: Self::Args) -> std::pin::Pin<Box<dyn futures::Future<Output = (Self::PrerenderedData, Self::MetaData)>>> {
        let meta_data = PageMetaData {
            title: "Backstage Home".into(),
        };
        Box::pin(async move {
            let current_user: Option<CurrentUserResp> = args.request_channel.request("/backstage/current-user", &CurrentUserReq { }).await.ok();
            let current_user = if let Some(CurrentUserResp::Logged(current_user)) = current_user {
                Some(current_user)
            } else {
                None
            };
            (current_user, meta_data)
        })
    }
    fn apply_prerendered_data(&mut self, data: &Self::PrerenderedData) {
        self.current_user = data.clone();
        self.ctx.update();
    }
}
impl<B: Backend> Home<B> {
    
}
