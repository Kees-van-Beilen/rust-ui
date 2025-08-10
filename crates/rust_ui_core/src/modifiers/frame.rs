use crate::layout::{ComputableLayout, RenderObject, Size};
#[derive(Clone, Copy, Debug,Default)]
pub struct Frame {
    min:Size<Option<f64>>,
    max:Size<Option<f64>>,
    preferred:Size<Option<f64>>
}

impl Frame {
    pub fn new(width:f64,height:f64)->Frame {
        Frame { min: Size::splat(None), max: Size::splat(None), preferred: Size { width: Some(width), height: Some(height) } }
    }

    pub fn no_preference()->Self{
        Frame {min: Size::splat(None),max:Size::splat(None),preferred:Size::splat(None)}
    }

    pub fn width(mut self,value:f64)->Self{
        self.preferred.width = Some(value);
        self
    }

    pub fn height(mut self,value:f64)->Self{
        self.preferred.height = Some(value);
        self
    }
}



pub trait FrameModifier: Sized + RenderObject {
    fn frame(self, frame: Frame) -> FrameView<Self> {
        FrameView(self, frame)
    }
}
impl<T: RenderObject> FrameModifier for T {}

pub struct FrameView<Child: RenderObject>(Child, Frame);
pub struct RenderedFrameView<Child: ComputableLayout>(Child, Frame);

impl<T: RenderObject> RenderObject for FrameView<T> {
    type Output = RenderedFrameView<T::Output>;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        RenderedFrameView(self.0.render(data), self.1)
    }
}
impl<T: ComputableLayout> ComputableLayout for RenderedFrameView<T> {
    fn preferred_size(
        &self,
        in_frame: &crate::layout::Size<f64>,
    ) -> Size<Option<f64>> {
        self.1.preferred
    }
    fn set_size(&mut self, mut to: crate::layout::Size<f64>) {
        // to.width -= self.1.left + self.1.right;
        // to.height -= self.1.top + self.1.bottom;
        self.0.set_size(to);
    }

    fn set_position(&mut self, mut to: crate::layout::Position<f64>) {
        // to.x += self.1.left;
        // to.y += self.1.top;
        self.0.set_position(to);
    }

    fn destroy(&mut self) {
        self.0.destroy();
    }
}
