// use crate::{layout::{ComputableLayout, RenderObject}, prelude::sheet::SheetModalPresenterView, view::state::PartialBinding};

// use crate::{layout::{ComputableLayout, RenderObject}, prelude::sheet::SheetModalPresenterView, view::state::PartialBinding};


// pub struct NativeModalPresenter<View:ComputableLayout,Sheet:RenderObject> {
//     binding:PartialBinding<bool>,
//     view:View,
//     sheet:Box<dyn Fn()->Sheet>
// }
// impl<Sheet:RenderObject,View:RenderObject> RenderObject for SheetModalPresenterView<View,Sheet> {
//     type Output=NativeModalPresenter<View::Output,Sheet>;

//     fn render(&self, data: crate::native::RenderData) -> Self::Output {
//         // // let identity =  self.sheet.unwrap()
//         // data.persistent_storage.borrow().get(identity)

//         // NativeModalPresenter {
//         //     binding: self,
//         //     child: todo!(),
//         // }
//         todo!()

//     }
// }

// impl <View:ComputableLayout,Sheet:RenderObject> ComputableLayout for NativeModalPresenter<View,Sheet> {
//     fn set_size(&mut self, to: crate::prelude::Size<f64>) {
//         todo!()
//     }

//     fn set_position(&mut self, to: crate::prelude::Position<f64>) {
//         todo!()
//     }

//     fn destroy(&mut self) {
//         todo!()
//     }
// }