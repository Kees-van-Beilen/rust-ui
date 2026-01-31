//! Wrapper around a native editable text
use crate::view::state::{PartialAnyBinding, PartialBindingBox};

///
/// TextField view allows user text input inside of a native text field element.
/// The textfield continuously updates the text_binding to reflect the current
/// text value of the text field.
pub struct TextField {
    /// Current value of the text field.
    pub text_binding: PartialBindingBox<String>,
    pub(crate) identity: Option<usize>,
}

/// A multiline variant of the [`TextField`] that also manages scrolling
pub struct TextEditor {
    /// Current value of the text editor.
    pub text_binding: PartialBindingBox<String>,
    pub(crate) identity: Option<usize>,
}

impl TextField {
    /// Construct a new text field with a given binding
    pub fn new(binding: impl for<'a> PartialAnyBinding<'a, Value = String> + 'static) -> Self {
        Self {
            text_binding: Box::new(binding),
            identity: None,
        }
    }
}

impl TextEditor {
    /// Construct a new text editor with a given binding
    pub fn new(binding: impl for<'a> PartialAnyBinding<'a, Value = String> + 'static) -> Self {
        Self {
            text_binding: Box::new(binding),
            identity: None,
        }
    }
}
