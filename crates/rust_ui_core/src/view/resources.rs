use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
};

#[clone_dyn::clone_dyn]
pub trait Resource: Any + Debug {
    fn as_any(&self) -> &dyn Any;
}

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


#[derive(Default, Debug, Clone)]
pub struct Resources {
    stack: HashMap<TypeId, Box<dyn Resource>>,
}

#[derive(Debug)]
pub enum ResourceStack<'a> {
    Owned(Resources),
    Borrow(&'a mut Resources),
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
    fn as_mut(&mut self) -> &mut Resources {
        match self {
            ResourceStack::Owned(resources) => resources,
            ResourceStack::Borrow(resources) => *resources,
        }
    }
    pub fn as_ref(&self) -> &Resources {
        match self {
            ResourceStack::Owned(resources) => resources,
            ResourceStack::Borrow(resources) => *resources,
        }
    }
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
    pub fn get_resource<T: Resource>(&self) -> Option<&T> {
        let v = self.as_ref().stack.get(&TypeId::of::<T>())?;
        (v.as_any()).downcast_ref::<T>()
    }
}
