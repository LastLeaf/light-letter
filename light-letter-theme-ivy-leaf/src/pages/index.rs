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
            <div class="foreword">
                <span class="foreword-line"> "当翻找出箱底的日记本时，"</span>
                <span class="foreword-line"> "是否还能重聚那段消散的时光？"</span>
            </div>
            <div class="coming-soon"> {&self.hint} </div>
        </div>
        <div class="footer">
            <div class="links"> <a href="/about"> "关于我" </a> </div>
            <div class="copyright"> "Copyright 2014-2021 lastleaf.me" </div>
            <div class="engine"> "Powered by " <a target="_blank" href="https://github.com/LastLeaf/light-letter"> "light-letter" </a> ", blog engine for future rust world." </div>
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
    a {
        text-decoration: none;
        color: #53d2ba;
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
    .foreword {
        text-align: center;
        padding: 80px 0 0;
        max-width: 18em;
        color: #888;
        margin: 0 auto;
    }
    .foreword-line {
        display: block;
    }
    .coming-soon {
        text-align: center;
        font-size: 1.5em;
        height: 40px;
        padding: 20px 0 200px;
        color: #c0f2e9;
    }
    .post {
        max-width: 600px;
        color: #666;
        margin: 0 auto;
        line-height: 1.5;
        padding: 20px;
    }
    .post-title {
        font-size: 1.5em;
        color: #53d2ba;
    }
    .post-content p {
        margin: 0.5em 0;
    }

    .footer {
        box-sizing: border-box;
        text-align: center;
        padding: 10px 0;
        background: #c0f2e9;
        color: #999;
        font-size: 0.8em;
    }
    .footer a {
        color: #777;
    }
    .links {
        margin-bottom: 0.5em;
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

template!(xml<B: Backend> for<B> About<B> ~INDEX {
    <div class="wrapper">
        <div class="header">
            <div class="title"> "最后的叶子" <span class="title-small"> "的" </span> "神秘故事" </div>
            <div class="subtitle"> "Secret Stories " <span class="subtitle-small"> "of" </span> " LastLeaf" </div>
        </div>
        <div class="body">
            <div class="post">
                <div class="post-title"> "关于我" </div>
                <div class="post-content">
                    <p> "最后的叶子 / LastLeaf" </p>
                    <p> "微信公众号：最后的叶子的神秘故事、最后的叶子的奇妙小屋" </p>
                    <p> "Github: " <a href="https://github.com/LastLeaf"> "@LastLeaf" </a> </p>
                    <p> "程序员，现服务于 " <a href="https://github.com/wechat-miniprogram"> "@wechat-miniprogram" </a> </p>
                    <p> "独自生活，热爱美食、花草和独立游戏。" </p>
                </div>
            </div>
        </div>
        <div class="footer">
            <div class="links"> <a href="/"> "首页" </a> </div>
            <div class="copyright"> "Copyright 2014-2021 lastleaf.me" </div>
            <div class="engine"> "Powered by " <a target="_blank" href="https://github.com/LastLeaf/light-letter"> "light-letter" </a> ", blog engine for future rust world." </div>
        </div>
    </div>
});
pub struct About<B: Backend> {
    #[allow(dead_code)]
    ctx: ComponentContext<B, Self>,
    hint: String,
}
impl<B: Backend> Component<B> for About<B> {
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
impl<B: Backend> PrerenderableComponent<B> for About<B> {
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
