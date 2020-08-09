use sha2::{Sha256, Digest};
use maomi::prelude::*;
use light_letter_rpc::backstage::login::*;

use crate::PageMetaData;
use super::*;

const PWD_SALT: &'static str = "~light~letter";

fn hash_pwd(unique_salt: &str, pwd_str: &str) -> String {
    let mut s = Sha256::new();
    s.input((String::new() + unique_salt + pwd_str + PWD_SALT).as_bytes());
    format!("{:x}", s.result())
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Query {
    username: String,
}

template!(xml<B: Backend> for<B> Login<B> ~LOGIN {
    <HintArea<_> mark="hint" />
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
    component_common!();

    fn login(&mut self) {
        let req = LoginReq {
            account: self.account.clone(),
            pwd: hash_pwd(&self.account.to_lowercase(), &self.pwd),
        };
        self.ctx.tick_with_component_rc(|this| {
            crate::run_client_async(async move {
                match crate::client_request_channel().request("/backstage/login", &req).await {
                    Err(e) => error!("{}", e),
                    Ok(resp) => {
                        let resp: LoginResp = resp;
                        let mut this = this.borrow_mut();
                        let (k, m) = match resp {
                            LoginResp::Success => (HintKind::Info, tr!("login-success")),
                            LoginResp::IdIllegal => (HintKind::Error, tr!("username-illegal")),
                            LoginResp::NoSuchAccount => (HintKind::Error, tr!("user-not-exists")),
                            LoginResp::WrongPassword => (HintKind::Error, tr!("wrong-password")),
                        };
                        Self::hint(&mut this, k, m);
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
            pwd: hash_pwd(&self.account.to_lowercase(), &self.pwd),
            email: self.email.clone(),
        };
        self.ctx.tick_with_component_rc(|this| {
            crate::run_client_async(async move {
                match crate::client_request_channel().request("/backstage/register", &req).await {
                    Err(e) => error!("{}", e),
                    Ok(resp) => {
                        let resp: RegisterResp = resp;
                        let mut this = this.borrow_mut();
                        let (k, m) = match resp {
                            RegisterResp::Success => (HintKind::Info, tr!("registration-success")),
                            RegisterResp::IdIllegal => (HintKind::Error, tr!("username-illegal")),
                            RegisterResp::IdUsed => (HintKind::Error, tr!("user-already-exists")),
                            RegisterResp::Denied => (HintKind::Error, tr!("registration-denied")),
                        };
                        Self::hint(&mut this, k, m);
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
