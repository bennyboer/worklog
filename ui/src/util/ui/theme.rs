use druid::{theme, Color, Env};

/// Configure the theme for druid.
pub fn configure_theme(env: &mut Env) {
    configure_button_theme(env);
}

/// Configure the button theme.
pub fn configure_button_theme(env: &mut Env) {
    env.set(theme::BUTTON_DARK, Color::rgb8(180, 190, 200));
    env.set(theme::BUTTON_LIGHT, Color::rgb8(180, 190, 200));
    env.set(theme::BORDER_LIGHT, Color::rgb8(140, 150, 160));
    env.set(theme::BORDER_DARK, Color::rgb8(120, 130, 140));
}
