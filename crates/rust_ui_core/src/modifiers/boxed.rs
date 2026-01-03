use crate::{layout::RenderObject, view::dyn_render::DynGroup};
pub trait BoxedModifier: Sized + RenderObject + Clone + 'static {
    fn boxed(self) -> DynGroup {
        DynGroup::new(self)
    }
}
impl<T: RenderObject + Clone + 'static> BoxedModifier for T {}
