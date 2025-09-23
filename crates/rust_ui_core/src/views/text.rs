use std::rc::Rc;

use crate::{
    impl_resource,
    layout::RenderObject,
    view::{
        persistent_storage::PersistentStorageRef,
        resources::{Resource, ResourceStack},
    },
};

/// Supports font weights 100-900, not every system treats font weights the same
/// thats why this is an enum instead of a number.
/// 
/// In the future this enum will change to a struct (to accommodate variable weights)
#[derive(Clone, Copy, Debug, Default,PartialEq)]
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

/// The alignment of text elements
#[derive(Clone, Copy, Debug, Default,PartialEq,Eq)]
pub enum TextAlignment {
    /// In most cases this means to the left
    Leading,
    /// Align text to the center
    #[default]
    Center,
    ///In most cases this means to the right
    Trailing,
}

/// The font family to use for text elements.
#[derive(Default, Clone, Debug,PartialEq)]
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
    #[doc(hidden)]
    Custom(Rc<str>),
}

/// The font size to use for text elements.
#[derive(Clone, Copy, Debug,PartialEq)]
pub struct FontSize(pub f64);

/// The color of elements that can be tinted.
#[derive(Clone,Copy,PartialEq)]
pub struct TintColor(bevy_color::Color);

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


#[derive(Debug)]
pub struct RenderDataDebug<'a> {
    pub stack: ResourceStack<'a>,
    pub persistent_storage: PersistentStorageRef,
}
///
/// Debug text allows you to introspect the current render data. This includes the resource stack and the persistent storage container.
/// 
pub struct DebugText {
    dbg_fn: Box<dyn Fn(RenderDataDebug) -> String>,
}

impl DebugText {
    ///
    /// construct a new empty DebugText element.
    pub fn new(_: ()) -> Self {
        Self {
            dbg_fn: Box::new(|_| String::default()),
        }
    }

    ///
    /// Used internally. Set the introspection function.
    pub fn with_capture_callback(
        mut self,
        callback: impl Fn(RenderDataDebug) -> String + 'static,
        _identity: usize,
    ) -> Self {
        self.dbg_fn = Box::new(callback);
        self
    }
}


impl RenderObject for DebugText {
    type Output = <Text as RenderObject>::Output;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        let out = (self.dbg_fn)(RenderDataDebug {
            stack: data.stack.clone(),
            persistent_storage: data.persistent_storage.clone(),
        });
        Text::new(out).render(data)
    }
}
