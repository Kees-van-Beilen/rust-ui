use std::rc::Rc;

use crate::view::resources::Resource;

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

// pub struct Font {
//     pub weight:Option<FontWeight>,
//     pub size:Option<f64>,
//     pub family:Option<FontFamily>
// }
#[derive(Clone, Copy, Debug)]
pub struct FontSize(pub f64);

impl Resource for FontSize {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Resource for FontWeight {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Resource for FontFamily {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct Text {
    pub content: String,
    // pub font:Option<Font>
}
