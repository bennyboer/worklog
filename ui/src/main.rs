// Hide the console window when not in debug mode for windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
pub(crate) mod util;
mod widget;

use crate::widget::day_view;
use druid::widget::{CrossAxisAlignment, Flex, LensWrap};
pub use druid::{im, RenderContext, Selector, WidgetId};
pub use druid::{
    lens, AppLauncher, Color, Data, LensExt, Point, Screen, Size, UnitPoint, Widget, WidgetExt,
    WindowDesc,
};
use std::error::Error;

/// The apps title.
const APP_TITLE: &str = "Worklog";

/// Entry point of the application.
fn main() -> Result<(), Box<dyn Error>> {
    let state = state::UiState {
        day: state::DayViewState::new(chrono::Local::today()),
    };

    // Create and configure main window
    let window_description = configure_window(WindowDesc::new(build_root_widget()));

    // Launch app
    AppLauncher::with_window(window_description).launch(state)?;

    Ok(())
}

/// Create the main window description.
fn configure_window<T>(window_description: WindowDesc<T>) -> WindowDesc<T>
where
    T: Data,
{
    // Find primary monitor
    let monitors = Screen::get_monitors();
    let primary_monitor =
        util::ui::get_primary_monitor(&monitors).expect("Expected to have a primary monitor!");

    // Get available space
    let monitor_rect = primary_monitor.virtual_work_rect();
    let available_space = Size::new(
        monitor_rect.x1 - monitor_rect.x0,
        monitor_rect.y1 - monitor_rect.y0,
    );

    // Calculate initial window size and position
    let initial_window_size = Size::new(available_space.width * 0.3, available_space.height * 0.4);
    let window_position = Point::new(
        monitor_rect.x0 + (available_space.width - initial_window_size.width) / 2.0,
        monitor_rect.y0 + (available_space.height - initial_window_size.height) / 2.0,
    );

    window_description
        .resizable(true)
        .window_size(initial_window_size)
        .with_min_size((500.0, 300.0))
        .set_position(window_position)
        .title(APP_TITLE)
}

/// Build the root widget.
fn build_root_widget() -> impl Widget<state::UiState> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_flex_child(
            LensWrap::new(day_view::DayViewWidget::new(), state::UiState::day),
            1.0,
        )
        .background(Color::rgb8(250, 250, 250))
        .env_scope(|env, _| {
            util::ui::env::configure_environment(env);
        })
}
