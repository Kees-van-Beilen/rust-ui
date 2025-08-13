use std::cell::RefCell;

use crate::layout::RenderObject;

pub trait OnTapModifier: Sized + RenderObject {
    fn on_tap(self, func: impl Fn() + 'static) -> OnTapView<Self> {
        OnTapView(self, RefCell::new(Box::new(func)))
    }
}

impl<C: RenderObject> OnTapView<C> {
    pub fn with_capture_callback(mut self, func: impl Fn() + 'static) -> Self {
        self.1 = RefCell::new(Box::new(func));
        self
    }
}
impl<T: RenderObject> OnTapModifier for T {}

pub struct OnTapView<Child: RenderObject>(pub(crate) Child, pub(crate) RefCell<Box<dyn Fn()>>);