use maomi::prelude::*;
use maomi::backend::BackendElement;

use super::*;

template!(xml<B: Backend> for<B> TextInput<B> ~COMPONENTS {
    <input
        mark="input"
        r#type={ if self.is_password { "password" } else { "text" } }
        class="input"
        value={ &self.value }
        placeholder={ &self.placeholder }
        // @key_up={ |mut s, _| {
        //     let text = s.marked_native_node("input").unwrap().borrow_mut_with(&mut s).backend_element().get_field("value").and_then(|x| x.as_string());
        //     s.update.new_event().trigger(s, &text.unwrap_or_default());
        // } }
    ></input>
});
pub struct TextInput<B: Backend> {
    #[allow(dead_code)] ctx: ComponentContext<B, Self>,
    pub is_password: bool,
    pub value: String,
    pub placeholder: String,
    pub update: Ev<B, str>,
}
impl<B: Backend> Component<B> for TextInput<B> {
    fn new(ctx: ComponentContext<B, Self>) -> Self {
        Self {
            ctx,
            is_password: false,
            value: String::new(),
            placeholder: String::new(),
            update: Ev::new(),
        }
    }
}
impl<B: Backend> TextInput<B> {
    // empty
}
