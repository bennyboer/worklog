use crate::state::work_item::UiWorkItemStatus;
use crate::state::{work_item, DayViewState};
use crate::widget::day_view::controller;
use crate::widget::day_view::work_item::WorkItemListItemWidget;
use crate::{state, Size};
use druid::widget::{
    Button, ControllerHost, CrossAxisAlignment, Flex, IdentityWrapper, Label, LensWrap, List,
    MainAxisAlignment, Painter, Scroll,
};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    RenderContext, UpdateCtx, Widget, WidgetExt, WidgetPod,
};
use std::rc::Rc;

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
                Flex::column()
                    .main_axis_alignment(MainAxisAlignment::Start)
                    .with_child(LensWrap::new(build_header(), state::DayViewState::date))
                    .with_spacer(10.0)
                    .with_flex_child(
                        LensWrap::new(build_day_view_work_items(), state::DayViewState::work_items),
                        1.0,
                    )
                    .boxed(),
            ),
        }
        .controller(controller::DayViewController)
        .with_id(controller::DAY_VIEW_WIDGET_ID)
    }
}

fn build_day_view_work_items() -> impl Widget<state::DayViewWorkItems> {
    Scroll::new(LensWrap::new(
        List::new(|| build_work_item_widget()),
        state::DayViewWorkItems::items,
    ))
    .vertical()
}

fn build_work_item_widget() -> impl Widget<work_item::UiWorkItem> {
    WorkItemListItemWidget::new(
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
fn build_header() -> impl Widget<Rc<chrono::Date<chrono::Local>>> {
    Flex::row()
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .with_spacer(10.0)
        .with_child(Button::new("<").on_click(|ctx, _, _| {
            ctx.submit_command(controller::PREV_DAY.to(controller::DAY_VIEW_WIDGET_ID))
        }))
        .with_flex_spacer(1.0)
        .with_child(build_header_date_label())
        .with_flex_spacer(1.0)
        .with_child(Button::new(">").on_click(|ctx, _, _| {
            ctx.submit_command(controller::NEXT_DAY.to(controller::DAY_VIEW_WIDGET_ID))
        }))
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
