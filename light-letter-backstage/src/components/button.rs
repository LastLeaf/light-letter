use maomi::prelude::*;

use super::*;

template!(xml<B: Backend> for<B> Button<B> ~COMPONENTS {
    <div
        class="button"
        @tap={ |s, _| {
            // s.press.new_event().trigger(s, &());
        } }
    >
        <slot />
    </div>
});
pub struct Button<B: Backend> {
    #[allow(dead_code)] ctx: ComponentContext<B, Self>,
    pub press: Ev<B, ()>,
}
impl<B: Backend> Component<B> for Button<B> {
    fn new(ctx: ComponentContext<B, Self>) -> Self {
        Self {
            ctx,
            press: Ev::new(),
        }
    }
}
impl<B: Backend> Button<B> {
    // empty
}
