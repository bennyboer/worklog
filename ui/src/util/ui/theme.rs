use druid::{theme, Color, Env};

/// Configure the theme for druid.
pub fn configure_theme(env: &mut Env) {
    env.set(theme::BACKGROUND_LIGHT, Color::rgb8(245, 245, 245));
    env.set(theme::BORDER_LIGHT, Color::rgb8(140, 150, 160));
    env.set(theme::BORDER_DARK, Color::rgb8(120, 130, 140));
    env.set(theme::SELECTION_COLOR, Color::rgb8(51, 152, 255));
    env.set(theme::CURSOR_COLOR, Color::rgb8(40, 40, 40));

    configure_button_theme(env);
    configure_label_theme(env);
    configure_scrollbar_theme(env);
}

/// Configure the button theme.
pub fn configure_button_theme(env: &mut Env) {
    env.set(theme::BUTTON_DARK, Color::rgb8(180, 190, 200));
    env.set(theme::BUTTON_LIGHT, Color::rgb8(190, 200, 210));
}

/// Configure the label theme.
pub fn configure_label_theme(env: &mut Env) {
    env.set(theme::LABEL_COLOR, Color::rgb8(20, 20, 20));
}

/// Configure the scrollbar theme.
pub fn configure_scrollbar_theme(env: &mut Env) {
    env.set(theme::SCROLLBAR_BORDER_COLOR, Color::rgb8(40, 40, 40));
    env.set(theme::SCROLLBAR_COLOR, Color::rgb8(40, 40, 40));
}
