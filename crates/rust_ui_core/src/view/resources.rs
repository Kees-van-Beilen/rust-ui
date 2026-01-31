#![warn(missing_docs)]
//! Pass down resources through the ui view hierarchy
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
};


///
/// Resources a scoped data structure that can be passed down to children.
/// For instance [`crate::views::text::TintColor`] can be set on a root view, 
/// and its children will still have access to the same resource, or it can 
/// even be changed for specific children.
#[clone_dyn::clone_dyn]
pub trait Resource: Any + Debug {
    ///
    /// Reference this resource as a &dyn Any
    ///
    fn as_any(&self) -> &dyn Any;
}

///
/// Implement the [`Resource`] trait, without writing the boiler plate
///
#[macro_export]
macro_rules! impl_resource {
    ($name:ident) => {
        impl Resource for $name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}

///
/// Used Internally. This is a collection of [`Resource`] objects, every type that implements [`Resource`] may only have one entry in this structure.
///
#[derive(Default, Debug, Clone)]
pub struct Resources {
    stack: HashMap<TypeId, Box<dyn Resource>>,
}

///
/// Used Internally. This is passed to view when its rendered. It might be a reference to a resource stack, or in the case of multiple children a clone.
/// This structure is similar to a [`std::borrow::Cow`]
///
#[derive(Debug)]
pub enum ResourceStack<'a> {
    /// Owned variant of the resource stack. This is usually handed to the last child in the hierarchy.
    /// But no guarantees can be made on if you will receive the owned variant.
    Owned(Resources),
    /// Borrowed variant 
    Borrow(&'a mut Resources),
}
impl Default for ResourceStack<'_> {
    fn default() -> Self {
        ResourceStack::Owned(Default::default())
    }
}

impl<'a> Clone for ResourceStack<'a> {
    fn clone(&self) -> Self {
        match self {
            Self::Owned(arg0) => Self::Owned(arg0.clone()),
            Self::Borrow(arg0) => Self::Owned((*arg0).clone()),
        }
    }
}

impl<'a> ResourceStack<'a> {
    ///
    /// Get a mut reference to the underlying resources.
    ///
    fn as_mut(&mut self) -> &mut Resources {
        match self {
            ResourceStack::Owned(resources) => resources,
            ResourceStack::Borrow(resources) => *resources,
        }
    }
    ///
    /// Get a reference to the underlying resources.
    ///
    pub fn as_ref(&self) -> &Resources {
        match self {
            ResourceStack::Owned(resources) => resources,
            ResourceStack::Borrow(resources) => *resources,
        }
    }
    ///
    /// Temporarily add/overwrite a resource
    ///
    pub fn amend_with<T: Resource, F, K>(&mut self, element: T, with_fn: F) -> K
    where
        for<'b> F: FnOnce(&mut Resources) -> K,
    {
        let element_container: Box<dyn Resource> = Box::new(element);
        let old = self
            .as_mut()
            .stack
            .insert(TypeId::of::<T>(), element_container);
        let a = with_fn(self.as_mut());
        if let Some(old) = old {
            self.as_mut().stack.insert(TypeId::of::<T>(), old);
        } else {
            self.as_mut().stack.remove(&TypeId::of::<T>());
        }

        a
    }
    ///
    /// Get a resource if it exists in the Resources collection, otherwise return None
    ///
    pub fn get_resource<T: Resource>(&self) -> Option<&T> {
        let v = self.as_ref().stack.get(&TypeId::of::<T>())?;
        (v.as_any()).downcast_ref::<T>()
    }
}
