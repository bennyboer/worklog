use crate::state::work_item::{UiWorkItem, UiWorkItemStatus};
use crate::Size;
use druid::widget::{CrossAxisAlignment, Flex, Label, LineBreaking, MainAxisAlignment, Painter};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    RenderContext, UpdateCtx, Widget, WidgetExt, WidgetPod,
};

pub(crate) struct WorkItemListItemWidget {
    child: WidgetPod<UiWorkItem, Box<dyn Widget<UiWorkItem>>>,
}

impl WorkItemListItemWidget {
    pub fn new() -> WorkItemListItemWidget {
        let child = Flex::row()
            .main_axis_alignment(MainAxisAlignment::Start)
            .with_child(build_status_panel().lens(UiWorkItem::status))
            .with_spacer(10.0)
            .with_flex_child(
                Flex::column()
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .with_child(
                        Label::new(|item: &UiWorkItem, _env: &_| format!("{}", item.description))
                            .with_text_size(18.0)
                            .with_line_break_mode(LineBreaking::Clip),
                    )
                    .with_child(
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
                        .with_text_color(Color::rgb8(100, 100, 100)),
                    ),
                1.0,
            );

        WorkItemListItemWidget {
            child: WidgetPod::new(child).boxed(),
        }
    }
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
        let size = ctx.size().to_rounded_rect(2.0);

        let color = if ctx.is_hot() {
            Color::rgb8(245, 245, 245)
        } else {
            Color::WHITE
        };

        ctx.fill(size, &color);

        self.child.paint(ctx, data, env);
    }
}
