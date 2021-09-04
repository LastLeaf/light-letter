use maomi::prelude::*;

use super::*;

/// Hint message type
pub enum HintKind {
    Error,
    Warn,
    Info,
    Common,
}

template!(xml<B: Backend> for<B> HintArea<B> ~COMPONENTS {
    <div
        class="hint-area"
    >
        <for (index, item) in {self.messages.iter()}>
            <div
                class={
                    match item.0 {
                        HintKind::Error => "hint hint-error",
                        HintKind::Warn => "hint hint-warn",
                        HintKind::Info => "hint hint-info",
                        HintKind::Common => "hint hint-common",
                    }
                }
                // @tap={ move |mut s, _| {
                //     s.remove(index);
                // } }
            >
                { &item.1 }
            </div>
        </for>
    </div>
});

/// An area to show hints.
/// Use `show_*` method to show a message.
pub struct HintArea<B: Backend> {
    #[allow(dead_code)] ctx: ComponentContext<B, Self>,
    messages: Vec<(HintKind, String)>,
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

    fn remove(&mut self, index: usize) {
        self.messages.remove(index);
        self.ctx.update();
    }

    /// Show an error
    #[allow(dead_code)]
    pub fn show_error(&mut self, message: &str) {
        self.messages.push((HintKind::Error, message.to_owned()));
        self.ctx.update();
    }

    /// Show a warning
    #[allow(dead_code)]
    pub fn show_warn(&mut self, message: &str) {
        self.messages.push((HintKind::Warn, message.to_owned()));
        self.ctx.update();
    }

    /// Show an infomative message
    #[allow(dead_code)]
    pub fn show_info(&mut self, message: &str) {
        self.messages.push((HintKind::Info, message.to_owned()));
        self.ctx.update();
    }

    /// Show a common hint
    #[allow(dead_code)]
    pub fn show_common(&mut self, message: &str) {
        self.messages.push((HintKind::Common, message.to_owned()));
        self.ctx.update();
    }

    /// Show a hint with specified type
    #[allow(dead_code)]
    pub fn show(&mut self, kind: HintKind, message: &str) {
        self.messages.push((kind, message.to_owned()));
        self.ctx.update();
    }
}
