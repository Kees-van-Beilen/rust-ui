use crate::{
    layout::{ComputableLayout, RenderObject},
    prelude::{FontFamily, FontSize, FontWeight},
};

#[derive(Clone)]
pub enum FontResource {
    Size(FontSize),
    Weight(FontWeight),
    Family(FontFamily),
}

pub struct FontResourceView<Child: RenderObject>(Child, FontResource);
pub struct RenderedFontResourceView<Child: ComputableLayout>(Child, FontResource);

pub trait FontResourceModifier: Sized + RenderObject {
    fn with_font_size(self, size: f64) -> FontResourceView<Self> {
        FontResourceView(self, FontResource::Size(FontSize(size)))
    }

    fn with_font_weight(self, weight: FontWeight) -> FontResourceView<Self> {
        FontResourceView(self, FontResource::Weight(weight))
    }

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
                RenderedFontResourceView(k, self.1.clone())
            }),
            FontResource::Weight(font_weight) => data.ament_with(*font_weight, |adp| {
                RenderedFontResourceView(self.0.render(adp), self.1.clone())
            }),
            FontResource::Family(font_family) => data.ament_with(font_family.clone(), |adp| {
                RenderedFontResourceView(self.0.render(adp), self.1.clone())
            }),
        }
    }
}
impl<T: ComputableLayout> ComputableLayout for RenderedFontResourceView<T> {
    fn preferred_size(
        &self,
        in_frame: &crate::layout::Size<f64>,
    ) -> Option<crate::layout::Size<f64>> {
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
