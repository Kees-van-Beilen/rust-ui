use crate::layout::RenderObject;
#[derive(Clone,Copy,PartialEq, Eq)]
pub enum ScrollBehavior {
    /// This means the content is stretched to the axis
    NoScroll,
    /// Content will use its preferred width/height
    Scroll
}
#[derive(Clone,Copy)]
pub struct Axis {
    pub x:ScrollBehavior,
    pub y:ScrollBehavior
}
/// ScrollViews automatically take as much room as needed
/// i.e. they don't have a preferred size
pub struct ScrollView<Child:RenderObject> {
    pub child:Child,
    pub axis: Axis,
}