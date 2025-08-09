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
    /// Align text to the center
    #[default]
    Center,
    ///In most cases this means to the right
    Trailing,
}

#[derive(Default, Clone, Debug)]
pub enum FontFamily {
    /// Use the default system bundled font.
    /// (This may not be the same as the default system font)
    #[default]
    SystemUI,
    /// Reference a custom font family by name
    /// This does require the font to be available as a system font
    /// or be properly bundled with the application.
    /// 
    /// This currently does nothing, as custom fonts aren't yet implemented
    Custom(Rc<str>),
}

#[derive(Clone, Copy, Debug)]
pub struct FontSize(pub f64);

impl_resource!(FontSize);
impl_resource!(FontWeight);
impl_resource!(FontFamily);
impl_resource!(TextAlignment);

///
/// This is a wrapper around a native text view. You may use modifiers to change text properties such as FontWeight or FontSize.
///
/// ## Example 1
/// <table style="width:100%">
/// <tr>
/// <td style="border:none">
///
///
/// ```rust
/// use rust_ui::prelude::*;
///
/// #[ui(main)]
/// struct RootView {
///     body:_ = view!{
///        Text("Hello world")
///     }
/// }
/// ```
///
/// </td>
/// <td style="border:none;width:400px"> <img width="400" src="https://inpolen.nl/profiles/rust-ui/public/example_images/text_1.png"> </td>
/// </tr>
/// </table>
///
/// # Example 2
/// <table style="width=100%">
/// <tr>
/// <td style="border:none">
///
///
/// ```rust
/// use rust_ui::prelude::*;
///
/// #[ui(main)]
/// struct RootView {
///     body:_ = view!{
///        VStack {
///            HStack {
///                Text("A title here").title()
///                Spacer()
///            }
///            HStack {
///                Text("All these children are small")
///                Text("Except").with_font_size(18.0)
///                Text("That one")
///           }
///        }
///     }
/// }
/// ```
///
/// </td>
/// <td style="border:none;width:400px" > <img width="400" src="https://inpolen.nl/profiles/rust-ui/public/example_images/text_2.png"> </td>
/// </tr>
/// </table>
///
#[doc(alias = "TextView")]
pub struct Text {
    pub content: String,
}

impl Text {
    ///
    /// Create a new Text view using the specified string.
    /// 
    pub fn new(str: impl ToString) -> Text {
        Text {
            content: str.to_string(),
        }
    }
}
