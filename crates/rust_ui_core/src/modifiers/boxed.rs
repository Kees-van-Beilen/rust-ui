//! this module contains the `.boxed()` modifier
use crate::{layout::RenderObject, view::dyn_render::DynGroup};

/// The boxed modifier
pub trait BoxedModifier: Sized + RenderObject + Clone + 'static {
    /// The boxed modifier. This can be very useful when implementing 
    /// switch statements. As the outcome of a switch statement has to be the same for 
    /// every arm, boxing different view types into the same [`DynGroup`] is a great way to 
    /// ensure this.
    fn boxed(self) -> DynGroup {
        DynGroup::new(self)
    }
}
impl<T: RenderObject + Clone + 'static> BoxedModifier for T {}
