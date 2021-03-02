use crate::state::work_item::{UiWorkItem, UiWorkItemStatus};
use crate::Size;
use druid::widget::{
    Controller, ControllerHost, CrossAxisAlignment, Flex, Label, LineBreaking, List,
    MainAxisAlignment, Painter,
};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    LinearGradient, PaintCtx, Point, Rect, RenderContext, Selector, UnitPoint, UpdateCtx, Widget,
    WidgetExt, WidgetId, WidgetPod,
};

const HOVER_CHANGED: Selector = Selector::new("work-item-header.hover-changed");

pub(crate) struct WorkItemListItemWidget {
    header_id: WidgetId,
    child: WidgetPod<UiWorkItem, Box<dyn Widget<UiWorkItem>>>,
}

impl WorkItemListItemWidget {
    pub fn new() -> WorkItemListItemWidget {
        let header_id = WidgetId::next();
        let item_header = ControllerHost::new(
            ItemHeaderWidget {
                left: WidgetPod::new(
                    Label::new(|item: &UiWorkItem, _env: &_| format!("{}", item.description))
                        .with_text_size(18.0)
                        .with_line_break_mode(LineBreaking::Clip),
                )
                .boxed(),
                right: WidgetPod::new(build_timing_label().padding((10.0, 0.0, 0.0, 0.0))).boxed(),
                hovered: false,
            },
            ItemHeaderWidgetController,
        )
        .with_id(header_id);

        let child = Flex::row()
            .main_axis_alignment(MainAxisAlignment::Start)
            .with_child(build_status_panel().lens(UiWorkItem::status))
            .with_spacer(10.0)
            .with_flex_child(
                Flex::column()
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .with_child(item_header)
                    .with_child(
                        Flex::row()
                            .with_child(build_status_label())
                            .with_flex_spacer(1.0)
                            .with_child(build_tags().lens(UiWorkItem::tags)),
                    ),
                1.0,
            )
            .with_spacer(10.0)
            .fix_height(60.0);

        WorkItemListItemWidget {
            header_id,
            child: WidgetPod::new(child).boxed(),
        }
    }
}

/// Build the work item timing label.
fn build_timing_label() -> Label<UiWorkItem> {
    Label::new(|item: &UiWorkItem, _: &Env| {
        let work_item = item.work_item.as_ref();

        let time_str = shared::time::get_local_date_time(work_item.created_timestamp())
            .format("%H:%M")
            .to_string();
        let duration_str = shared::time::format_duration((work_item.time_taken() / 1000) as u32);

        format!("{} ({})", duration_str, time_str)
    })
    .with_text_size(12.0)
}

/// Build the tags list widget.
fn build_tags() -> impl Widget<im::Vector<String>> {
    List::new(|| build_tag_widget())
        .horizontal()
        .with_spacing(2.0)
}

/// Build a widget representing a tag.
fn build_tag_widget() -> impl Widget<String> {
    let tag_color = rand_color(); // TODO: Use fixed color per tag instead

    Label::new(|text: &String, _: &Env| format!("#{}", text))
        .with_text_color(invert_color(&tag_color))
        .with_text_size(11.0)
        .padding((3.0, 1.0))
        .background(tag_color)
        .rounded(100.0)
}

fn invert_color(color: &Color) -> Color {
    let (red, green, blue, _) = color.as_rgba();
    let sum = red + green + blue;

    if sum < 1.5 {
        Color::WHITE
    } else {
        Color::BLACK
    }
}

fn rand_color() -> Color {
    Color::rgb(
        rand::random::<f64>(),
        rand::random::<f64>(),
        rand::random::<f64>(),
    )
    .with_alpha(0.4)
}

/// Build the status label of the work item.
fn build_status_label() -> Label<UiWorkItem> {
    Label::new(|item: &UiWorkItem, _env: &_| {
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
    .with_text_color(Color::rgb8(100, 100, 100))
}

/// Build the status panel that is part of the work item list item widget.
fn build_status_panel() -> impl Widget<UiWorkItemStatus> {
    Painter::new(|ctx, status: &UiWorkItemStatus, _: &_| {
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

impl Widget<UiWorkItem> for WorkItemListItemWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut UiWorkItem, env: &Env) {
        self.child.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &UiWorkItem,
        env: &Env,
    ) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
            ctx.submit_command(HOVER_CHANGED.to(self.header_id))
        }

        self.child.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &UiWorkItem,
        data: &UiWorkItem,
        env: &Env,
    ) {
        self.child.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &UiWorkItem,
        env: &Env,
    ) -> Size {
        self.child.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &UiWorkItem, env: &Env) {
        let is_hot = ctx.is_hot();
        let size = ctx.size().to_rounded_rect(2.0);

        ctx.fill(size, &get_item_background_color(is_hot));

        self.child.paint(ctx, data, env);
    }
}

struct ItemHeaderWidget {
    left: WidgetPod<UiWorkItem, Box<dyn Widget<UiWorkItem>>>,
    right: WidgetPod<UiWorkItem, Box<dyn Widget<UiWorkItem>>>,
    hovered: bool,
}

impl Widget<UiWorkItem> for ItemHeaderWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut UiWorkItem, env: &Env) {
        self.left.event(ctx, event, data, env);
        self.right.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &UiWorkItem,
        env: &Env,
    ) {
        self.left.lifecycle(ctx, event, data, env);
        self.right.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &UiWorkItem,
        data: &UiWorkItem,
        env: &Env,
    ) {
        self.left.update(ctx, data, env);
        self.right.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &UiWorkItem,
        env: &Env,
    ) -> Size {
        // First layout right item
        let right_size = self.right.layout(ctx, bc, data, env);

        // Give left item all the remaining space
        let left_bc = BoxConstraints::new(
            Size::ZERO,
            Size::new(bc.max().width - right_size.width, bc.max().height),
        );
        let left_size = self.left.layout(ctx, &left_bc, data, env);

        // Translate right widget by the left widget width
        self.right
            .set_origin(ctx, data, env, Point::new(left_bc.max().width, 0.0));

        Size::new(bc.max().width, left_size.height.max(right_size.height))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &UiWorkItem, env: &Env) {
        self.left.paint(ctx, data, env);
        self.right.paint(ctx, data, env);

        // Draw fade gradient for left widget
        let right_rect = self.right.layout_rect();
        let fade_width = right_rect.x0.min(20.0);
        let fade_rect = Rect::new(
            right_rect.x0 - fade_width,
            0.0,
            right_rect.x0,
            ctx.size().height,
        );

        let color = get_item_background_color(self.hovered);
        let transparent_color = get_item_background_color(self.hovered).with_alpha(0.0);

        ctx.fill(
            fade_rect,
            &LinearGradient::new(
                UnitPoint::LEFT,
                UnitPoint::RIGHT,
                (transparent_color, color),
            ),
        );
    }
}

struct ItemHeaderWidgetController;

impl Controller<UiWorkItem, ItemHeaderWidget> for ItemHeaderWidgetController {
    fn event(
        &mut self,
        child: &mut ItemHeaderWidget,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut UiWorkItem,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) => {
                if cmd.is(HOVER_CHANGED) {
                    child.hovered = !child.hovered;
                    ctx.request_paint();
                }
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}

fn get_item_background_color(is_hot: bool) -> Color {
    if is_hot {
        Color::rgb8(245, 245, 245)
    } else {
        Color::WHITE
    }
}
