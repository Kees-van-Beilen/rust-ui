//! Contains the traits needed to layout views
use crate::native;


/// A generic size struct
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Size<T> {
    /// A width
    pub width: T,
    /// A height
    pub height: T,
}

impl<T: Copy> Size<T> {
    /// Construct a size with a width and height both equal to `value`
    pub const fn splat(value: T) -> Self {
        Self {
            width: value,
            height: value,
        }
    }
}

/// A generic position struct
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Position<T> {
    /// The x position
    pub x: T,
    /// The y position
    pub y: T,
}

/// All rendered Rust-ui views implement ComputableLayout
/// This trait manages the layout of a view. That is: 
///  - the min/max/preferred sizes
///  - the layout of the children
///  - the position
///  - the destruction
pub trait ComputableLayout {
    ///this must cascade down to the children
    fn set_size(&mut self, to: Size<f64>);
    ///this must cascade down to the children
    fn set_position(&mut self, to: Position<f64>);
    /// Remove this view and its descendants
    /// This should call the same method on any child views.
    /// views that need to continue focus can opt to not remove itself from the parent view.
    /// instead register a delayed removal
    fn destroy(&mut self);
    ///a layout may contain dynamic elements.
    ///in that case we off course want to iterate over them
    fn v_tables_mut(&mut self) -> &mut [&mut dyn ComputableLayout] {
        //the default is to not do so, as most layouts do not contain dynamic
        &mut []
    }
    ///a layout may contain dynamic elements.
    ///in that case we off course want to iterate over them
    fn v_tables(&self) -> &[&dyn ComputableLayout] {
        //the default is to not do so, as most layouts do not contain dynamic
        &[]
    }
    ///a layout may contain dynamic elements.
    ///in that case we off course want to iterate over them
    fn v_tables_len(&self) -> usize {
        0
    }
    /// Write vtables to a vector
    fn write_v_tables<'a, 'b>(&'a self, _buf: &'b mut Vec<&'a dyn ComputableLayout>) {
        //by default a layout is just one element, so there is nothing dynamic to write
    }
    /// Write vtables to a vector
    fn write_v_tables_mut<'a, 'b>(&'a mut self, _buf: &'b mut Vec<&'a mut dyn ComputableLayout>) {
        //by default a layout is just one element, so there is nothing dynamic to write
    }

    /// Return the preferred size of a view.
    /// This method is implemented by views like Text.
    /// to signal to a layout manager that the view wants a certain (minimum size)
    /// note that this isn't a true minimum size.
    /// A layout manager should:
    ///  1. check the preferred size
    ///  2. if the layout manager wants a smaller size, then check the min_size
    ///  3. if the layout manager needs a bigger size, then check max_size
    ///
    /// The preferred size doesn't have to be bounded by the frame, the `in_frame` is merely a suggestion
    fn preferred_size(&self, _in_frame: &Size<f64>) -> Size<Option<f64>> {
        Size::splat(None)
    }
    /// Currently doesn't play a big role, as not all layouts respect min size.
    /// Defaults to None
    fn min_size(&self, _in_frame: &Size<f64>) -> Size<Option<f64>> {
        Size::splat(None)
    }
    /// Currently doesn't play a big role, as not all layouts respect max size.
    /// Defaults to None
    fn max_size(&self, _in_frame: &Size<f64>) -> Size<Option<f64>> {
        Size::splat(None)
    }
}


/// All rust-ui views implement this trait. 
/// It is the bridge necessary to possibly convert a view 
/// into a native view. 
pub trait RenderObject: Sized {
    /// The Rendered view to output
    type Output: ComputableLayout;
    ///create a native view
    fn render(&self, data: native::RenderData) -> Self::Output;

    /// When the identity of a view is set, it enables persistent views
    fn set_identity(self, _identity: usize) -> Self {
        //by default views are opted out of the identity system
        self
    }
}
