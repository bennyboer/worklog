use crate::state;
use crate::widget::day_view::DayViewWidget;
use druid::widget::Controller;
use druid::{Env, Event, EventCtx, Selector, Widget, WidgetId};

/// ID of the day view widget.
pub(crate) const DAY_VIEW_WIDGET_ID: WidgetId = WidgetId::reserved(1);

/// Previous day selector.
pub(crate) const PREV_DAY: Selector = Selector::new("day-view.previous-day");

/// Next day selector.
pub(crate) const NEXT_DAY: Selector = Selector::new("day-view.next-day");

/// Controller for the day view widget.
pub(crate) struct DayViewController;

impl Controller<state::DayViewState, DayViewWidget> for DayViewController {
    fn event(
        &mut self,
        child: &mut DayViewWidget,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut state::DayViewState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) => {
                let new_date = if cmd.is(NEXT_DAY) {
                    data.date.succ()
                } else {
                    data.date.pred()
                };

                data.update(new_date);
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}
