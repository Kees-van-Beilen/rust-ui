use std::cell::RefCell;

pub struct Button {
    pub label: String,
    pub callback: RefCell<Box<dyn Fn()>>,
}
