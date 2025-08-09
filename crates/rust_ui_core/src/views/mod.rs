pub mod button;
pub mod layout;
pub mod tabbar;
pub mod text;
pub mod image;
pub mod scrollview;

//this becomes a native view
pub struct ColorView(pub bevy_color::Color);
impl ColorView {
    pub fn new(color:bevy_color::Color)->Self {
        Self(color)
    }
}

pub use button::*;
pub use layout::*;
pub use tabbar::*;
pub use text::*;
pub use image::*;
pub use scrollview::*;
