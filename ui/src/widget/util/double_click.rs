use crate::Widget;
use druid::widget::Controller;
use druid::{Data, Env, Event, EventCtx, MouseButton, TimerToken};
use std::time::Duration;

/// Timeout in which a second click must occur after the first click
/// to be counted as a double click.
/// In milliseconds.
const DOUBLE_CLICK_TIMEOUT_MS: u64 = 400;

/// Controller for executing an action on a registered double click.
pub(crate) struct DoubleClickController<T> {
    /// Token used to identify the used double click timer.
    double_click_timer_token: Option<TimerToken>,
    /// Action to execute on double click.
    action: Box<dyn Fn(&mut EventCtx, &mut T, &Env)>,
}

impl<T> DoubleClickController<T>
where
    T: Data,
{
    pub fn new(action: impl Fn(&mut EventCtx, &mut T, &Env) + 'static) -> DoubleClickController<T> {
        DoubleClickController {
            double_click_timer_token: None,
            action: Box::new(action),
        }
    }
}

impl<T, W> Controller<T, W> for DoubleClickController<T>
where
    T: Data,
    W: Widget<T>,
{
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(mouse_event) => {
                if mouse_event.button == MouseButton::Left {
                    ctx.set_active(true);
                }
            }
            Event::MouseUp(mouse_event) => {
                let is_click = ctx.is_active() && mouse_event.button == MouseButton::Left;

                if is_click {
                    let is_double_click = self.double_click_timer_token.is_some();

                    if is_double_click {
                        self.double_click_timer_token = None; // Prevent another double click action execution
                        (self.action)(ctx, data, env);
                        ctx.set_handled();
                    } else {
                        // Start double click timer in which a second click must occur
                        self.double_click_timer_token =
                            Some(ctx.request_timer(Duration::from_millis(DOUBLE_CLICK_TIMEOUT_MS)));
                    }
                }
            }
            Event::Timer(timer_token) => {
                if let Some(dbl_click_timer_token) = self.double_click_timer_token {
                    if *timer_token == dbl_click_timer_token {
                        self.double_click_timer_token = None; // Set double click no more possible
                    }
                }
            }
            _ => {}
        }

        child.event(ctx, event, data, env);
    }
}
