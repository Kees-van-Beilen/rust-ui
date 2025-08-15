use crate::{layout::{ComputableLayout, RenderObject, Size}, view::dyn_render::DynGroup};
pub trait BoxedModifier: Sized + RenderObject + 'static{
    fn boxed(self) -> DynGroup {
        DynGroup::new(self)
    }
}
impl<T: RenderObject+'static> BoxedModifier for T {}

