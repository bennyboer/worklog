use crate::Size;
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Rect, RenderContext, UpdateCtx, Widget,
};

/// A simple horizontal separator.
pub(crate) struct HorizontalSeparator {
    /// Width of the separator line.
    line_width: f64,
    /// Color of the line.
    color: Color,
}

impl HorizontalSeparator {
    /// Create a new horizontal separator widget.
    pub fn new(line_width: f64, color: Color) -> HorizontalSeparator {
        HorizontalSeparator { line_width, color }
    }
}

impl Widget<()> for HorizontalSeparator {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut (), _env: &Env) {
        // Nothing to do
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &(), _env: &Env) {
        // Nothing to do
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &(), _data: &(), _env: &Env) {
        // Nothing to do
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &(),
        _env: &Env,
    ) -> Size {
        Size::new(bc.max().width, self.line_width)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &(), _env: &Env) {
        let size = ctx.size();

        let rect = Rect::from_points((0.0, 0.0), (size.width, size.height))
            .to_rounded_rect(self.line_width / 2.0);

        ctx.fill(rect, &self.color);
    }
}
