use crate::state::work_item::{UiWorkItem, UiWorkItemStatus};
use crate::state::{DayViewState, SelectedWorkItemLens};
use crate::util::icon;
use crate::widget::button::UiButton;
use crate::widget::day_view::controller;
use crate::widget::day_view::work_item::{WorkItemListItemWidget, ITEM_CHANGED};
use crate::widget::editable_field::EditableFieldWidget;
use crate::widget::horizontal_separator::HorizontalSeparator;
use crate::widget::sidebar::{SideBar, OPEN_SIDEBAR};
use crate::widget::stack::Stack;
use crate::widget::tag_cloud::TagCloud;
use crate::widget::when::When;
use crate::{state, Size};
use druid::widget::{
    ControllerHost, CrossAxisAlignment, Flex, IdentityWrapper, Label, LensWrap, LineBreaking, List,
    MainAxisAlignment, Maybe, Padding, Scroll, SizedBox, Svg, TextBox,
};
use druid::{
    lens, BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, TextAlignment, UpdateCtx, Widget, WidgetExt, WidgetId, WidgetPod,
};
use std::cell::RefCell;
use std::rc::Rc;

/// Widget ID of the sidebar.
const SIDEBAR_WIDGET_ID: WidgetId = WidgetId::reserved(2);

/// Widget ID of the item list.
const ITEM_LIST_WIDGET_ID: WidgetId = WidgetId::reserved(3);

/// Widget displaying work items for a day.
pub(crate) struct DayViewWidget {
    /// Child widget
    child: WidgetPod<state::DayViewState, Box<dyn Widget<state::DayViewState>>>,
}

impl DayViewWidget {
    /// Create a new instance of the day view widget.
    pub fn new() -> IdentityWrapper<ControllerHost<DayViewWidget, controller::DayViewController>> {
        DayViewWidget {
            child: WidgetPod::new(
                Stack::new()
                    .with_child(
                        Flex::column()
                            .main_axis_alignment(MainAxisAlignment::Start)
                            .with_child(LensWrap::new(build_header(), state::DayViewState::date))
                            .with_spacer(10.0)
                            .with_flex_child(
                                Maybe::new(
                                    || build_day_view_work_items(),
                                    || build_placeholder().lens(lens::Unit),
                                )
                                .lens(state::DayViewState::work_items),
                                1.0,
                            ),
                    )
                    .with_child(
                        build_detail_view_wrapper().lens(state::DayViewState::selected_work_item),
                    )
                    .boxed(),
            ),
        }
        .controller(controller::DayViewController)
        .with_id(controller::DAY_VIEW_WIDGET_ID)
    }
}

/// Build the detail view of a work item (if one is selected).
fn build_detail_view_wrapper() -> impl Widget<Option<Rc<RefCell<UiWorkItem>>>> {
    Maybe::new(
        || {
            SideBar::new(
                build_detail_view().lens(SelectedWorkItemLens),
                false,
                true,
                |ctx, _, _| {
                    ctx.submit_command(
                        controller::SELECT_ITEM
                            .with(-1)
                            .to(controller::DAY_VIEW_WIDGET_ID),
                    );
                },
            )
        },
        || SizedBox::empty().lens(lens::Unit),
    )
    .with_id(SIDEBAR_WIDGET_ID)
}

fn build_detail_view() -> impl Widget<UiWorkItem> {
    Scroll::new(Padding::new(
        (15.0, 10.0),
        Flex::column()
            .main_axis_alignment(MainAxisAlignment::Start)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(build_detail_view_title())
            .with_child(
                HorizontalSeparator::new(4.0, Color::rgb(0.9, 0.9, 0.9))
                    .lens(lens::Unit)
                    .padding((100.0, 10.0)),
            )
            .with_child(build_detail_view_status())
            .with_child(
                HorizontalSeparator::new(4.0, Color::rgb(0.9, 0.9, 0.9))
                    .lens(lens::Unit)
                    .padding((100.0, 10.0)),
            )
            .with_child(
                Label::new("Tags")
                    .with_text_size(20.0)
                    .center()
                    .padding((0.0, 0.0, 0.0, 10.0)),
            )
            .with_child(build_detail_view_tags())
            .with_child(
                HorizontalSeparator::new(4.0, Color::rgb(0.9, 0.9, 0.9))
                    .lens(lens::Unit)
                    .padding((100.0, 10.0)),
            )
            .with_child(Label::new(|data: &UiWorkItem, _: &Env| {
                let work_item = data.work_item.as_ref().borrow();

                format!(
                    "Duration: {}",
                    shared::time::format_duration((work_item.time_taken() / 1000) as u32)
                )
            })),
    ))
    .vertical()
    .background(Color::WHITE)
    .fix_width(400.0)
    .expand_height()
}

