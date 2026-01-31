//! This module contains the `.background {}` modifier
use crate::layout::{ComputableLayout, RenderObject, Size};

///
/// Background modifier
pub trait BackgroundModifier: Sized + RenderObject {
    ///
    /// Background modifier. This renders the parent view and uses its size and layout
    /// to render a background view below the parent view.
    /// 
    fn background<T: RenderObject>(self, background: T) -> BackgroundView<Self, T> {
        BackgroundView(self, background)
    }
}
impl<T: RenderObject> BackgroundModifier for T {}

/// The actual background view in the view hierarchy
pub struct BackgroundView<Child: RenderObject, Background: RenderObject>(Child, Background);
/// The rendered background view.
pub struct RenderedBackgroundView<Child: ComputableLayout, Background: ComputableLayout>(
    Child,
    Background,
);

impl<T: RenderObject, R: RenderObject> RenderObject for BackgroundView<T, R> {
    type Output = RenderedBackgroundView<T::Output, R::Output>;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        let bg = self.1.render(data.clone());
        RenderedBackgroundView(self.0.render(data), bg)
    }
}
impl<T: ComputableLayout, R: ComputableLayout> ComputableLayout for RenderedBackgroundView<T, R> {
    fn preferred_size(&self, in_frame: &crate::layout::Size<f64>) -> Size<Option<f64>> {
        self.0.preferred_size(in_frame)
    }
    fn set_size(&mut self, to: crate::layout::Size<f64>) {
        self.0.set_size(to);
        self.1.set_size(to);
    }

    fn set_position(&mut self, to: crate::layout::Position<f64>) {
        self.0.set_position(to);
        self.1.set_position(to);
    }

    fn destroy(&mut self) {
        self.0.destroy();
        self.1.destroy();
    }
}
