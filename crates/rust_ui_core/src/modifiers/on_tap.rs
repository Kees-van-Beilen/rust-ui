use std::cell::RefCell;

use crate::layout::{ComputableLayout, RenderObject, Size};


pub trait OnTapModifier: Sized + RenderObject {
   fn on_tap(self,func:impl Fn()+'static)->OnTapView<Self>{
        OnTapView(self,RefCell::new(Box::new(func)))
   }
}
impl<T: RenderObject> OnTapModifier for T {}

pub struct OnTapView<Child: RenderObject>(pub(crate) Child, pub(crate)  RefCell<Box<dyn Fn()>>,);
// pub struct RenderedOnTap<Child: ComputableLayout>(Child);

// impl<T: RenderObject> RenderObject for OnTapView<T> {
//     type Output = RenderedMarginView<T::Output>;

//     fn render(&self, data: crate::native::RenderData) -> Self::Output {
//         RenderedMarginView(self.0.render(data), self.1)
//     }
// }
// impl<T: ComputableLayout> ComputableLayout for RenderedMarginView<T> {
//     fn preferred_size(
//         &self,
//         in_frame: &crate::layout::Size<f64>,
//     ) -> Size<Option<f64>> {
//         let mut size = self.0.preferred_size(in_frame);
//         if let Some(width) = &mut size.width {
//             *width += self.1.left + self.1.right;
//         }
//         if let Some(height) = &mut size.height {
//             *height +=  self.1.top + self.1.bottom;
//         }
//         size
//     }
//     fn set_size(&mut self, mut to: crate::layout::Size<f64>) {
//         to.width -= self.1.left + self.1.right;
//         to.height -= self.1.top + self.1.bottom;
//         self.0.set_size(to);
//     }

//     fn set_position(&mut self, mut to: crate::layout::Position<f64>) {
//         to.x += self.1.left;
//         to.y += self.1.top;
//         self.0.set_position(to);
//     }

//     fn destroy(&mut self) {
//         self.0.destroy();
//     }
// }
