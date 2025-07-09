use crate::layout::{ComputableLayout, Position, RenderObject, Size};


/*
When used in a layout, takes all the remaining space.
For example:

HStack {
    Spacer,
    Text("hello")
}

puts the text hello to the right
*/
pub struct Spacer;
impl RenderObject for Spacer {
    type Output = Spacer;

    fn render(&self, _: crate::native::RenderData) -> Self::Output {
        Spacer
    }
}
impl ComputableLayout for Spacer {
    fn set_size(&mut self, _: Size<f64>) {}

    fn set_position(&mut self, _: Position<f64>) {}

    fn destroy(&mut self) {}
}

impl Spacer {
    pub fn new(_:impl Into<()>)->Self{
        Self
    }
}