use light_letter_theme_ivy_leaf;

pub(crate) fn get(name: &str) -> impl light_letter_web::Theme {
    match name {
        _ => light_letter_theme_ivy_leaf::Theme::new(),
    }
}
