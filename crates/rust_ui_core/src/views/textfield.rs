use crate::view::state::PartialBinding;

pub struct TextField {
    pub text_binding:PartialBinding<String>,
    pub (crate)identity:Option<usize>
}

impl TextField {
    pub fn new(binding:PartialBinding<String>)->Self {
        Self {
            text_binding: binding,
            identity:None
        }
    }
}