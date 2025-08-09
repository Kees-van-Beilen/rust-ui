use std::{
    cell::{Cell, Ref, RefCell, RefMut},
    collections::{BTreeMap, BTreeSet, HashSet},
    fmt::Display,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::view::mutable::MutableViewRerender;

/*
when specialization becomes available `copy` types should use Cell instead of RefCell

*/

pub struct PartialState<T> {
    data: Rc<RefCell<T>>,
}
impl<T> PartialState<T> {
    pub fn get<'a>(&'a self) -> Ref<'a, T> {
        self.data.borrow()
    }
}
impl<T> PartialBinding<T> {
    pub fn get<'a>(&'a self) -> Ref<'a, T> {
        self.data.borrow()
    }

    pub fn as_binding<'a>(&'a self,queue:&'a BindingQueue<'a>) -> Binding<'a, T> {
        Binding {
            data: &self.data,
            updater: &self.updater.1,
            view: self.updater.0,
            queue:queue
            
        }
    }
}

pub struct State<'a, T> {
    data: Rc<RefCell<T>>,
    signal: &'a Cell<bool>,
}

pub struct PartialBinding<T> {
    data: Rc<RefCell<T>>,
    updater: (*const u8, Box<dyn Fn()>),
}

struct Subscriber {
    //
    identifier: usize,
}
pub struct Binding<'a, T> {
    data: &'a Rc<RefCell<T>>,
    view: *const u8,
    updater: &'a Box<dyn Fn()>,
    queue: &'a BindingQueue<'a>
}

impl<T> Binding<'_,T> {
    pub fn get<'a>(&'a self) -> Ref<'a, T> {
        //this should never fail because an error should pop up at compile time
        self.data.borrow()
    }
    pub fn get_mut(&mut self) -> RefMut<'_, T> {
        // self
        self.queue.add(self.view, self.updater);
        self.data.borrow_mut()
    }
}
impl<T> PartialState<T> {
    pub fn as_state<'a>(&self, s: &'a Cell<bool>) -> State<'a, T> {
        State {
            data: self.data.clone(),
            signal: s,
        }
    }
    pub fn as_binding<V: crate::view::mutable::MutableView + 'static>(
        &self,
        view: Rc<RefCell<V>>,
    ) -> PartialBinding<T> {
        PartialBinding {
            data: self.data.clone(),
            updater: (
                Rc::as_ptr(&view) as *const u8,
                Box::new(move || view.rerender()),
            ),
        }
    }
}
#[derive(Default)]
pub struct BindingQueue<'a> {
    views_to_update: RefCell<BTreeMap<*const u8, &'a Box<dyn Fn()>>>,
}
impl<'a> BindingQueue<'a> {
    pub fn add<'b:'a>(&self,view:*const u8,updater:&'b Box<dyn Fn()>){
        self.views_to_update.borrow_mut().insert(view, updater);
    }
    pub fn execute(&self) {
        for view in self.views_to_update.borrow().values() {
            view();
        }
    }
}

//This panic should be removed
impl<T> Default for PartialBinding<T> {
    fn default() -> Self {
        Self {
            data: unsafe { Rc::new_uninit().assume_init() },
            updater: (0 as *const u8, Box::new(|| {})),
        }
    }
}

impl<T> From<T> for PartialState<T> {
    fn from(value: T) -> Self {
        Self {
            data: Rc::new(RefCell::new(value)),
        }
    }
}

impl<T> State<'_, T> {
    pub fn get<'a>(&'a self) -> Ref<'a, T> {
        //this should never fail because an error should pop up at compile time
        self.data.borrow()
    }
    pub fn get_mut(&mut self) -> RefMut<'_, T> {
        self.signal.set(true);
        self.data.borrow_mut()
    }
}

impl<T: Display> Display for PartialState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}
impl<T: Display> Display for PartialBinding<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}
impl<T: Display> Display for Binding<'_,T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
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
