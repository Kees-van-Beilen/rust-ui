//! `.on_appear() || {}` modifier
use std::cell::RefCell;
use crate::layout::{ComputableLayout, RenderObject, Size};
/// OnAppear modifier
pub trait OnAppearModifier: Sized + RenderObject {
    /// Cal a function when a view is rendered for the first time.
    /// If a view is removed (garbage collected) and then reappears
    /// this callback will be called again.
    /// 
    /// Please note there are currently some oddities with using this modifier
    /// Especially if using a [`crate::view::task::Task`]
    fn on_appear(self) -> OnAppearView<Self> {
        OnAppearView {
            child: self,
            callback: RefCell::new(Box::new(|| {})),
            identity: None,
        }
    }
}
impl<T: RenderObject> OnAppearModifier for T {}

/// A view with callback
pub struct OnAppearView<Child: RenderObject> {
    child: Child,
    callback: RefCell<Box<dyn Fn()>>,
    identity: Option<usize>,
}
impl<Child: RenderObject> OnAppearView<Child> {
    #[doc(hidden)]
    pub fn with_capture_callback(mut self, func: impl Fn() + 'static, identity: usize) -> Self {
        self.callback = RefCell::new(Box::new(func));
        self.identity = Some(identity);
        self
    }
}
/// A rendered view with callback
pub struct RenderedOnAppearView<Child: ComputableLayout> {
    child: Child,
}

impl<T: RenderObject> RenderObject for OnAppearView<T> {
    type Output = RenderedOnAppearView<T::Output>;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        if let Some(identity) = self.identity {
            let _ = data
                .persistent_storage
                .borrow_mut()
                .get_or_init_with(identity, || {
                    (self.callback.borrow())();
                    true;
                });
        }
        RenderedOnAppearView {
            child: self.child.render(data),
        }
    }
}
impl<T: ComputableLayout> ComputableLayout for RenderedOnAppearView<T> {
    fn preferred_size(&self, in_frame: &crate::layout::Size<f64>) -> Size<Option<f64>> {
        self.child.preferred_size(in_frame)
    }
    fn set_size(&mut self, to: crate::layout::Size<f64>) {
        self.child.set_size(to);
    }

    fn set_position(&mut self, to: crate::layout::Position<f64>) {
        self.child.set_position(to);
    }

    fn destroy(&mut self) {
        self.child.destroy();
    }
}
