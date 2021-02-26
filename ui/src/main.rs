// Hide the console window when not in debug mode for windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod util;
mod widget;

use crate::state::work_item::UiWorkItemStatus;
use crate::widget::ListItemWidget;

use druid::widget::{
    Align, Button, CrossAxisAlignment, Flex, Label, LensWrap, List, MainAxisAlignment, Maybe,
    Painter, Scroll,
};
use druid::{im, RenderContext};
pub use druid::{
    lens, AppLauncher, Color, Data, LensExt, Point, Screen, Size, UnitPoint, Widget, WidgetExt,
    WindowDesc,
};
use persistence::calc::Status;
use state::work_item;
use std::error::Error;
use std::rc::Rc;

/// The apps title.
const APP_TITLE: &str = "Worklog";

/// Entry point of the application.
fn main() -> Result<(), Box<dyn Error>> {
    let today = chrono::Local::today();
    let from_timestamp = today.and_hms(0, 0, 0).timestamp_millis();
    let to_timestamp = today.succ().and_hms(0, 0, 0).timestamp_millis();

    let items = persistence::find_items_by_timerange(from_timestamp, to_timestamp)?;

    // Map items to the UI representation
    let mut ui_work_items = im::Vector::new();
    for item in &items {
        ui_work_items.push_back(work_item::UiWorkItem {
            id: item.id().unwrap(),
            description: item.description().to_owned(),
            status: match item.status() {
                Status::Done => work_item::UiWorkItemStatus::Finished,
                Status::InProgress => work_item::UiWorkItemStatus::InProgress,
                Status::Paused => work_item::UiWorkItemStatus::Paused,
            },
        });
    }

    let state = state::UiState {
        day: state::DayViewState {
            date: Rc::new(chrono::Local::today()),
            work_items: Some(state::DayViewWorkItems {
                items: ui_work_items,
            }),
        },
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
        .with_min_size((400.0, 300.0))
        .set_position(window_position)
        .title(APP_TITLE)
}

/// Build the root widget.
fn build_root_widget() -> impl Widget<state::UiState> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_flex_child(LensWrap::new(build_day_view(), state::UiState::day), 1.0)
        .background(Color::rgb8(250, 250, 250))
        .env_scope(|env, _| {
            util::ui::env::configure_environment(env);
        })
}

/// Build the day view showing all work items of a specific date.
fn build_day_view() -> impl Widget<state::DayViewState> {
    Flex::column()
        .main_axis_alignment(MainAxisAlignment::Start)
        .with_child(LensWrap::new(
            build_day_view_header(),
            state::DayViewState::date,
        ))
        .with_spacer(10.0)
        .with_flex_child(build_day_view_work_item_list_container(), 1.0)
}

/// Build the work item list for the day view.
fn build_day_view_work_item_list_container() -> impl Widget<state::DayViewState> {
    LensWrap::new(
        Maybe::new(
            || build_day_view_work_items(),
            || LensWrap::new(build_day_view_work_item_progress_indicator(), lens::Unit),
        ),
        state::DayViewState::work_items,
    )
}

fn build_day_view_work_item_progress_indicator() -> impl Widget<()> {
    Align::centered(Label::new("Loading..."))
}

fn build_day_view_work_items() -> impl Widget<state::DayViewWorkItems> {
    Scroll::new(LensWrap::new(
        List::new(|| build_work_item_widget()),
        state::DayViewWorkItems::items,
    ))
    .vertical()
}

fn build_work_item_widget() -> impl Widget<work_item::UiWorkItem> {
    ListItemWidget::new(
        Flex::row()
            .main_axis_alignment(MainAxisAlignment::Start)
            .with_child(build_work_item_status_panel().lens(work_item::UiWorkItem::status))
            .with_spacer(10.0)
            .with_flex_child(
                Flex::column()
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .with_child(
                        Label::new(|item: &work_item::UiWorkItem, _env: &_| {
                            format!("{}", item.description)
                        })
                        .with_text_size(18.0),
                    )
                    .with_child(
                        Label::new(|item: &work_item::UiWorkItem, _env: &_| {
                            format!(
                                "{}",
                                match item.status {
                                    UiWorkItemStatus::InProgress => "In progress",
                                    UiWorkItemStatus::Paused => "Paused",
                                    UiWorkItemStatus::Finished => "Done",
                                }
                            )
                        })
                        .with_text_size(12.0)
                        .with_text_color(Color::rgb8(100, 100, 100)),
                    ),
                1.0,
            ),
    )
    .fix_height(50.0)
    .background(Color::WHITE)
    .rounded(2.0)
    .padding((10.0, 4.0))
}

fn build_work_item_status_panel() -> impl Widget<work_item::UiWorkItemStatus> {
    Painter::new(|ctx, status: &work_item::UiWorkItemStatus, _: &_| {
        let size = ctx.size().to_rounded_rect(2.0);

        let color = match *status {
            UiWorkItemStatus::InProgress => Color::rgb8(130, 200, 50),
            UiWorkItemStatus::Paused => Color::rgb8(216, 139, 100),
            UiWorkItemStatus::Finished => Color::rgb8(100, 177, 216),
        };

        ctx.fill(size, &color)
    })
    .fix_width(4.0)
}

/// Build the header of the day view.
fn build_day_view_header() -> impl Widget<Rc<chrono::Date<chrono::Local>>> {
    Flex::row()
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .with_spacer(10.0)
        .with_child(Button::new("<"))
        .with_flex_spacer(1.0)
        .with_child(build_day_view_header_date_label())
        .with_flex_spacer(1.0)
        .with_child(Button::new(">"))
        .with_spacer(10.0)
        .padding((0.0, 10.0))
}

/// Build the date label for the day view header.
fn build_day_view_header_date_label() -> Label<Rc<chrono::Date<chrono::Local>>> {
    Label::dynamic(|date_ref: &Rc<chrono::Date<chrono::Local>>, _| {
        date_ref.as_ref().format("%A, %d. %B").to_string()
    })
    .with_text_size(32.0)
}
