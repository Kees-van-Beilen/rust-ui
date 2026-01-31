//! the `.frame()` modifier
use crate::layout::{ComputableLayout, RenderObject, Size};
#[derive(Clone, Copy, Debug, Default)]

/// A frame layout, it describes the min, max and preferred size.
pub struct Frame {
    /// Min size this frame will allow
    pub min: Size<Option<f64>>,
    /// The max size this frame allows
    pub max: Size<Option<f64>>,
    /// The preferred size, of the frame.
    pub preferred: Size<Option<f64>>,
}

impl Frame {
    /// Construct a new frame with a preferred size.
    pub const fn new(width: f64, height: f64) -> Frame {
        Frame {
            min: Size::splat(None),
            max: Size::splat(None),
            preferred: Size {
                width: Some(width),
                height: Some(height),
            },
        }
    }
    /// Construct a frame with no sizing information.
    /// A frame like that will take up as much size as 
    /// necessary.
    pub const fn no_preference() -> Self {
        Frame {
            min: Size::splat(None),
            max: Size::splat(None),
            preferred: Size::splat(None),
        }
    }
    /// Set the preferred width
    pub const fn width(mut self, value: f64) -> Self {
        self.preferred.width = Some(value);
        self
    }
    /// Set the preferred height
    pub const fn height(mut self, value: f64) -> Self {
        self.preferred.height = Some(value);
        self
    }
}
/// Frame modifier
pub trait FrameModifier: Sized + RenderObject {
    /// Modify a view's preferred frame layout
    /// Note that certain views like text views react differently
    /// towards getting a forced size. One platform may vertically and 
    /// horizontally center the text, whilst an other platform sticks it 
    /// in the top left corner
    fn frame(self, frame: Frame) -> FrameView<Self> {
        FrameView(self, frame)
    }
}
impl<T: RenderObject> FrameModifier for T {}

/// A view with modified frame layout
pub struct FrameView<Child: RenderObject>(Child, Frame);
/// A rendered view with modified frame layout
pub struct RenderedFrameView<Child: ComputableLayout>(Child, Frame);

impl<T: RenderObject> RenderObject for FrameView<T> {
    type Output = RenderedFrameView<T::Output>;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        RenderedFrameView(self.0.render(data), self.1)
    }
}
impl<T: ComputableLayout> ComputableLayout for RenderedFrameView<T> {
    fn preferred_size(&self, _in_frame: &crate::layout::Size<f64>) -> Size<Option<f64>> {
        self.1.preferred
    }
    fn set_size(&mut self, to: crate::layout::Size<f64>) {
        self.0.set_size(to);
    }

    fn max_size(&self, _in_frame: &Size<f64>) -> Size<Option<f64>> {
        self.1.max
    }

    fn min_size(&self, _in_frame: &Size<f64>) -> Size<Option<f64>> {
        self.1.min
    }

    fn set_position(&mut self, to: crate::layout::Position<f64>) {
        self.0.set_position(to);
    }

    fn destroy(&mut self) {
        self.0.destroy();
    }
}
