use std::ops::Deref;

use crate::view::state::{ PartialAnyBinding, PartialBinding, PartialBindingBox};

pub struct TextField {
    pub text_binding:PartialBindingBox<String>,
    pub (crate)identity:Option<usize>
}

impl TextField {
    pub fn new(binding:impl for<'a> PartialAnyBinding<'a,Value = String>+'static)->Self {
        Self {
            text_binding: Box::new(binding),
            identity:None
        }
    }
}