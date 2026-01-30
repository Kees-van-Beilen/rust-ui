#![warn(missing_docs)]
//! Manage the type of view at runtime

use clone_dyn::dependency::clone_dyn_meta;

use crate::{
    layout::{ComputableLayout, Position, RenderObject, Size},
    native::RenderData,
};

/// This wrapper view doesn't care about the inner view.
/// This allows you to decide the final view at runtime.
/// You primarily use this using the [`.boxed() modifier`](crate::modifiers::BoxedModifier).
/// ```rust
/// use rust_ui::prelude::*;
/// 
/// #[ui(main)]
/// pub struct RootView {
///     #[state] page: i32 = 0,
///     body: _ = view!{
///         // Notice the output of the match statement
///         // is either a button view or a text view
///         // therefor they must be wrapped in a box
///         match page {
///             0 => Button("Next page") || {*page.get_mut() = 1}.boxed()
///             1 => Text("this contains text").boxed()
///         }
///     }
/// }
/// ```
pub struct DynGroup(Box<dyn DynRender>);

/// Rendered variant of [`DynGroup`].
/// Allows an arbitrary [`ComputableLayout`] in a static view hierarchy
pub struct DynRendered(Box<dyn ComputableLayout>);


///
/// A more flexible variant of the [`ComputableLayout`] trait.
/// This render method render immediately to an box and is automatically
/// implemented for all [`ComputableLayouts`](`ComputableLayout`)
/// 
#[clone_dyn_meta::clone_dyn]
pub trait DynRender {
    ///
    /// More flexible variant of [`ComputableLayout::render`]
    /// 
    fn render_dyn(&self, data: RenderData) -> Box<dyn ComputableLayout>;
}

impl<T: RenderObject + Clone> DynRender for T
where
    <T as RenderObject>::Output: 'static,
{
    fn render_dyn(&self, data: RenderData) -> Box<dyn ComputableLayout> {
        Box::new(self.render(data))
    }
}

impl DynGroup {
    ///
    /// Construct a [DynGroup] by boxing an view.
    pub fn new<T: RenderObject + Clone + 'static>(obj: T) -> Self {
        DynGroup(Box::new(obj))
    }
    ///
    /// Creates a clone.
    /// The clone is made using [`clone_dyn`]
    pub fn cloned(&self) -> Self {
        let b = clone_dyn::clone(&self.0);
        Self(b)
    }
}


///
/// RustUI view for dynamic views.
/// Sometimes is not possible to manually box/clone views.
/// In that case you may construct [`DynInstance`]. Please note that
/// this instance clones the "template" [`DynGroup`]
pub struct DynInstance;

impl DynInstance {
    /// Instantiate a dynamic instance
    pub fn new(obj: &DynGroup) -> DynGroup {
        obj.cloned()
    }
}

impl ComputableLayout for DynRendered {
    fn preferred_size(&self, in_frame: &Size<f64>) -> Size<Option<f64>> {
        self.0.preferred_size(in_frame)
    }
    fn v_tables(&self) -> &[&dyn ComputableLayout] {
        self.0.v_tables()
    }
    fn v_tables_len(&self) -> usize {
        self.0.v_tables_len()
    }
    fn v_tables_mut(&mut self) -> &mut [&mut dyn ComputableLayout] {
        self.0.v_tables_mut()
    }
    fn set_size(&mut self, to: Size<f64>) {
        self.0.set_size(to);
    }

    fn set_position(&mut self, to: Position<f64>) {
        self.0.set_position(to);
    }

    fn destroy(&mut self) {
        self.0.destroy();
    }
}

impl RenderObject for DynGroup {
    type Output = DynRendered;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        DynRendered(self.0.render_dyn(data))
    }
}
