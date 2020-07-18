use light_letter_theme_ivy_leaf;

// TODO use dylib to load theme
pub(crate) fn get(name: &str) -> Option<Box<dyn light_letter_web::Theme>> {
    match name {
        "light_letter_theme_ivy_leaf" => Some(Box::new(light_letter_theme_ivy_leaf::Theme::new())),
        _ => None,
    }
}
