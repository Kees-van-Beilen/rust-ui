use std::{cell::{Cell, Ref, RefCell, RefMut}, ops::{Deref, DerefMut}, rc::Rc};

use crate::view::mutable::MutableViewRerender;

/*
when specialization becomes available `copy` types should use Cell instead of RefCell

*/

pub struct PartialState<T> {
    data: Rc<RefCell<T>>
}
impl<T> PartialState<T> {
    pub fn get(&self)->Ref<T> {
        self.data.borrow()
    }
}

pub struct State<'a,T> {
    data: Rc<RefCell<T>>,
    signal: &'a Cell<bool>
}


pub struct Binding<T> {
    data: Rc<RefCell<T>>,
    view: Rc<RefCell<crate::native::MutableView>>
}

impl<T> PartialState<T> {
    pub fn as_state<'a>(&self,s:&'a Cell<bool>)->State<'a, T>{
        State {
            data: self.data.clone(),
            signal: s,
        }
    }
}

impl<T> From<T> for PartialState<T> {
    fn from(value: T) -> Self {
        Self { data: Rc::new(RefCell::new(value)) }
    }
}

impl<T> State<'_,T> {
    pub fn get<'a>(&'a self)->Ref<'a, T>{
        //this should never fail because an error should pop up at compile time
        self.data.borrow()
    }
    pub fn get_mut(&mut self)->RefMut<'_,T>{
        self.signal.set(true);
        self.data.borrow_mut()
    }
}

// impl<T> State<'_,T> {
//     pub fn binding(&self)->Binding<T>{
//         Binding { data: self.data.clone(), view: self.view.clone().unwrap() }
//     }
// }

// impl<T> Deref for State<'_,T> {
//     type Target=T;

//     fn deref(&self) -> &Self::Target {
//         unsafe { self.data.try_borrow_unguarded().unwrap() }
//     }
// }

// impl<T> DerefMut for State<'_,T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self.view.unwrap()
//     }
// }

// impl<T> DerefMut for State<'_,T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self.
//     }
// }