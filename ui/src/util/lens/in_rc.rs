use druid::{Data, Lens};
use std::rc::Rc;

#[derive(Debug, Copy, Clone)]
pub struct InRc<L> {
    inner: L,
}

impl<L> InRc<L> {
    pub fn new<A, B>(inner: L) -> Self
    where
        A: Clone,
        B: Data,
        L: Lens<A, B>,
    {
        Self { inner }
    }
}

impl<A, B, L> Lens<Rc<A>, B> for InRc<L>
where
    A: Clone,
    B: Data,
    L: Lens<A, B>,
{
    fn with<V, F: FnOnce(&B) -> V>(&self, data: &Rc<A>, f: F) -> V {
        self.inner.with(data, f)
    }

    fn with_mut<V, F: FnOnce(&mut B) -> V>(&self, data: &mut Rc<A>, f: F) -> V {
        let mut temp = self.inner.with(data, |x| x.clone());
        let v = f(&mut temp);
        if self.inner.with(data, |x| !x.same(&temp)) {
            self.inner.with_mut(Rc::make_mut(data), |x| *x = temp);
        }
        v
    }
}
