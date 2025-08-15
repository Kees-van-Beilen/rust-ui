use crate::{layout::RenderObject, view::state::PartialBinding};


// pub struct SheetModalPresenterView<View:RenderObject,Sheet:RenderObject> {
//     pub view:View,
//     pub sheet:Option<Box<dyn Fn()->Sheet>>,
// }
// pub trait FrameModifier: Sized + RenderObject {
//     fn sheet(self, present_modal: PartialBinding<bool>) -> SheetModalPresenterView<Self,_> {
//         SheetModalPresenterView {
//             view:self,
//             sheet:None
//         }
//     }
// }

// impl<View:RenderObject,Sheet:RenderObject> SheetModalPresenterView<View,Sheet> {
//     fn with_capture_callback(mut self,sheet_fn:impl Fn()->Sheet+'static)->Self{
//         self.sheet = Some(Box::new(sheet_fn));
//         self
//     }
// }
// impl<T: RenderObject> FrameModifier for T {}

