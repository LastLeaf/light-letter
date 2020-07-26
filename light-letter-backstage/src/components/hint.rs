use maomi::prelude::*;

use super::*;

enum MessageKind {
    Error,
    Warn,
    Info,
    Common,
}

template!(xml<B: Backend> for<B> HintArea<B> ~COMPONENTS {
    <div
        class="hint-area"
    >
        <for item in {self.messages.iter()}>
            <div class={
                match item.0 {
                    MessageKind::Error => "hint-error",
                    MessageKind::Warn => "hint-warn",
                    MessageKind::Info => "hint-info",
                    MessageKind::Common => "hint-common",
                }
            }>
                { &item.1 }
            </div>
        </for>
    </div>
});
pub struct HintArea<B: Backend> {
    #[allow(dead_code)] ctx: ComponentContext<B, Self>,
    messages: Vec<(MessageKind, String)>,
}
impl<B: Backend> Component<B> for HintArea<B> {
    fn new(ctx: ComponentContext<B, Self>) -> Self {
        Self {
            ctx,
            messages: Vec::new(),
        }
    }
}
impl<B: Backend> HintArea<B> {
    #[allow(dead_code)]
    pub(crate) fn show_error(&mut self, message: &str) {
        self.messages.push((MessageKind::Error, message.to_owned()));
        self.ctx.update();
    }
    #[allow(dead_code)]
    pub(crate) fn show_warn(&mut self, message: &str) {
        self.messages.push((MessageKind::Warn, message.to_owned()));
        self.ctx.update();
    }
    #[allow(dead_code)]
    pub(crate) fn show_info(&mut self, message: &str) {
        self.messages.push((MessageKind::Info, message.to_owned()));
        self.ctx.update();
    }
    #[allow(dead_code)]
    pub(crate) fn show(&mut self, message: &str) {
        self.messages.push((MessageKind::Common, message.to_owned()));
        self.ctx.update();
    }
}
