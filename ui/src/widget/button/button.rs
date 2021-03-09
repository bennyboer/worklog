use crate::Size;
use druid::widget::{Click, ControllerHost};
use druid::{
    BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, RenderContext, UpdateCtx, Widget, WidgetPod,
};

const DEFAULT_COLOR: Color = Color::rgb8(220, 225, 230);

/// Custom button that may take another widget to display as child.
pub struct UiButton<T> {
    /// Corner radius of the button.
    corner_radius: f64,
    /// Button color when active (mouse pressed).
    active_color: Color,
    /// Button color on hover.
    hover_color: Color,
    /// Default button color.
    color: Color,
    /// Child to display in the button.
    child: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T: Data> UiButton<T> {
    /// Create a new UiButton instance
    /// with the given child widget.
    pub fn new(child: impl Widget<T> + 'static) -> UiButton<T> {
        let button = UiButton {
            corner_radius: 4.0,
            active_color: DEFAULT_COLOR,
            hover_color: DEFAULT_COLOR,
            color: DEFAULT_COLOR,
            child: WidgetPod::new(child).boxed(),
        };

        button.with_color(DEFAULT_COLOR)
    }

    /// Set a custom corner radius.
    pub fn with_corner_radius(mut self, radius: f64) -> Self {
        self.corner_radius = radius;

        self
    }

    /// Set a custom active color.
    pub fn with_color(mut self, color: Color) -> Self {
        let (red, green, blue, alpha) = color.as_rgba();

        self.color = color;
        self.active_color = Color::rgba(red * 0.9, green * 0.9, blue * 0.9, alpha);
        self.hover_color = Color::rgba(
            (red * 1.05).min(1.0),
            (green * 1.05).min(1.0),
            (blue * 1.05).min(1.0),
            alpha,
        );

        self
    }

    /// Specify a callback to be called when the button has been clicked.
    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, Click<T>> {
        ControllerHost::new(self, Click::new(f))
    }
}

impl<T> Widget<T> for UiButton<T>
where
    T: Data,
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    ctx.request_paint();
                }
            }
            _ => (),
        }
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
        let rounded_rect = ctx.size().to_rounded_rect(self.corner_radius);

        let bg_color = if ctx.is_active() {
            &self.active_color
        } else if ctx.is_hot() {
            &self.hover_color
        } else {
            &self.color
        };

        ctx.fill(rounded_rect, bg_color);

        self.child.paint(ctx, data, env);
    }
}
