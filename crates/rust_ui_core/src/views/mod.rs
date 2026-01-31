#![warn(missing_docs)]
//!
//! Views are the core building blocks in rust-ui. This module offers a number of useful views. Some of these views like text or button views
//! translate directly to native views. Others are considered layout-views, those views merely position and size their child views.
//!
//!

pub mod button;
pub mod control_flows;
pub mod image;
pub mod layout;
pub mod scrollview;
pub mod tabbar;
pub mod text;
pub mod textfield;

/// A simple view that displays a given color.
/// This view has no preferred size. 
/// So you may have to use a 
/// [`.frame()`](`crate::modifiers::frame::FrameModifier::frame`)
pub struct ColorView(pub bevy_color::Color);
impl ColorView {
    /// construct a Color view using a given color.
    pub fn new(color: bevy_color::Color) -> Self {
        Self(color)
    }
}

pub use button::*;
pub use image::*;
pub use layout::*;
pub use scrollview::*;
pub use tabbar::*;
pub use text::*;
pub use textfield::TextField;
