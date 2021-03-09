use crate::widget::util::double_click::DoubleClickController;
use crate::Size;
use druid::widget::ControllerHost;
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, KbKey, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Selector, UpdateCtx, Widget, WidgetExt, WidgetPod,
};

/// Selector to change the edit mode of a editable field widget.
pub(crate) const EDIT_MODE_CHANGE: Selector<bool> =
    Selector::new("editable-field-widget.edit-mode-change");

/// Widget representing an editable field.
pub(crate) struct EditableFieldWidget<T> {
    /// Whether the field is currently being edited.
    is_editing: bool,
    /// Whether to switch from edit mode to non-edit mode on enter.
    react_on_enter: bool,
    /// Widget displayed when the field is edited.
    editing_widget: WidgetPod<T, Box<dyn Widget<T>>>,
    /// Widget displayed when the field not currently not edited.
    non_editing_widget: WidgetPod<T, Box<dyn Widget<T>>>,
    /// Callback to be called when the field has been edited.
    on_update_action: Box<dyn Fn(&mut EventCtx, &mut T, &Env)>,
}

impl<T: Data> EditableFieldWidget<T> {
    pub fn new(
        non_editing_widget: impl Widget<T> + 'static,
        editing_widget: impl Widget<T> + 'static,
        on_update_action: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> EditableFieldWidget<T> {
        EditableFieldWidget {
            is_editing: false,
            react_on_enter: false,
            editing_widget: WidgetPod::new(editing_widget.boxed()),
            non_editing_widget: WidgetPod::new(non_editing_widget.boxed()),
            on_update_action: Box::new(on_update_action),
        }
    }

    /// React on double click (jump into edit mode).
    pub fn with_react_on_enter(mut self) -> Self {
        self.react_on_enter = true;

        self
    }

    /// React on double click (jump into edit mode).
    pub fn with_react_on_dbl_click(
        self,
    ) -> ControllerHost<EditableFieldWidget<T>, DoubleClickController<T>> {
        ControllerHost::new(
            self,
            DoubleClickController::new(|ctx, _, _| {
                ctx.submit_command(EDIT_MODE_CHANGE.with(true).to(ctx.widget_id()));
            }),
        )
    }

    fn current_widget(&mut self) -> &mut WidgetPod<T, Box<dyn Widget<T>>> {
        if self.is_editing {
            &mut self.editing_widget
        } else {
            &mut self.non_editing_widget
        }
    }
}

impl<T: Data> Widget<T> for EditableFieldWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        let handle = match event {
            Event::Command(cmd) => {
                match cmd.get(EDIT_MODE_CHANGE) {
                    Some(edit_mode) => {
                        if *edit_mode != self.is_editing {
                            if !*edit_mode {
                                // Leave edit mode -> Update value
                                (self.on_update_action)(ctx, data, env);
                            }

                            self.is_editing = *edit_mode;

                            ctx.request_update();
                            ctx.request_layout();
                        }
                    }
                    None => {}
                };

                false
            }
            Event::KeyDown(key_event) => {
                let is_enter = key_event.key == KbKey::Enter;

                if is_enter && self.react_on_enter {
                    ctx.submit_command(EDIT_MODE_CHANGE.with(false).to(ctx.widget_id()));
                    false
                } else {
                    true
                }
            }
            _ => true,
        };

        if handle {
            if event.should_propagate_to_hidden() {
                self.editing_widget.event(ctx, event, data, env);
                self.non_editing_widget.event(ctx, event, data, env);
            } else {
                self.current_widget().event(ctx, event, data, env);
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if event.should_propagate_to_hidden() {
            self.editing_widget.lifecycle(ctx, event, data, env);
            self.non_editing_widget.lifecycle(ctx, event, data, env);
        } else {
            self.current_widget().lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.current_widget().update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.current_widget().layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.current_widget().paint(ctx, data, env)
    }
}
