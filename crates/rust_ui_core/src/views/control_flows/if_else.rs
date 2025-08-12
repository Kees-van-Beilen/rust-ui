//! This module provides a view that the rust-ui macro language can use to translate if-else blocks in ui code
use crate::layout::{ComputableLayout, RenderObject};


/// Used internally, this view represents an if-else block
pub enum EitherView<A:RenderObject,B:RenderObject> {
    ViewA(A),
    ViewB(B)
}
/// Used internally, this view represents a rendered if-else block
pub enum RenderEitherView<A:ComputableLayout,B:ComputableLayout> {
    ViewA(A),
    ViewB(B)
}

impl<A:RenderObject,B:RenderObject> RenderObject for EitherView<A,B> {
    type Output = RenderEitherView<A::Output,B::Output>;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        match self {
            EitherView::ViewA(a) => RenderEitherView::ViewA(a.render(data)),
            EitherView::ViewB(b) => RenderEitherView::ViewB(b.render(data)),
        }
    }
}
impl<A:ComputableLayout,B:ComputableLayout> ComputableLayout for RenderEitherView<A,B> {
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        match self {
            RenderEitherView::ViewA(a) => a.set_size(to),
            RenderEitherView::ViewB(b) => b.set_size(to),
        }
    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        match self {
            RenderEitherView::ViewA(a) => a.set_position(to),
            RenderEitherView::ViewB(b) => b.set_position(to),
        }
    }

    fn destroy(&mut self) {
        match self {
            RenderEitherView::ViewA(a) => a.destroy(),
            RenderEitherView::ViewB(b) => b.destroy(),
        }
    }
}