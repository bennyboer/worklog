use crate::state::work_item::UiWorkItem;
use crate::util::icon;
use crate::widget::button::UiButton;
use crate::widget::editable_field::{EditableFieldWidget, EDIT_MODE_CHANGE};
use crate::{Size, WidgetId};
use druid::widget::{
    Align, CrossAxisAlignment, Flex, Label, ListIter, MainAxisAlignment, Svg, TextBox,
};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Point, Selector, UnitPoint, UpdateCtx, Widget, WidgetExt, WidgetPod,
};
use std::cmp::Ordering;

/// Selector used to delete a tag.
const DELETE_TAG: Selector<String> = Selector::new("tag-cloud.delete");

/// Selector used to notify the tag cloud of an added tag.
const TAG_ADDED: Selector = Selector::new("tag-cloud.added");

/// Cloud of tags (of a work item).
pub(crate) struct TagCloud {
    /// Children displaying the tags.
    children: Vec<WidgetPod<String, Box<dyn Widget<String>>>>,
    add_widget: WidgetPod<UiWorkItem, Box<dyn Widget<UiWorkItem>>>,
    on_update: Box<dyn Fn(&mut EventCtx, &UiWorkItem)>,
}

impl TagCloud {
    pub fn new(on_update: impl Fn(&mut EventCtx, &UiWorkItem) + 'static) -> TagCloud {
        let editing_widget_id = WidgetId::next();

        TagCloud {
            children: Vec::new(),
            add_widget: WidgetPod::new(Box::new(
                EditableFieldWidget::new(
                    UiButton::new(Svg::new(icon::get_icon(icon::ADD)).fix_size(24.0, 24.0))
                        .on_click(move |ctx, _, _| {
                            ctx.submit_command(EDIT_MODE_CHANGE.with(true).to(editing_widget_id))
                        }),
                    TextBox::new().lens(UiWorkItem::tmp),
                    move |ctx, data: &mut UiWorkItem, _| {
                        data.tags.push_back(data.tmp.to_owned());

                        // Save in backends work item
                        let mut work_item = data.work_item.as_ref().borrow_mut();
                        work_item.push_tag(data.tmp.to_owned());

                        persistence::update_items(vec![&work_item]).unwrap();

                        data.tmp.clear(); // Reset to empty for the next edit

                        ctx.submit_notification(TAG_ADDED);
                    },
                )
                .with_react_on_enter()
                .with_id(editing_widget_id)
                .padding((4.0, 3.0)),
            )),
            on_update: Box::new(on_update),
        }
    }

    fn update_child_count(&mut self, widget_id: WidgetId, data: &UiWorkItem, _env: &Env) -> bool {
        let len = self.children.len();

        match len.cmp(&data.tags.data_len()) {
            Ordering::Greater => self.children.truncate(data.tags.data_len()),
            Ordering::Less => data.tags.for_each(|_, i| {
                if i >= len {
                    let child = WidgetPod::new(build_tag_widget(widget_id));
                    self.children.push(child.boxed());
                }
            }),
            Ordering::Equal => (),
        }
        len != data.tags.data_len()
    }
}

/// Widget displaying a tag
struct TagWidget {
    inner: WidgetPod<String, Box<dyn Widget<String>>>,
    hover: WidgetPod<String, Box<dyn Widget<String>>>,
    is_hovered: bool,
}

impl TagWidget {
    pub fn new(on_delete: impl Fn(&mut EventCtx, &String) + 'static) -> TagWidget {
        let tag_color = rand_color(); // TODO: Use fixed color per tag instead

        let tag_label = Label::new(|text: &String, _: &Env| format!("#{}", text))
            .with_text_color(invert_color(&tag_color))
            .with_text_size(13.0)
            .padding((6.0, 4.0))
            .background(tag_color)
            .rounded(100.0)
            .padding((4.0, 2.0));

        let hover_widget = Flex::row()
            .main_axis_alignment(MainAxisAlignment::Center)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(
                UiButton::new(
                    Align::new(
                        UnitPoint::CENTER,
                        Svg::new(icon::get_icon(icon::DELETE)).fix_height(18.0),
                    )
                    .fix_width(22.0)
                    .fix_height(22.0),
                )
                .with_corner_radius(999.0)
                .with_color(Color::rgb8(204, 51, 51))
                .on_click(move |ctx, data: &mut String, _| {
                    on_delete(ctx, &*data);
                }),
            )
            .background(Color::rgba(0.0, 0.0, 0.0, 0.2))
            .rounded(100.0)
            .padding((4.0, 2.0));

        TagWidget {
            inner: WidgetPod::new(tag_label.boxed()),
            hover: WidgetPod::new(hover_widget.boxed()),
            is_hovered: false,
        }
    }
}

