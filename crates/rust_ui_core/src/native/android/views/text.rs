use crate::layout::{ComputableLayout, RenderObject};

impl RenderObject for crate::views::Text {
    type Output=NativeTextView;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        todo!()
    }
}

pub struct NativeTextView {

}

impl ComputableLayout for NativeTextView {
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        todo!()
    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        todo!()
    }

    fn destroy(&mut self) {
        todo!()
    }
}