use crate::view::mutable::MutableViewRerender;
use std::{
    cell::{Cell, Ref, RefCell, RefMut},
    collections::BTreeMap,
    fmt::Display,
    rc::Rc,
};

/*
when specialization becomes available `copy` types should use Cell instead of RefCell.
A lot of this can be optimized better.
*/

///
/// Used internally when a property is marked with `#[state]`. A partial state can be turned into a [`PartialBinding`] or [`State`].
/// Partial states can be accessed in the `view!` body of a view.
/// A partial state is never mutable as otherwise ui code could trigger rerenders and cause a loop.
///
pub struct PartialState<T> {
    data: Rc<RefCell<T>>,
}

impl<T> PartialState<T> {
    ///
    /// Get a reference to the interior value of the partial state
    /// ## Example
    /// ```
    /// # #![feature(more_qualified_paths,default_field_values)]
    /// # use rust_ui::prelude::*;
    /// #[ui(main)]
    /// struct RootView {
    ///     #[state] counter: i32 = 0,
    ///     body:_ = view!{
    ///         HStack {
    ///             Text(format!("counter: {}",counter.get()))
    ///         }
    ///     }
    /// }
    /// ```
    pub fn get<'a>(&'a self) -> Ref<'a, T> {
        self.data.borrow()
    }
}
impl<T> PartialBinding<T> {
    ///
    ///  Get a reference to the interior value of the partial binding. Similar to [`PartialState::get`].
    /// ## Example
    /// ```
    /// # #![feature(more_qualified_paths,default_field_values)]
    /// # use rust_ui::prelude::*;
    /// #[ui]
    /// struct MyView {
    ///     #[binding] value: i32,
    ///     body:_ = view!{
    ///         HStack {
    ///             Text(format!("binding value: {}",value.get()))
    ///         }
    ///     }
    /// }
    /// ```
    pub fn get<'a>(&'a self) -> Ref<'a, T> {
        self.data.borrow()
    }

    ///
    /// Used internally. Convert A partial binding into a mutable [`Binding`]
    /// ## Inputs
    /// To create a binding a [`BindingQueue`] is required. This queue schedules the view rerenders.
    /// The queue has to manually be executed.
    #[doc(hidden)]
    pub fn as_binding<'a>(&'a self, queue: &'a BindingQueue<'a>) -> Binding<'a, T> {
        Binding {
            data: &self.data,
            updater: &self.updater.1,
            view: self.updater.0,
            queue: queue,
        }
    }

    /// update the value in a binding and rerender
    pub(crate) fn update_value(&self, value: T) {
        self.data.replace(value);
        self.updater.1();
    }
}

///
/// Used internally to represent mutable state. This structure is automatically created for every [PartialState] of the view at the `view!` callback blocks.
/// Like in the `.on_tap` modifier or [`crate::views::Button`] view.
///
pub struct State<'a, T> {
    data: Rc<RefCell<T>>,
    signal: &'a Cell<bool>,
}

///
/// Used internally when a property is marked with `#[binding]`. A partial binding can be turned into a [`Binding`]
/// Partial bindings can be accessed in the `view!` body of a view.
/// A partial binding is never mutable as otherwise ui code could trigger rerenders and cause a loop.
///
#[derive(Clone)]
pub struct PartialBinding<T> {
    data: Rc<RefCell<T>>,
    updater: (*const u8, Rc<Box<dyn Fn()>>),
}

///
/// Used internally to represent mutable binding. This structure is automatically created for every [PartialBinding] of the view at the `view!` callback blocks.
/// Like in the `.on_tap` modifier or [`crate::views::Button`] view.
///
pub struct Binding<'a, T> {
    data: &'a Rc<RefCell<T>>,
    view: *const u8,
    updater: &'a Box<dyn Fn()>,
    queue: &'a BindingQueue<'a>,
}

