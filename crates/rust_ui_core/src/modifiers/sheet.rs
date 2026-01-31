//! `.sheet() {}` modifier
use crate::{layout::RenderObject, view::state::PartialBinding};

/// A view with a modal presenter attached.
pub struct SheetModalPresenterView<View: RenderObject, Sheet: RenderObject> {
    /// the child view
    pub view: View,
    /// A boolean binding indicating if the modal should be presented
    pub binding: PartialBinding<bool>,
    /// The modal sheet, constructed lazily
    pub sheet: Option<(Box<dyn Fn() -> Sheet>, usize)>,
}

/// The sheet modifier
pub trait SheetModifier: Sized + RenderObject {
    /// Present a sheet. 
    /// 
    /// Sheets have access to the host view's variables.
    /// 
    /// Currently on macos the sheet cannot be dismissed using a user
    /// gesture, this will change in the future.
    /// 
    /// 
    /// ```rust
    /// use rust_ui::prelude::*;
    /// 
    /// #[ui(main)]
    /// pub struct RootView {
    ///     #[state] show_sheet: bool = false,
    ///     body: _ = view! {
    ///         Button("show sheet") || {
    ///             *show_sheet.get_mut() = true
    ///         }
    ///            .sheet(bind!(show_sheet)) {
    ///                 Text("Hello world, this is a sheet")
    ///            }
    ///     }
    /// }
    /// ```
    fn sheet<Sheet: RenderObject>(
        self,
        present_modal: PartialBinding<bool>,
    ) -> SheetModalPresenterView<Self, Sheet> {
        SheetModalPresenterView {
            view: self,
            sheet: None,
            binding: present_modal,
        }
    }
}

impl<View: RenderObject, Sheet: RenderObject> SheetModalPresenterView<View, Sheet> {
    #[doc(hidden)]
    pub fn with_capture_callback(
        mut self,
        sheet_fn: impl Fn() -> Sheet + 'static,
        identity: usize,
    ) -> Self {
        self.sheet = Some((Box::new(sheet_fn), identity));
        self
    }
}
impl<T: RenderObject> SheetModifier for T {}
