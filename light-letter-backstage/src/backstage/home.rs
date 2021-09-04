use maomi::prelude::*;
use light_letter_rpc::session::LoginUser;
use light_letter_rpc::backstage::login::*;
use light_letter_rpc::backstage::post::*;

use crate::PageMetaData;
use super::*;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Query { }

template!(xml<B: Backend> for<B> Home<B> ~HOME {
    <HintArea<_> mark="hint" />
    <div class="head">
        <if { self.current_user.is_some() }>
            <div> { tr!("login-as") } <span>{ &self.current_user.as_ref().unwrap().name }</span> </div>
        </if>
    </div>
    <div class="body">
        <div class="section">
            <div class="section-title"> { tr!("recent-posts") } </div>
            <for item in { &self.post_list }>
                <div>
                    // <a href="javascript:;" @tap={ move |_, _| crate::route_to(&format!("/backstage/post/{}", item.id), "") }>
                    //     { item.title }
                    // </a>
                </div>
            </for>
        </div>
    </div>
});

skin!(HOME = r#"
    @import "/backstage/style.skin";

"#);

pub struct Home<B: Backend> {
    ctx: ComponentContext<B, Self>,
    current_user: Option<LoginUser>,
    post_list: Vec<PostMeta>,
}

impl<B: Backend> Component<B> for Home<B> {
    fn new(ctx: ComponentContext<B, Self>) -> Self {
        Self {
            ctx,
            current_user: None,
            post_list: vec![],
        }
    }
    fn attached(&mut self) {
        if self.current_user.is_none() {
            let query: login::Query = Default::default();
            crate::redirect_to("/backstage/login", query);
            return;
        }
        self.load_post_list(0);
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
    component_common!();

    fn load_post_list(&self, skip: usize) {
        self.ctx.tick_with_component_rc(move |this| {
            crate::run_client_async(async move {
                let req = PostListReq {
                    skip,
                    count: 20,
                    filter: PostListFilter::None,
                };
                match crate::client_request_channel().request("/backstage/post/recent", &req).await {
                    Err(e) => error!("{}", e),
                    Ok(resp) => {
                        let resp: PostListResp = resp;
                        let mut this = this.borrow_mut();
                        if let PostListResp::Success(list) = resp {
                            this.post_list = list;
                            this.ctx.update();
                        }
                    }
                }
            });
        });
    }
}
