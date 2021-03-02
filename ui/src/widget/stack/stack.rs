use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx, Widget, WidgetExt, WidgetPod,
};

/// Widget used to stack widgets over each other.
pub(crate) struct Stack<T> {
    children: Vec<StackChild<T>>,
}

impl<T> Stack<T>
where
    T: Data,
{
    /// Create a stack instance.
    pub fn new() -> Stack<T> {
        Stack {
            children: Vec::new(),
        }
    }

    /// Add a child to the stack.
    pub fn with_child(mut self, child: impl Widget<T> + 'static) -> Self {
        self.children.push(StackChild::new(child));

        self
    }
}

impl<T> Widget<T> for Stack<T>
where
    T: Data,
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        for child in self.children.iter_mut().rev() {
            child.event(ctx, event, data, env)
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        for child in self.children.iter_mut() {
            child.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        for child in self.children.iter_mut() {
            child.update(ctx, old_data, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        // A stacks size is determined by the largest child (stacked over another).
        // So we will layout each child individually and determine the largest size.

        let mut largest_size = Size::ZERO;

        for child in self.children.iter_mut() {
            let size = child.layout(ctx, bc, data, env);

            largest_size.width = largest_size.width.max(size.width);
            largest_size.height = largest_size.height.max(size.height);
        }

        largest_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        for child in self.children.iter_mut() {
            child.paint(ctx, data, env);
        }
    }
}

/// Child widget of a stack.
struct StackChild<T> {
    /// Whether the child will let mouse events through.
    pod: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T> StackChild<T>
where
    T: Data,
{
    pub fn new(inner: impl Widget<T> + 'static) -> StackChild<T> {
        StackChild {
            pod: WidgetPod::new(inner.boxed()),
        }
    }
}

impl<T> Widget<T> for StackChild<T>
where
    T: Data,
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.pod.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.pod.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.pod.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.pod.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.pod.paint(ctx, data, env);
    }
}
