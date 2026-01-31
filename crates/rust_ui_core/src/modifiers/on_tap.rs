//! `.on_tap || {}` modifier
use std::cell::RefCell;
use crate::layout::RenderObject;

/// OnTap Modifier
pub trait OnTapModifier: Sized + RenderObject {
    /// Run a callback on tap/click of a view.
    #[doc(alias = "on_click")]
    fn on_tap(self, func: impl Fn() + 'static) -> OnTapView<Self> {
        OnTapView(self, RefCell::new(Box::new(func)))
    }
}

impl<C: RenderObject> OnTapView<C> {
    #[doc(hidden)]
    pub fn with_capture_callback(mut self, func: impl Fn() + 'static, _identity: usize) -> Self {
        self.1 = RefCell::new(Box::new(func));
        self
    }
}
impl<T: RenderObject> OnTapModifier for T {}

/// A view with a on tap callback. The rendered variant is a native implementation
pub struct OnTapView<Child: RenderObject>(pub(crate) Child, pub(crate) RefCell<Box<dyn Fn()>>);