fn build_detail_view_tags() -> impl Widget<UiWorkItem> {
    TagCloud::new(|ctx, data| {
        ctx.submit_command(ITEM_CHANGED.with(data.id).to(ITEM_LIST_WIDGET_ID))
    })
}

fn build_detail_view_status() -> impl Widget<UiWorkItem> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(
            Label::new("Status: ")
                .with_text_size(18.0)
                .with_text_color(Color::rgb8(120, 120, 120)),
        )
        .with_child(
            Label::new(|data: &UiWorkItem, _: &Env| {
                format!(
                    "{}",
                    match data.status {
                        UiWorkItemStatus::InProgress => "In progress",
                        UiWorkItemStatus::Paused => "Paused",
                        UiWorkItemStatus::Finished => "Done",
                    }
                )
            })
            .with_text_size(18.0),
        )
        .with_flex_spacer(1.0)
        .with_child(build_detail_view_status_buttons())
}

fn build_detail_view_status_buttons() -> impl Widget<UiWorkItem> {
    Flex::row()
        .with_child(When::new(
            |data: &UiWorkItem| data.status == UiWorkItemStatus::InProgress,
            UiButton::new(Label::new("Pause").padding((4.0, 2.0)))
                .with_color(Color::rgb8(255, 179, 102))
                .on_click(|ctx, data: &mut UiWorkItem, _| {
                    // Update UI work item
                    data.status = UiWorkItemStatus::Paused;

                    // Update work item in backend
                    let mut work_item = data.work_item.borrow_mut();
                    work_item.pause_working().unwrap();
                    persistence::update_items(vec![&work_item]).unwrap();

                    // Notify list item that it needs to update as well
                    ctx.submit_command(ITEM_CHANGED.with(data.id).to(ITEM_LIST_WIDGET_ID));

                    ctx.request_update();
                }),
        ))
        .with_spacer(4.0)
        .with_child(When::new(
            |data: &UiWorkItem| data.status == UiWorkItemStatus::Paused,
            UiButton::new(Label::new("Continue").padding((4.0, 2.0)))
                .with_color(Color::rgb8(102, 204, 153))
                .on_click(|ctx, data: &mut UiWorkItem, _| {
                    // Update UI work item
                    data.status = UiWorkItemStatus::InProgress;

                    // Update work item in backend
                    let mut work_item = data.work_item.borrow_mut();
                    work_item.continue_working().unwrap();
                    persistence::update_items(vec![&work_item]).unwrap();

                    // Notify list item that it needs to update as well
                    ctx.submit_command(ITEM_CHANGED.with(data.id).to(ITEM_LIST_WIDGET_ID));

                    ctx.request_update();
                }),
        ))
        .with_spacer(4.0)
        .with_child(When::new(
            |data: &UiWorkItem| data.status != UiWorkItemStatus::Finished,
            UiButton::new(Label::new("Finish").padding((4.0, 2.0)))
                .with_color(Color::rgb8(140, 140, 140))
                .on_click(|ctx, data: &mut UiWorkItem, _| {
                    // Update UI work item
                    data.status = UiWorkItemStatus::Finished;

                    // Update work item in backend
                    let mut work_item = data.work_item.borrow_mut();
                    work_item.finish_working(None).unwrap();
                    persistence::update_items(vec![&work_item]).unwrap();

                    // Notify list item that it needs to update as well
                    ctx.submit_command(ITEM_CHANGED.with(data.id).to(ITEM_LIST_WIDGET_ID));

                    ctx.request_update();
                }),
        ))
}

