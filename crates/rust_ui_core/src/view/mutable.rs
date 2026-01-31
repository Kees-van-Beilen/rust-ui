#![warn(missing_docs)]
//! This module includes the traits required to make views that can rerender.
use crate::layout::RenderObject;
use std::{cell::RefCell, rc::Rc};

///
/// This trait is automatically implemented for structures decorated with `#[ui]` or `#[ui(main)]`. It should be implemented for all views that could be rerendered (or trigger the rerender of an other)
///
pub trait MutableView {
    /// Return "child" view of this view. In actuality this is the view that really is being rendered.
    fn children(data: Rc<RefCell<Self>>) -> impl RenderObject + 'static;
    /// set the identity of the current view. Defaults to a nop.
    fn set_identity(&mut self, _identity: usize) {}
    /// get the identity of a view.
    fn get_identity(&self) -> usize {
        0
    }
    /// Clone this views bindings into a different instance of this view.
    /// This is used when views are refreshed in ui updates to make sure all
    /// bindings remain valid.
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
