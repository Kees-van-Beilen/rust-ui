use crate::{layout::RenderObject, view::state::PartialBinding};


pub struct SheetModalPresenterView<View:RenderObject,Sheet:RenderObject> {
    pub view:View,
    pub binding:PartialBinding<bool>,
    pub sheet:Option<(
        Box<dyn Fn()->Sheet>,
        usize
    )>,
}
pub trait SheetModifier: Sized + RenderObject {
    fn sheet<Sheet:RenderObject>(self, present_modal: PartialBinding<bool>) -> SheetModalPresenterView<Self,Sheet> {
        SheetModalPresenterView {
            view:self,
            sheet:None,
            binding:present_modal
        }
    }
}

impl<View:RenderObject,Sheet:RenderObject> SheetModalPresenterView<View,Sheet> {
    pub fn with_capture_callback(mut self,sheet_fn:impl Fn()->Sheet+'static,identity:usize)->Self{
        self.sheet = Some((Box::new(sheet_fn),identity));
        self
    }
}
impl<T: RenderObject> SheetModifier for T {}

