use crate::state::{work_item, DayViewState};
use crate::util::icon;
use crate::widget::button::UiButton;
use crate::widget::day_view::controller;
use crate::widget::day_view::work_item::WorkItemListItemWidget;
use crate::{state, Size};
use druid::widget::{
    ControllerHost, Flex, IdentityWrapper, Label, LensWrap, LineBreaking, List, MainAxisAlignment,
    Maybe, Scroll, Svg,
};
use druid::{
    lens, BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, TextAlignment, UpdateCtx, Widget, WidgetExt, WidgetPod,
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
                        Maybe::new(
                            || build_day_view_work_items(),
                            || build_placeholder().lens(lens::Unit),
                        )
                        .lens(state::DayViewState::work_items),
                        1.0,
                    )
                    .boxed(),
            ),
        }
        .controller(controller::DayViewController)
        .with_id(controller::DAY_VIEW_WIDGET_ID)
    }
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
        List::new(|| build_work_item_widget()),
        state::DayViewWorkItems::items,
    ))
    .vertical()
}

fn build_work_item_widget() -> impl Widget<work_item::UiWorkItem> {
    WorkItemListItemWidget::new()
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
