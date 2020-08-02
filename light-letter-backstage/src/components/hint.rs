use maomi::prelude::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HintKind {
    Error,
    Warn,
    Info,
    Common,
    Hidden,
}

template!(xml<B: Backend> for<B> HintArea<B> ~COMPONENTS {
    <div class="hint-area"><slot /></div>
});
pub struct HintArea<B: Backend> {
    #[allow(dead_code)] ctx: ComponentContext<B, Self>,
}
impl<B: Backend> Component<B> for HintArea<B> {
    fn new(ctx: ComponentContext<B, Self>) -> Self {
        Self {
            ctx,
        }
    }
}
impl<B: Backend> HintArea<B> { }

template!(xml<B: Backend> for<B> Hint<B> ~COMPONENTS {
    <div class={
        match self.kind {
            HintKind::Error => "hint-error",
            HintKind::Warn => "hint-warn",
            HintKind::Info => "hint-info",
            HintKind::Common => "hint-common",
            HintKind::Hidden => "hint-hidden",
        }
    }>
        { &self.msg }
    </div>
});
pub struct Hint<B: Backend> {
    #[allow(dead_code)] ctx: ComponentContext<B, Self>,
    pub kind: HintKind,
    pub msg: String,
}
impl<B: Backend> Component<B> for Hint<B> {
    fn new(ctx: ComponentContext<B, Self>) -> Self {
        Self {
            ctx,
            kind: HintKind::Hidden,
            msg: String::new(),
        }
    }
}
impl<B: Backend> Hint<B> { }
