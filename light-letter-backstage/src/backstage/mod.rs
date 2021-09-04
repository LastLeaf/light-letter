use super::components::*;

macro_rules! component_common {
    () => {
        fn hint<'a>(_this: &'a mut ComponentRefMut<B, Self>, _kind: HintKind, _message: impl Into<&'a str>) {
            // this.marked_component::<crate::components::hint::HintArea<B>>("hint").unwrap().borrow_mut_with(this).show(kind, message.into());
        }
    };
}

pub mod login;
pub mod home;
pub mod post;
