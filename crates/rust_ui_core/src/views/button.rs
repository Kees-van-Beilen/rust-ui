use std::cell::RefCell;

pub struct Button {
    pub label: String,
    pub callback: RefCell<Box<dyn Fn()>>,
}

pub trait ButtonPartialInit {
    fn init(self)->Button;
}

// impl<T:ToString,F:Fn()+'static> ButtonPartialInit for (T, F) {
//     fn init(self)->Button {
//         Button::create(self.0, self.1)
//     }
// }

// impl<T:ToString> ButtonPartialInit for T {
//     fn init(self)->Button {
//         Button::create(self.0,||{})
//     }
// }


impl Button {
    pub fn create(title:impl ToString, callback:impl Fn()+'static)->Self{
        Button { label: title.to_string(), callback: RefCell::new(Box::new(callback)) }
    }
    pub fn new(init:impl ToString) ->Self {
        Self::create(init, ||{})
    }
    pub fn with_capture_callback(mut self,callback:impl Fn()+'static)->Self {
        self.callback = RefCell::new(Box::new(callback));
        self
    }
}