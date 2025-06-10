pub mod button;
pub mod layout;
pub mod tabbar;
pub mod text;

//this becomes a native view
pub struct ColorView(pub bevy_color::Color);

pub use button::*;
pub use layout::*;
pub use tabbar::*;
pub use text::*;
