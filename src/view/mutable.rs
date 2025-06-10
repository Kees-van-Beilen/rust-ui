use std::{cell::RefCell, rc::Rc};

use crate::layout::RenderObject;

pub trait MutableView {
    fn children(data: Rc<RefCell<Self>>) -> impl RenderObject + 'static;
    fn get_attached(&self) -> &Option<Rc<RefCell<crate::native::MutableView>>>;
    fn get_mut_attached(&mut self) -> &mut Option<Rc<RefCell<crate::native::MutableView>>>;
}
pub trait MutableViewRerender {
    fn rerender(&self);
}
