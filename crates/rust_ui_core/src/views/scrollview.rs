use crate::{PartialInitialisable, layout::RenderObject};
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScrollBehavior {
    /// This means the content is stretched to the axis
    NoScroll,
    /// Content will use its preferred width/height
    Scroll,
}
#[derive(Clone, Copy)]
pub struct Axis {
    pub x: ScrollBehavior,
    pub y: ScrollBehavior,
}

impl Axis {
    pub fn scroll_vertical()->Self{
        Self {
            x: ScrollBehavior::NoScroll,
            y: ScrollBehavior::Scroll,
        }
    }
    pub fn scroll_horizontal()->Self{
        Self {
            x: ScrollBehavior::Scroll,
            y: ScrollBehavior::NoScroll,
        }
    }
}
/// ScrollViews automatically take as much room as needed
/// i.e. they don't have a preferred size
pub struct ScrollView<Child: RenderObject> {
    pub child: Child,
    pub axis: Axis,
    pub(crate) identity: usize,
}

impl<Child: RenderObject> ScrollView<Child> {
    pub fn new(init: ScrollViewPartialInit<Child>) -> Self {
        Self {
            child: init.children.unwrap().0,
            axis: Axis {
                x: init.x.unwrap_or(ScrollBehavior::NoScroll),
                y: init.y.unwrap_or(ScrollBehavior::NoScroll),
            },
            identity: 0,
        }
    }
}

pub struct ScrollViewPartialInit<Child: RenderObject> {
    pub x: Option<ScrollBehavior>,
    pub y: Option<ScrollBehavior>,
    pub children: Option<(Child,)>,
}

impl<C: RenderObject> Default for ScrollViewPartialInit<C> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            children: Default::default(),
        }
    }
}
impl<Child: RenderObject> PartialInitialisable for ScrollView<Child> {
    type PartialInit = ScrollViewPartialInit<Child>;
}
