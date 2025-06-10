use crate::layout::{ComputableLayout, RenderObject};
#[derive(Clone, Copy, Debug)]
pub struct Margin {
    top: f64,
    bottom: f64,
    left: f64,
    right: f64,
}
impl Margin {
    pub const fn all(value: f64) -> Self {
        Self {
            top: value,
            bottom: value,
            left: value,
            right: value,
        }
    }
}
pub trait MarginModifier: Sized + RenderObject {
    fn margin(self, margin: Margin) -> MarginView<Self> {
        MarginView(self, margin)
    }
}
impl<T: RenderObject> MarginModifier for T {}

pub struct MarginView<Child: RenderObject>(Child, Margin);
pub struct RenderedMarginView<Child: ComputableLayout>(Child, Margin);

impl<T: RenderObject> RenderObject for MarginView<T> {
    type Output = RenderedMarginView<T::Output>;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        RenderedMarginView(self.0.render(data), self.1)
    }
}
impl<T: ComputableLayout> ComputableLayout for RenderedMarginView<T> {
    fn preferred_size(
        &self,
        in_frame: &crate::layout::Size<f64>,
    ) -> Option<crate::layout::Size<f64>> {
        let mut size = self.0.preferred_size(in_frame)?;
        size.width += self.1.left + self.1.right;
        size.height += self.1.top + self.1.bottom;
        Some(size)
    }
    fn set_size(&mut self, mut to: crate::layout::Size<f64>) {
        to.width -= self.1.left + self.1.right;
        to.height -= self.1.top + self.1.bottom;
        self.0.set_size(to);
    }

    fn set_position(&mut self, mut to: crate::layout::Position<f64>) {
        to.x += self.1.left;
        to.y += self.1.top;
        self.0.set_position(to);
    }

    fn destroy(&mut self) {
        self.0.destroy();
    }
}