fn build_detail_view_title() -> impl Widget<UiWorkItem> {
    let title_edit_id = WidgetId::next();

    let non_editing_widget = Label::new(|data: &UiWorkItem, _: &Env| data.description.to_owned())
        .with_line_break_mode(LineBreaking::WordWrap)
        .with_text_size(20.0)
        .padding(2.0)
        .expand_width();

    let editing_widget = TextBox::multiline()
        .with_text_size(20.0)
        .lens(UiWorkItem::description)
        .expand_width();

    EditableFieldWidget::new(
        non_editing_widget,
        editing_widget,
        |ctx, data: &mut UiWorkItem, _| {
            // Update work item in backend
            let mut work_item = data.work_item.borrow_mut();
            work_item.set_description(data.description.to_owned());
            persistence::update_items(vec![&work_item]).unwrap();

            // Notify list item that it needs to update as well
            ctx.submit_command(ITEM_CHANGED.with(data.id).to(ITEM_LIST_WIDGET_ID));
        },
    )
    .with_react_on_enter()
    .with_react_on_dbl_click()
    .with_id(title_edit_id)
}

/// Build placeholder for no items.
fn build_placeholder() -> impl Widget<()> {
    Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(Svg::new(icon::get_icon(icon::SLOTH)).fix_height(150.0))
        .with_spacer(30.0)
        .with_child(
            Label::new("No work items for the day!")
                .with_text_size(24.0)
                .with_line_break_mode(LineBreaking::WordWrap)
                .with_text_alignment(TextAlignment::Center)
                .with_text_color(Color::rgb8(100, 100, 100))
                .fix_width(300.0),
        )
        .expand_height()
}

fn build_day_view_work_items() -> impl Widget<state::DayViewWorkItems> {
    Scroll::new(LensWrap::new(
        List::new(|| build_work_item_widget().lens(SelectedWorkItemLens)),
        state::DayViewWorkItems::items,
    ))
    .vertical()
    .with_id(ITEM_LIST_WIDGET_ID)
}

fn build_work_item_widget() -> impl Widget<UiWorkItem> {
    WorkItemListItemWidget::new()
        .on_click(|ctx, item, _| {
            ctx.submit_command(
                controller::SELECT_ITEM
                    .with(item.id)
                    .to(controller::DAY_VIEW_WIDGET_ID),
            );
            ctx.submit_command(OPEN_SIDEBAR.to(SIDEBAR_WIDGET_ID));
        })
        .background(Color::WHITE)
        .rounded(2.0)
        .padding((10.0, 4.0))
}

/// Build the header of the day view.
fn build_header() -> impl Widget<Rc<chrono::Date<chrono::Local>>> {
    let arrow_left_svg = icon::get_icon(icon::ARROW_LEFT);
    let arrow_right_svg = icon::get_icon(icon::ARROW_RIGHT);

    Flex::row()
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .with_spacer(10.0)
        .with_child(
            UiButton::new(Svg::new(arrow_left_svg).fix_width(18.0).padding(8.0))
                .with_corner_radius(100.0)
                .on_click(|ctx, _, _| {
                    ctx.submit_command(controller::PREV_DAY.to(controller::DAY_VIEW_WIDGET_ID))
                }),
        )
        .with_flex_spacer(1.0)
        .with_child(build_header_date_label())
        .with_flex_spacer(1.0)
        .with_child(
            UiButton::new(Svg::new(arrow_right_svg).fix_width(18.0).padding(8.0))
                .with_corner_radius(100.0)
                .on_click(|ctx, _, _| {
                    ctx.submit_command(controller::NEXT_DAY.to(controller::DAY_VIEW_WIDGET_ID))
                }),
        )
        .with_spacer(10.0)
        .padding((0.0, 10.0))
}

/// Build the date label for the day view header.
fn build_header_date_label() -> Label<Rc<chrono::Date<chrono::Local>>> {
    Label::dynamic(|date_ref: &Rc<chrono::Date<chrono::Local>>, _| {
        date_ref.as_ref().format("%A, %d. %B").to_string()
    })
    .with_text_size(32.0)
}

impl Widget<state::DayViewState> for DayViewWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DayViewState, env: &Env) {
        self.child.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &DayViewState,
        env: &Env,
    ) {
        self.child.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &DayViewState,
        data: &DayViewState,
        env: &Env,
    ) {
        self.child.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &DayViewState,
        env: &Env,
    ) -> Size {
        self.child.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &DayViewState, env: &Env) {
        self.child.paint(ctx, data, env);
    }
}
