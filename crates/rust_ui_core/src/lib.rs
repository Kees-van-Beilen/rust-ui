// #![doc(html_logo_url = "https://inpolen.nl/profiles/rust-ui/public/assets/logo-dark.svg")]
// #![doc = include_str!("../readme.md")]

pub mod icon;
pub mod layout;
pub mod modifiers;
#[doc(hidden)]
pub mod native;
pub mod view;
pub mod views;

pub mod prelude {
    pub use crate::icon::*;
    pub use crate::layout::{self, Position, Size};
    pub use crate::views::*;
    pub use crate::modifiers::*;
    pub use bevy_color::Color;
}


pub trait PartialInitialisable {
    type PartialInit;
}

