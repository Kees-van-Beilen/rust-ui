#![warn(missing_docs)]
//! Manage storage between ui state changes.
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::BTreeMap,
    fmt::Debug,
    rc::Rc,
};

///
/// PersistentStorage is used in mutable views during rendering. It is passed to the children, which can use their identity
/// that has been set using [`crate::layout::RenderObject::set_identity`] to query data that persist during rerender.
///
/// In the future this might change to allow for a more SwiftUI like identity system (which also tracks variable dependencies)
///
/// Currently the views that provide a persistent storage are:
/// - any custom view (with mutable state)
/// - [`crate::views::control_flows::list::ListView`] which in rust_ui are `for loops` however there children do not automatically get an identity (these have to be assigned manually, that way the render system can properly identify each array item no matter the order )
///
#[derive(Default, Debug)]
pub struct PersistentStorage {
    map: BTreeMap<(usize, TypeId), Box<dyn Any>>,
    /// views that want to register to the garbage collection
    /// this happens if a view needs to continue having focus between rerenders
    garbage_collection: BTreeMap<usize, GarbageCollectable>,
}

/// Meta information on variables to be garbage collected.
/// This object keeps track of the cleanup function and if it is time to cleanup the object
pub struct GarbageCollectable {
    remove_fn: Box<dyn FnOnce()>,
    /// before rerendering this flag will be set to false
    /// afterwards if it is still false the `remove_fn` will be called.
    flagged_to_keep: bool,
}
impl Debug for GarbageCollectable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GarbageCollectable")
            .field("flagged_to_keep", &self.flagged_to_keep)
            .finish()
    }
}

/// A wrapper around [`PersistentStorage`] that can be easily cloned
#[derive(Default, Clone, Debug)]
pub struct PersistentStorageRef {
    /// Inner reference to the [`PersistentStorage`]
    pub cell: Rc<RefCell<PersistentStorage>>,
}

impl PersistentStorage {
    /// Get an storage object from an identity and Type.
    pub fn get<T: Any>(&self, identity: usize) -> Option<&T> {
        self.map
            .get(&(identity, TypeId::of::<T>()))
            .and_then(|e| e.downcast_ref())
    }
    /// Get an storage object from an identity and Type or instantiate one
    pub fn get_or_init_with<'a, T: Any>(
        &'a mut self,
        identity: usize,
        init: impl FnOnce() -> T,
    ) -> &'a T {
        match self.get::<T>(identity) {
            None => {
                let item = init();
                self.insert(identity, item);
            }
            _ => {}
        }
        self.get(identity).unwrap()
    }
    /// Get an storage object from an identity and Type or instantiate one and register it with the garbage collector.
    /// However note that you may only register one gc object per identity
    pub fn get_or_register_gc<'a, T: Any, A: FnOnce() + 'static>(
        &'a mut self,
        identity: usize,
        init: impl FnOnce() -> (T, A),
    ) -> &'a T {
        match self.get::<T>(identity) {
            None => {
                let (item, gc) = init();
                self.insert(identity, item);
                self.register_for_garbage_collection(identity, gc)
            }
            _ => {}
        }
        self.get(identity).unwrap()
    }
    /// Insert an storage object for a given identity and type.
    /// This will override any preexisting object with the same type and identity
    pub fn insert<T: Any>(&mut self, identity: usize, item: T) {
        self.map
            .insert((identity, TypeId::of::<T>()), Box::new(item));
    }

    ///
    /// the removal function cannot reference much data, it can reference the native view
    /// and destroy it. As it is an FnOnce every view should only be able to be destroyed once.
    ///
    pub fn register_for_garbage_collection(
        &mut self,
        identity: usize,
        removal_fn: impl FnOnce() + 'static,
    ) {
        self.garbage_collection.insert(
            identity,
            GarbageCollectable {
                remove_fn: Box::new(removal_fn),
                flagged_to_keep: true,
            },
        );
    }
    /// Mark an specific identity as used. This means that it won't be cleaned up
    /// in the next gc cycle.
    pub fn garbage_collection_mark_used(&mut self, identity: usize) {
        if let Some(gc) = self.garbage_collection.get_mut(&identity) {
            gc.flagged_to_keep = true;
        }
    }
    /// After a gc cycle is done, all flags shut be unset, such that views can be collected by gc again.
    pub fn garbage_collection_unset_all(&mut self) {
        for value in self.garbage_collection.values_mut() {
            value.flagged_to_keep = false;
        }
    }
    /// Commit a gc cycle. Calls the destructor of any [`GarbageCollectable`] that is not marked as used.
    pub fn garbage_collection_cycle(&mut self) {
        let removals: Vec<usize> = self
            .garbage_collection
            .iter()
            .filter(|(_, v)| !v.flagged_to_keep)
            .map(|(k, _)| *k)
            .collect();
        for identity in removals.iter() {
            let entry = self.garbage_collection.remove(identity).unwrap();
            (entry.remove_fn)();
        }
    }
}

impl PersistentStorageRef {
    /// Get a reference to the storage
    pub fn borrow(&self) -> std::cell::Ref<'_, PersistentStorage> {
        self.cell.borrow()
    }

    /// Get a mutable reference to the storage
    pub fn borrow_mut(&self) -> std::cell::RefMut<'_, PersistentStorage> {
        self.cell.borrow_mut()
    }
}