impl<T> Binding<'_, T> {
    ///
    ///  Get a reference to the interior value of the binding. Similar to [`State::get`].
    /// ## Example
    /// ```
    /// # #![feature(more_qualified_paths,default_field_values)]
    /// # use rust_ui::prelude::*;
    /// #[ui]
    /// struct MyView {
    ///     #[binding] counter: i32,
    ///     body:_ = view!{
    ///         HStack {
    ///             Button("click me") || {
    ///                 *counter.get_mut() += 1;
    ///                 println!("pressed {} times",counter.get());
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn get<'a>(&'a self) -> Ref<'a, T> {
        //this should never fail because an error should pop up at compile time
        self.data.borrow()
    }
    ///
    /// Get a mutable reference to the interior value of the binding. Similar to [`State::get_mut`].
    /// There can only be one mutable borrow at a time, and rust borrow check rules must still be satisfied.
    /// ## Example
    /// ```
    /// # #![feature(more_qualified_paths,default_field_values)]
    /// # use rust_ui::prelude::*;
    /// #[ui]
    /// struct MyView {
    ///     #[binding] counter: i32,
    ///     body:_ = view!{
    ///         HStack {
    ///             Button("click me") || {
    ///                 *counter.get_mut() += 1;
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn get_mut(&mut self) -> RefMut<'_, T> {
        //event though we don't need self to be mutable, we still mark it as such to avoid an error at runtime if rust borrow rules are ignored
        self.queue.add(self.view, self.updater);
        self.data.borrow_mut()
    }
}
impl<T> PartialState<T> {
    ///
    /// Used internally. Convert A partial state into a mutable [`State`]
    /// ## Inputs
    /// To create a state, a [`Cell<bool>`] is required. This is the "signal" that indicates that any state of the current view has changed.
    /// One must still check if the signal was set and rerender the view accordingly.
    #[doc(hidden)]
    pub fn as_state<'a>(&self, s: &'a Cell<bool>) -> State<'a, T> {
        State {
            data: self.data.clone(),
            signal: s,
        }
    }
    ///
    /// **If you need to turn a variable marked as `#[state]` into a binding, use the `bind!` macro instead of this function**
    /// ## Example
    /// ```
    /// # #![feature(more_qualified_paths,default_field_values)]
    /// # use rust_ui::prelude::*;
    ///
    /// #[ui]
    /// struct MyView {
    ///     #[binding] value:String,
    ///     body: _ = view!{
    ///         Text(value.get().as_str())
    ///     }
    /// }
    ///
    /// #[ui(main)]
    /// struct RootView {
    ///     #[state] my_string: String = "Hello World".to_string(),
    ///     body:_ = view!{
    ///         MyView{
    ///             value:bind!(my_string)
    ///         }
    ///     }
    /// }
    /// ```
    pub fn as_binding<V: crate::view::mutable::MutableView + 'static>(
        &self,
        view: Rc<RefCell<V>>,
    ) -> PartialBinding<T> {
        PartialBinding {
            data: self.data.clone(),
            updater: (
                Rc::as_ptr(&view) as *const u8,
                Rc::new(Box::new(move || view.rerender())),
            ),
        }
    }
}

///
/// Used internally. The binding queue is attached to every [`Binding`] to keep track of which views to rerender after mutating a binding.
///
#[derive(Default)]
pub struct BindingQueue<'a> {
    views_to_update: RefCell<BTreeMap<*const u8, &'a Box<dyn Fn()>>>,
}

impl<'a> BindingQueue<'a> {
    ///
    /// Add a rerender function to the binding queue (if it wasn't already present)
    /// ## Inputs
    /// - The first argument `view` is a pointer obtained from [`Rc::as_ptr`] that points to the rc refcell containing a [crate::view::mutable::MutableView] object.
    /// - The second argument `updater` is a reference to a closure that calls [crate::view::mutable::MutableViewRerender::rerender].
    /// This has to be done using a closure as [crate::view::mutable::MutableViewRerender] is a trait object.
    pub fn add<'b: 'a>(&self, view: *const u8, updater: &'b Box<dyn Fn()>) {
        self.views_to_update.borrow_mut().insert(view, updater);
    }
    ///
    /// Execute all the scheduled view rerenders
    ///
    pub fn execute(&self) {
        for view in self.views_to_update.borrow().values() {
            view();
        }
    }
}

impl<T> Default for PartialBinding<T> {
    fn default() -> Self {
        //This is horrible and should be removed as fast as possible
        //it is here as a patch for the `..Default::default()` behavior. However an assertion should happen at compile time for future versions of rust-ui
        Self {
            data: unsafe { Rc::new_uninit().assume_init() },
            updater: (0 as *const u8, Rc::new(Box::new(|| {}))),
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
    ///
    ///  Get a reference to the interior value of the state. Similar to [`Binding::get`].
    /// ## Example
    /// ```
    /// # #![feature(more_qualified_paths,default_field_values)]
    /// # use rust_ui::prelude::*;
    /// #[ui]
    /// struct MyView {
    ///     #[state] counter: i32,
    ///     body:_ = view!{
    ///         HStack {
    ///             Button("click me") || {
    ///                 *counter.get_mut() += 1;
    ///                 println!("pressed {} times",counter.get());
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn get<'a>(&'a self) -> Ref<'a, T> {
        //this should never fail because an error should pop up at compile time
        self.data.borrow()
    }
    ///
    ///  Get a mutable reference to the interior value of the state. Similar to [`Binding::get_mut`].
    /// ## Example
    /// ```
    /// # #![feature(more_qualified_paths,default_field_values)]
    /// # use rust_ui::prelude::*;
    /// #[ui]
    /// struct MyView {
    ///     #[state] counter: i32,
    ///     body:_ = view!{
    ///         HStack {
    ///             Button("click me") || {
    ///                 *counter.get_mut() += 1;
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
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
impl<T: Display> Display for Binding<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}
