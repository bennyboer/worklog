use super::theme;
use druid::Env;

/// Configure the druid environment.
pub fn configure_environment(env: &mut Env) {
    theme::configure_theme(env);
}
