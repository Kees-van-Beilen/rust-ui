//! wrapper around the native scroll views.
use crate::{PartialInitialisable, layout::RenderObject};

/// Scroll behavior dictates if a axis should scroll or not,
/// In the future more kinds of scroll behaviors may be added.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScrollBehavior {
    /// This means the content is stretched to the axis
    NoScroll,
    /// Content will use its preferred width/height
    Scroll,
}

/// Scroll behavior on the x and y axis.
#[derive(Clone, Copy)]
pub struct Axis {
    /// x axis scroll behavior
    pub x: ScrollBehavior,
    /// y axis scroll behavior
    pub y: ScrollBehavior,
}

impl Axis {
    /// Scroll vertically
    pub fn scroll_vertical() -> Self {
        Self {
            x: ScrollBehavior::NoScroll,
            y: ScrollBehavior::Scroll,
        }
    }
    /// Scroll horizontally
    pub fn scroll_horizontal() -> Self {
        Self {
            x: ScrollBehavior::Scroll,
            y: ScrollBehavior::NoScroll,
        }
    }
}
/// ScrollViews automatically take as much room as needed
/// i.e. they don't have a preferred size
/// 
/// Please note that android doesn't support nested scroll views.
/// Also currently android doesn't support scrolling vertically and horizontally
/// at the same time.
pub struct ScrollView<Child: RenderObject> {
    /// the scrolled "document"
    pub child: Child,
    /// the axis to scroll on
    pub axis: Axis,
    pub(crate) identity: usize,
}

impl<Child: RenderObject> ScrollView<Child> {
    /// Construct a new scrollview
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

/// The partial initializer for the [`ScrollView`]
pub struct ScrollViewPartialInit<Child: RenderObject> {
    /// Scroll behavior of the x axis, defaults to [`ScrollBehavior::NoScroll`]
    pub x: Option<ScrollBehavior>,
    /// Scroll behavior of the y axis, defaults to [`ScrollBehavior::NoScroll`]
    pub y: Option<ScrollBehavior>,
    /// The "document" to scroll
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
