use crate::{Color, Size};
use druid::widget::{Controller, ControllerHost};
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, MouseButton,
    PaintCtx, Point, RenderContext, Selector, UpdateCtx, Widget, WidgetExt, WidgetPod,
};

/// Selector used to open the sidebar.
pub(crate) const OPEN_SIDEBAR: Selector = Selector::new("sidebar-widget.open");

/// Selector used to close the sidebar.
pub(crate) const _CLOSE_SIDEBAR: Selector = Selector::new("sidebar-widget.close");

/// A sidebar widget.
pub(crate) struct SideBar<T> {
    /// Duration of the open/close animation.
    anim_duration_ms: u32,
    /// Whether the open/close animation is currently in progress.
    anim_in_progress: bool,
    /// The current progress of the open/close animation where 1.0 is completely visible, and 0.0 is hidden.
    anim_progress: f64,
    /// Curve of the open/close animation.
    anim_curve: Box<dyn Fn(f64) -> f64>,
    /// Close when the backdrop (dark background) has been clicked.
    close_on_backdrop_click: bool,
    /// Whether the sidebar is currently open.
    is_open: bool,
    /// Whether the sidebar will be positioned to the left or to the right.
    is_left: bool,
    /// Callback of when the sidebar is closed.
    on_close: Box<dyn Fn(&mut EventCtx, &mut T, &Env)>,
    /// Child widget pod.
    pod: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T> SideBar<T>
where
    T: Data,
{
    /// Create a new sidebar widget.
    pub fn new(
        inner: impl Widget<T> + 'static,
        is_left: bool,
        close_on_backdrop_click: bool,
        on_close: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<SideBar<T>, SideBarController> {
        SideBar {
            anim_duration_ms: 300,
            anim_in_progress: false,
            anim_progress: 0.0,
            anim_curve: Box::new(|p| {
                // Cubit ease in-out curve
                if p < 0.5 {
                    4.0 * p * p * p
                } else {
                    (p - 1.0) * (2.0 * p - 2.0) * (2.0 * p - 2.0) + 1.0
                }
            }), // Cubic ease in out
            close_on_backdrop_click,
            is_open: false,
            is_left,
            on_close: Box::new(on_close),
            pod: WidgetPod::new(inner.boxed()),
        }
        .controller(SideBarController)
    }
}

impl<T> Widget<T> for SideBar<T>
where
    T: Data,
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::AnimFrame(interval) => {
                let interval_ms = (*interval as f64) / 1_000_000.0;

                let progress_diff = interval_ms / (self.anim_duration_ms as f64);

                if self.is_open {
                    // Progress is added
                    self.anim_progress = (self.anim_progress + progress_diff).min(1.0);
                } else {
                    // Progress is subtracted
                    self.anim_progress = (self.anim_progress - progress_diff).max(0.0);
                }

                ctx.request_layout();

                if (!self.is_open && self.anim_progress == 0.0)
                    || (self.is_open && self.anim_progress == 1.0)
                {
                    self.anim_in_progress = false;

                    if !self.is_open {
                        (self.on_close)(ctx, data, env); // Sidebar has been closed
                    }
                }

                if self.anim_in_progress {
                    ctx.request_anim_frame();
                }
            }
            _ => (),
        }

        self.pod.event(ctx, event, data, env);

        if self.is_open {
            match event {
                Event::MouseDown(_) | Event::MouseUp(_) | Event::MouseMove(_) | Event::Wheel(_) => {
                    match event {
                        Event::MouseDown(mouse_event) => {
                            if mouse_event.button == MouseButton::Left {
                                if self.pod.is_hot() {
                                    ctx.set_active(false);
                                } else {
                                    ctx.set_active(true);
                                }
                            }
                        }
                        Event::MouseUp(mouse_event) => {
                            if ctx.is_active() && mouse_event.button == MouseButton::Left {
                                ctx.set_active(false);

                                if !self.pod.is_hot() {
                                    // Backdrop has been clicked
                                    if self.close_on_backdrop_click {
                                        self.is_open = false;
                                        self.anim_in_progress = true;
                                        ctx.request_anim_frame();
                                    }
                                }
                            }
                        }
                        _ => {}
                    }

                    ctx.set_handled();
                }
                _ => (),
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.pod.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.pod.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = self.pod.layout(ctx, bc, data, env);

        // Set origin of the child based on alignment (left/right) and current open/close animation position.
        let max_size = bc.max();

        let anim_progress = (self.anim_curve)(self.anim_progress);
        let origin = if self.is_left {
            Point::new(-size.width * (1.0 - anim_progress), 0.0)
        } else {
            Point::new(max_size.width - size.width * anim_progress, 0.0)
        };

        self.pod.set_origin(ctx, data, env, origin);

        max_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = ctx.size();

        // Paint background first
        let anim_progress = (self.anim_curve)(self.anim_progress);
        let bg_color = Color::rgba(0.0, 0.0, 0.0, anim_progress * 0.4);
        ctx.fill(size.to_rect(), &bg_color);

        self.pod.paint(ctx, data, env);
    }
}

pub(crate) struct SideBarController;

impl<T: Data> Controller<T, SideBar<T>> for SideBarController {
    fn event(
        &mut self,
        child: &mut SideBar<T>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut T,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) => {
                let start_animation = if cmd.is(OPEN_SIDEBAR) {
                    child.is_open = true;

                    true
                } else if cmd.is(_CLOSE_SIDEBAR) {
                    child.is_open = false;

                    true
                } else {
                    false
                };

                if start_animation {
                    // Reset widget animation state
                    child.anim_in_progress = true;

                    ctx.request_anim_frame();
                }
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}
