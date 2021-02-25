// Hide the console window when not in debug mode for windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod util;

use druid::widget::{Button, Flex, Label, MainAxisAlignment, Padding};
use druid::{AppLauncher, Color, Data, Point, Screen, Size, Widget, WidgetExt, WindowDesc};
use std::error::Error;
use std::sync::Arc;

/// The apps title.
const APP_TITLE: &str = "Worklog";

/// Entry point of the application.
fn main() -> Result<(), Box<dyn Error>> {
    let state = state::UiState {
        test: String::from("Hello World"),
        work_items: Arc::new(Vec::new()),
    };

    // Create and configure main window
    let window_description = configure_window(WindowDesc::new(|| build_root_widget()));

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
        .set_position(window_position)
        .title(APP_TITLE)
}

/// Build the root widget.
fn build_root_widget() -> impl Widget<state::UiState> {
    let test_widget = Label::dynamic(|str: &String, _| str.to_string())
        .with_text_color(Color::rgb8(50, 50, 50))
        .lens(state::UiState::test);

    let label_container = Padding::new(10.0, test_widget);

    let btn = Button::from_label(Label::new("Do something").with_text_color(Color::rgb8(0, 0, 0)))
        .on_click(|_ctx, data: &mut state::UiState, _env| {
            data.test = String::from("The text has been magically changed! WOOOOW!");
        });

    Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(label_container)
        .with_child(btn)
        .background(Color::rgb8(255, 255, 255))
        .env_scope(|env, _| {
            util::ui::env::configure_environment(env);
        })
}
