use maomi::prelude::*;
use light_letter_rpc::backstage::login::*;

use crate::PageMetaData;
use super::*;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Query {
    username: String,
}

template!(xml<B: Backend> for<B> Login<B> ~LOGIN {
    <HintArea<_> mark="hint">
        <for login_resp in { &self.hints }>
            <Hint<_> kind={
                match login_resp {
                    LoginResp::Success => hint::HintKind::Common,
                    _ => hint::HintKind::Error,
                }
            } msg={
                match login_resp {
                    LoginResp::Success => "Success",
                    LoginResp::IdIllegal => "The account name is not legal",
                    LoginResp::NoSuchAccount => "The account does not exists",
                    LoginResp::WrongPassword => "The account and the password does not match",
                }
            } />
        </for>
    </HintArea>
    <div<_>
        @tap={|mut s, _| { s.is_register = false; s.ctx.update() } }
    >
        "Login"
    </div>
    <div<_>
        @tap={|mut s, _| { s.is_register = true; s.ctx.update() } }
    >
        "Register"
    </div>
    <TextInput<_>
        value={ &self.account }
        placeholder={ tr!("account") }
        @update={ |mut s, value: &str| {
            s.account = value.to_string();
            s.ctx.update();
        } }
    />
    <TextInput<_>
        value={ &self.pwd }
        placeholder={ tr!("password") }
        @update={ |mut s, value: &str| {
            s.pwd = value.to_string();
            s.ctx.update();
        } }
    />
    <if { self.is_register }>
        <TextInput<_>
            value={ &self.pwd2 }
            placeholder={ tr!("retype-password") }
            @update={ |mut s, value: &str| {
                s.pwd2 = value.to_string();
                s.ctx.update();
            } }
        />
        <TextInput<_>
            value={ &self.email }
            placeholder={ tr!("email") }
            @update={ |mut s, value: &str| {
                s.email = value.to_string();
                s.ctx.update();
            } }
        />
    </if>
    <Button<_>
        @press={|mut s, _| {
            if s.is_register { s.register() } else { s.login() }
        } }
    >
        { if self.is_register { "register" } else { "Login" } }
    </Button>
});

skin!(LOGIN = r#"

"#);

pub struct Login<B: Backend> {
    ctx: ComponentContext<B, Self>,
    is_register: bool,
    account: String,
    pwd: String,
    pwd2: String,
    email: String,
    hints: Vec<LoginResp>,
}

impl<B: Backend> Component<B> for Login<B> {
    fn new(ctx: ComponentContext<B, Self>) -> Self {
        Self {
            ctx,
            is_register: false,
            account: String::new(),
            pwd: String::new(),
            pwd2: String::new(),
            email: String::new(),
            hints: vec![],
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
        self.ctx.tick_with_component_rc(|this| {
            crate::run_client_async(async move {
                match crate::client_request_channel().request("/backstage/login", &req).await {
                    Err(e) => error!("{}", e),
                    Ok(resp) => {
                        let resp: LoginResp = resp;
                        let mut this = this.borrow_mut();
                        this.hints.push(resp.clone());
                        this.ctx.update();
                        if let LoginResp::Success = resp {
                            let query: home::Query = Default::default();
                            crate::redirect_to("/backstage", query);
                        }
                    }
                }
            });
        });
    }

    fn register(&mut self) {
        if self.pwd != self.pwd2 {
            return
        }
        let req = RegisterReq {
            account: self.account.clone(),
            name: self.account.clone(),
            pwd: self.pwd.clone(),
            email: self.email.clone(),
        };
        self.ctx.tick_with_component_rc(|this| {
            crate::run_client_async(async move {
                match crate::client_request_channel().request("/backstage/register", &req).await {
                    Err(e) => error!("{}", e),
                    Ok(resp) => {
                        let resp: RegisterResp = resp;
                        let mut this = this.borrow_mut();
                        // this.hints.push(resp.clone());
                        this.ctx.update();
                        if let RegisterResp::Success = resp {
                            let query = login::Query {
                                username: this.account.clone(),
                            };
                            crate::redirect_to("/backstage/login", query);
                        }
                    }
                }
            });
        });
    }
}
