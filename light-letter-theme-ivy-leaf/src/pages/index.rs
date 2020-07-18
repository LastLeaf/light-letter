use maomi::prelude::*;

use crate::PageMetaData;

#[derive(Default, serde::Deserialize)]
pub struct Query {
}

template!(xml<B: Backend> for<B> Index<B> ~INDEX {
    <div class="wrapper">
        <div class="header">
            <div class="title"> "最后的叶子" <span class="title-small"> "的" </span> "神秘故事" </div>
            <div class="subtitle"> "Secret Stories " <span class="subtitle-small"> "of" </span> " LastLeaf" </div>
        </div>
        <div class="body">
            <div class="coming-soon"> {&self.hint} </div>
        </div>
        <div class="footer">
            <div class="copyright"> "Copyright 2014-2020 LastLeaf" </div>
            <div class="engine"> "Powered by " <a target="_blank" href="https://github.com/LastLeaf/light-letter"> "light-letter" </a> ", blog engine for the future rust world." </div>
        </div>
    </div>
});
skin!(INDEX = r#"
    html {
        font-family: sans-serif;
    }
    body {
        font-size: 20px;
        margin: 0;
        background: #c0f2e9;
    }
    .wrapper {
        box-sizing: border-box;
        padding-bottom: footer-size;
    }

    .header {
        text-align: center;
        background-color: #effffc;
        background-image: url("/theme/leaf.svg");
        background-position: top center;
        background-repeat: repeat-x;
    }
    .title {
        font-size: 2em;
        line-height: 2em;
        padding: 140px 0 0;
        color: #53d2ba;
        letter-spacing: 4px;
    }
    .title-small {
        font-size: 0.8em;
        padding: 0 6px;
    }
    .subtitle {
        font-size: 1.2em;
        line-height: 1.5em;
        padding: 0px 0;
        color: #70dfca;
    }
    .subtitle-small {
        font-size: 0.8em;
    }
    @media (max-width: 600px) {
        .title {
            font-size: 1.5em;
            letter-spacing: 3px;
        }
        .subtitle {
            font-size: 1.15em;
        }
    }
    @media (max-width: 400px) {
        .title {
            font-size: 1.3em;
            letter-spacing: 2px;
        }
        .subtitle {
            font-size: 1.1em;
        }
    }

    .body {
        min-height: 300px;
        background: #effffc;
    }
    .coming-soon {
        height: 500px;
        line-height: 500px;
        text-align: center;
        font-size: 2em;
        color: #c0f2e9;
    }

    .footer {
        box-sizing: border-box;
        text-align: center;
        padding: 10px 0;
        background: #c0f2e9;
    }
    .copyright {
        font-size: 0.8em;
        color: #999;
    }
    .engine {
        font-size: 0.8em;
        color: #999;
    }
    .engine a {
        color: #777;
        text-decoration: none;
    }
"#);
pub struct Index<B: Backend> {
    #[allow(dead_code)]
    ctx: ComponentContext<B, Self>,
    hint: String,
}
impl<B: Backend> Component<B> for Index<B> {
    fn new(ctx: ComponentContext<B, Self>) -> Self {
        Self {
            ctx,
            hint: String::new(),
        }
    }
    fn attached(&mut self) {
        self.hint = "Coming Soon...".into();
        self.ctx.update();
    }
}
impl<B: Backend> PrerenderableComponent<B> for Index<B> {
    type Args = crate::ReqArgs<Query>;
    type PrerenderedData = ();
    type MetaData = PageMetaData;
    fn get_prerendered_data(&self, _args: Self::Args) -> std::pin::Pin<Box<dyn futures::Future<Output = (Self::PrerenderedData, Self::MetaData)>>> {
        let meta_data = PageMetaData {
            title: "最后的叶子的神秘故事".into(),
        };
        let prerendered_data = ();
        Box::pin(futures::future::ready((prerendered_data, meta_data)))
    }
    fn apply_prerendered_data(&mut self, _data: &Self::PrerenderedData) {
        // self.ctx.update();
    }
}
impl<B: Backend> Index<B> {
}
