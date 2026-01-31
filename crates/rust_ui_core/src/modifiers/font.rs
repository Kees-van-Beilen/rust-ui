//! A collection of text modifiers like `.foreground_color()`
//! `.with_font_size()`
//! `.with_font_weight()`
//! `.align()`
use crate::{
    layout::{ComputableLayout, RenderObject, Size},
    prelude::{FontFamily, FontSize, FontWeight},
    views::{ForegroundColor, TextAlignment},
};

/// A shorthand wrapper around different font resource types.
/// This is used instead of creating a different view for 
/// for every resource.
#[derive(Clone)]
// allow unused because font family is not yet implemented.
#[allow(unused)]
enum FontResource {
    Size(FontSize),
    Weight(FontWeight),
    Family(FontFamily),
    Align(TextAlignment),
    Color(ForegroundColor),
}
/// A view to modify the child's  resources
pub struct FontResourceView<Child: RenderObject>(Child, FontResource);
/// rendered version of [`FontResourceView`]
pub struct RenderedFontResourceView<Child: ComputableLayout>(Child);

/// Collection of font resource modifiers.
pub trait FontResourceModifier: Sized + RenderObject {
    /// Modify the foreground color, this will for instance change the text color
    /// and for monochrome icons it also changes the color.
    fn foreground_color(self, color: bevy_color::Color) -> FontResourceView<Self> {
        FontResourceView(self, FontResource::Color(ForegroundColor(color)))
    }
    /// Modify the font size, font size is in pixels (for now)
    fn with_font_size(self, size: f64) -> FontResourceView<Self> {
        FontResourceView(self, FontResource::Size(FontSize(size)))
    }
    /// Modify the font weight, this may not work on android, as you need to bundle 
    /// the appropriate font weights.
    fn with_font_weight(self, weight: FontWeight) -> FontResourceView<Self> {
        FontResourceView(self, FontResource::Weight(weight))
    }
    /// Set the text alignment
    fn align(self, alignment: TextAlignment) -> FontResourceView<Self> {
        FontResourceView(self, FontResource::Align(alignment))
    }
    /// Set the font weight to bold and the size to be bigger 
    /// then a normal body text.
    fn title(self) -> FontResourceView<FontResourceView<Self>> {
        self.with_font_size(21.0).with_font_weight(FontWeight::Bold)
    }
}
impl<T: RenderObject> FontResourceModifier for T {}

impl<T: RenderObject> RenderObject for FontResourceView<T> {
    type Output = RenderedFontResourceView<T::Output>;

    fn render(&self, mut data: crate::native::RenderData) -> Self::Output {
        match &self.1 {
            FontResource::Size(font_size) => data.ament_with(*font_size, |adp| {
                let k = self.0.render(adp);
                RenderedFontResourceView(k)
            }),
            FontResource::Weight(font_weight) => data.ament_with(*font_weight, |adp| {
                RenderedFontResourceView(self.0.render(adp))
            }),
            FontResource::Family(font_family) => data.ament_with(font_family.clone(), |adp| {
                RenderedFontResourceView(self.0.render(adp))
            }),
            FontResource::Align(text_alignment) => data.ament_with(text_alignment.clone(), |adp| {
                RenderedFontResourceView(self.0.render(adp))
            }),
            FontResource::Color(foreground_color) => data
                .ament_with(foreground_color.clone(), |adp| {
                    RenderedFontResourceView(self.0.render(adp))
                }),
        }
    }
}
impl<T: ComputableLayout> ComputableLayout for RenderedFontResourceView<T> {
    fn preferred_size(&self, in_frame: &crate::layout::Size<f64>) -> Size<Option<f64>> {
        self.0.preferred_size(in_frame)
    }
    fn set_size(&mut self, to: crate::layout::Size<f64>) {
        self.0.set_size(to)
    }

    fn set_position(&mut self, to: crate::layout::Position<f64>) {
        self.0.set_position(to)
    }

    fn destroy(&mut self) {
        self.0.destroy();
    }
}
