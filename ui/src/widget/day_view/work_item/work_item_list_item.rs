use crate::Size;
use druid::{
    BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, RenderContext, UpdateCtx, Widget, WidgetPod,
};

pub(crate) struct WorkItemListItemWidget<T> {
    child: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T> WorkItemListItemWidget<T> {
    pub fn new(child: impl Widget<T> + 'static) -> WorkItemListItemWidget<T> {
        WorkItemListItemWidget {
            child: WidgetPod::new(child).boxed(),
        }
    }
}

impl<T: Data> Widget<T> for WorkItemListItemWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.child.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }

        self.child.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.child.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.child.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = ctx.size().to_rounded_rect(2.0);

        let color = if ctx.is_hot() {
            Color::rgb8(230, 240, 250)
        } else {
            Color::WHITE
        };

        ctx.fill(size, &color);

        self.child.paint(ctx, data, env);
    }
}
