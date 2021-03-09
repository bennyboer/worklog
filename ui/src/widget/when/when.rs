use crate::Size;
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    UpdateCtx, Widget, WidgetExt, WidgetPod,
};

/// Widget that will only display something when a condition is met.
pub(crate) struct When<T> {
    /// Whether the condition is currently met.
    condition_met: bool,
    /// Condition that is evaluated on the data.
    condition: Box<dyn Fn(&T) -> bool>,
    /// Widget to display when the condition is met.
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T> When<T>
where
    T: Data,
{
    pub fn new(
        condition: impl Fn(&T) -> bool + 'static,
        inner: impl Widget<T> + 'static,
    ) -> When<T> {
        When {
            condition_met: false,
            condition: Box::new(condition),
            inner: WidgetPod::new(inner.boxed()),
        }
    }
}

impl<T> Widget<T> for When<T>
where
    T: Data,
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if self.condition_met || event.should_propagate_to_hidden() {
            self.inner.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if matches!(event, LifeCycle::WidgetAdded) {
            let condition_met: bool = (self.condition)(data);
            if condition_met != self.condition_met {
                self.condition_met = condition_met;
            }
        }

        if self.condition_met || event.should_propagate_to_hidden() {
            self.inner.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        let condition_met: bool = (self.condition)(data);
        if condition_met != self.condition_met {
            self.condition_met = condition_met;
            ctx.request_layout();
        }

        if self.condition_met {
            self.inner.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        if self.condition_met {
            self.inner.layout(ctx, bc, data, env)
        } else {
            Size::ZERO
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        if self.condition_met {
            self.inner.paint(ctx, data, env);
        }
    }
}