impl Widget<String> for TagWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut String, env: &Env) {
        if ctx.is_hot() != self.is_hovered {
            self.is_hovered = ctx.is_hot();
            ctx.children_changed();
        }

        self.inner.event(ctx, event, data, env);
        self.hover.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &String, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
        self.hover.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &String, data: &String, env: &Env) {
        self.inner.update(ctx, data, env);
        self.hover.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &String,
        env: &Env,
    ) -> Size {
        let size = self.inner.layout(ctx, bc, data, env);

        if self.is_hovered {
            self.hover
                .layout(ctx, &BoxConstraints::tight(size), data, env);
        }

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &String, env: &Env) {
        self.inner.paint(ctx, data, env);
        if self.is_hovered {
            self.hover.paint(ctx, data, env);
        }
    }
}

fn build_tag_widget(widget_id: WidgetId) -> impl Widget<String> {
    TagWidget::new(move |ctx: &mut EventCtx, data: &String| {
        ctx.submit_command(DELETE_TAG.with(data.to_owned()).to(widget_id));
    })
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

impl Widget<UiWorkItem> for TagCloud {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut UiWorkItem, env: &Env) {
        match event {
            Event::Command(cmd) => {
                if cmd.is(DELETE_TAG) {
                    if let Some(tag) = cmd.get(DELETE_TAG) {
                        if let Some(index) = data.tags.index_of(tag) {
                            data.tags.remove(index);
                        }

                        // Remove from backend work item and update it
                        let mut work_item = data.work_item.as_ref().borrow_mut();
                        work_item.pop_tag(tag);

                        persistence::update_items(vec![&work_item]).unwrap();

                        ctx.request_update();

                        (self.on_update)(ctx, &*data);
                    }
                }
            }
            Event::Notification(notification) => {
                if notification.is(TAG_ADDED) {
                    (self.on_update)(ctx, &*data);
                }
            }
            _ => {}
        }

        let mut children = self.children.iter_mut();
        data.tags.for_each_mut(|child_data, _| {
            if let Some(child) = children.next() {
                child.event(ctx, event, child_data, env);
            }
        });
        self.add_widget.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &UiWorkItem,
        env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            if self.update_child_count(ctx.widget_id(), data, env) {
                ctx.children_changed();
            }
        }

        let mut children = self.children.iter_mut();
        data.tags.for_each(|child_data, _| {
            if let Some(child) = children.next() {
                child.lifecycle(ctx, event, child_data, env);
            }
        });
        self.add_widget.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &UiWorkItem,
        data: &UiWorkItem,
        env: &Env,
    ) {
        let mut children = self.children.iter_mut();
        data.tags.for_each(|child_data, _| {
            if let Some(child) = children.next() {
                child.update(ctx, child_data, env);
            }
        });
        self.add_widget.update(ctx, data, env);

        if self.update_child_count(ctx.widget_id(), data, env) {
            ctx.children_changed();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &UiWorkItem,
        env: &Env,
    ) -> Size {
        let max_width = bc.max().width;

        let mut cur_y = 0.0;
        let mut cur_x = 0.0;

        let mut max_line_height = 0.0;

        let mut children = self.children.iter_mut();
        let child_bc = BoxConstraints::new(
            Size::new(0.0, bc.min().height),
            Size::new(f64::INFINITY, bc.max().height),
        );
        data.tags.for_each(|child_data, _| {
            let child = match children.next() {
                Some(child) => child,
                None => {
                    return;
                }
            };

            let child_size = child.layout(ctx, &child_bc, child_data, env);

            // Check if line break is needed
            if cur_x + child_size.width > max_width {
                // Break line
                cur_y += max_line_height;
                cur_x = 0.0;

                max_line_height = 0.0; // Reset maximum line height
            }

            // Adjust maximum line height
            if child_size.height > max_line_height {
                max_line_height = child_size.height;
            }

            // Position child
            child.set_origin(ctx, child_data, env, Point::new(cur_x, cur_y));
            cur_x += child_size.width;
        });

        // Layout add widget
        let add_widget_size = self.add_widget.layout(ctx, &child_bc, data, env);

        // Check if line break is needed
        if cur_x + add_widget_size.width > max_width {
            cur_y += max_line_height;
            cur_x = 0.0;

            max_line_height = 0.0; // Reset maximum line height
        }
        if add_widget_size.height > max_line_height {
            max_line_height = add_widget_size.height;
        }
        self.add_widget
            .set_origin(ctx, data, env, Point::new(cur_x, cur_y));

        Size::new(max_width, cur_y + max_line_height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &UiWorkItem, env: &Env) {
        let mut children = self.children.iter_mut();
        data.tags.for_each(|child_data, _| {
            if let Some(child) = children.next() {
                child.paint(ctx, child_data, env);
            }
        });
        self.add_widget.paint(ctx, data, env);
    }
}
