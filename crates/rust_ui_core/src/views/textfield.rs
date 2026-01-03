use crate::view::state::{PartialAnyBinding, PartialBindingBox};

pub struct TextField {
    pub text_binding: PartialBindingBox<String>,
    pub(crate) identity: Option<usize>,
}

pub struct TextEditor {
    pub text_binding: PartialBindingBox<String>,
    pub(crate) identity: Option<usize>,
}

impl TextField {
    pub fn new(binding: impl for<'a> PartialAnyBinding<'a, Value = String> + 'static) -> Self {
        Self {
            text_binding: Box::new(binding),
            identity: None,
        }
    }
    
}

impl TextEditor {
    pub fn new(binding: impl for<'a> PartialAnyBinding<'a, Value = String> + 'static) -> Self {
        Self {
            text_binding: Box::new(binding),
            identity: None,
        }
    }
    
}


