use crate::native;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

pub trait ComputableLayout {
    ///this must cascade down to the children
    fn set_size(&mut self, to: Size<f64>);
    ///this must cascade down to the children
    fn set_position(&mut self, to: Position<f64>);
    ///remove this view and its descendants
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

    /// Return the preferred size of a view.
    /// This method is implemented by views like Text.
    /// to signal to a layout manager that the view wants a certain (minimum size)
    /// note that this isn't a true minimum size.
    /// A layout manager should:
    ///  1. check the preferred size
    ///  2. if the layout manager wants a smaller size, then check the min_size
    ///  3. if the layout manager needs a bigger size, then check max_size
    fn preferred_size(&self, _in_frame: &Size<f64>) -> Option<Size<f64>> {
        None
    }

    fn min_size(&self, _in_frame: &Size<f64>) -> Option<Size<f64>> {
        None
    }
    fn max_size(&self, _in_frame: &Size<f64>) -> Option<Size<f64>> {
        None
    }
}
pub trait RenderObject {
    type Output: ComputableLayout;
    ///create a native view
    fn render(&self, data: native::RenderData) -> Self::Output;
}
