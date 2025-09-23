//! This module includes the traits required to make views that can rerender.
use crate::layout::RenderObject;
use std::{cell::RefCell, rc::Rc};

///
/// This trait is automatically implemented for structures decorated with `#[ui]` or `#[ui(main)]`. It should be implemented for all views that could be rerendered (or trigger the rerender of an other)
///
pub trait MutableView {
    // fn take_state()
    /// Return "child" view of this view. In actuality this is the view that really is being rendered.
    fn children(data: Rc<RefCell<Self>>) -> impl RenderObject + 'static;

    fn set_identity(&mut self, _identity: usize) {}
    fn get_identity(&self) -> usize {
        0
    }

    fn clone_bindings(&self, _into: &mut Self) {}

    /// Used internally.
    #[doc(hidden)]
    fn get_attached(&self) -> &Option<Rc<RefCell<crate::native::MutableView>>>;
    /// Used internally.
    #[doc(hidden)]
    fn get_mut_attached(&mut self) -> &mut Option<Rc<RefCell<crate::native::MutableView>>>;

    #[doc(hidden)]
    #[deprecated]
    fn set_changed(&mut self) {}
    #[doc(hidden)]
    #[deprecated]
    fn read_changed(&mut self) -> bool {
        false
    }
}

///
/// This trait is automatically implemented by your platform backend for all views that implemented [`MutableView`]
///
pub trait MutableViewRerender {
    ///
    /// Rerender the mutable view attached to this
    ///
    fn rerender(&self);
}
