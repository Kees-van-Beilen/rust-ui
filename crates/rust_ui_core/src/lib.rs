// #![doc(html_logo_url = "https://inpolen.nl/profiles/rust-ui/public/assets/logo-dark.svg")]
// #![doc = include_str!("../readme.md")]
// #![warn(missing_docs)]

pub mod icon;
pub mod layout;
pub mod modifiers;

#[doc(hidden)]
pub mod native;

pub mod view;
pub mod views;

///
/// The rust-ui prelude, contains all the views and modifier one needs.
///
pub mod prelude {
    pub use crate::icon::*;
    pub use crate::layout::{self, Position, Size};
    pub use crate::modifiers::*;
    pub use crate::views::*;
    pub use bevy_color::Color;

    /// Don't use this, this is here temporarily
    #[cfg(target_os = "android")]
    pub const DPI: f64 = 2.0;
    /// Don't use this, this is here temporarily
    #[cfg(not(target_os = "android"))]
    pub const DPI: f64 = 1.0;
}

pub trait PartialInitialisable {
    type PartialInit;
}
