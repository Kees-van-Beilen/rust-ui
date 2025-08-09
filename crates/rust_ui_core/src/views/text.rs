use std::rc::Rc;

use crate::{impl_resource, view::resources::Resource};

/// Supports font weights 100-900, not every system treats font weights the same
/// thats why this is an enum instead of a number
#[derive(Clone, Copy, Debug, Default)]
pub enum FontWeight {
    Ultralight,
    Thin,
    Light,
    #[default]
    Regular,
    Medium,
    Semibold,
    Bold,
    Heavy,
    Black,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum TextAlignment {
    /// In most cases this means to the left
    Leading,
    #[default]
    Center,
    ///In most cases this means to the right
    Trailing
}

#[derive(Default, Clone, Debug)]
pub enum FontFamily {
    ///Use the default system font
    #[default]
    SystemUI,
    /// Reference a custom font family by name
    /// This does require the font to be available as a system font
    /// or be properly bundled with the application
    Custom(Rc<str>),
}


#[derive(Clone, Copy, Debug)]
pub struct FontSize(pub f64);


impl_resource!(FontSize);
impl_resource!(FontWeight);
impl_resource!(FontFamily);
impl_resource!(TextAlignment);


pub struct Text {
    pub content: String,
}

impl Text {
    pub fn new(str:impl ToString)->Text{
        Text {
            content:str.to_string()
        }
    }
}