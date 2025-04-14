use crate::native;

//manage view layout system
#[derive(Clone, Copy)]
#[allow(unused)] // will be used soon™
pub enum Val {
    Px(f64),
    Percent(f64),
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

pub trait ComputableLayout {
    //this must cascade down to the children
    fn set_size(&mut self, to: Size<f64>);
    fn set_position(&mut self, to: Position<f64>);
    //remove this view and its descendants
    fn destroy(&mut self);
}
pub trait RenderObject {
    type Output: ComputableLayout;
    //create a native view
    fn render(&self, data: native::RenderData) -> Self::Output;
}
