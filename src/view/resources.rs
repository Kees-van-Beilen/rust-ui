use std::{
    any::{Any, TypeId},
    borrow::Cow,
    collections::HashMap,
    fmt::Debug,
};

use clone_dyn::CloneDyn;

// /// Warning this file is very fragile and very unsafe.
// /// Changes must be made with great consideration
// use std::{alloc::{self, Layout}, any::{Any, TypeId}, collections::HashMap, os::{macos::raw, raw::c_void}, rc::Rc};
#[clone_dyn::clone_dyn]
pub trait Resource: Any + Debug {
    //blanket

    fn as_any(&self) -> &dyn Any;
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
        // dbg!(&TypeId::of::<T>());

        let v = self.as_ref().stack.get(&TypeId::of::<T>())?;
        // println!("get {:?} {:?}",TypeId::of::<T>(),(&*v as &dyn Any).is::<Box<dyn Resource>>());
        (v.as_any()).downcast_ref::<T>()
    }
}

// struct RawVec {
//     data: *mut c_void,
//     len: usize,
//     capacity:usize,
//     layout:Layout
// }
// //
// impl RawVec {
//     fn new<T>() -> Self {
//         Self { data: std::ptr::null_mut(), len: Default::default(), capacity: Default::default(), layout: Layout::new::<T>() }
//     }
//     fn from_layout(layout:Layout) -> Self {
//         Self { data: std::ptr::null_mut(), lecan: Default::default(), capacity: Default::default(), layout }
//     }
// }

// pub struct Resources {
//     /// The problem is: We need a vector where the buffer can be transmuted as needed
//     stack:HashMap<std::any::TypeId,RawVec>
// }

// //we have to impl clone here because of the raw vectors
// //we can now also make a funny optimization as we don't need the historic stack values
// //we can just copy the top of every vector
// impl Clone for Resources {
//     fn clone(&self) -> Self {
//         let stack = HashMap::new();

//         for (key,value) in self.stack.iter() {
//             if value.len == 0 {continue;}
//             unsafe {
//                 ///SAFETY: ptr exists and is in bounds
//                 let ptr = value.data.byte_add(value.layout.size()*(value.len-1));
//                 let mut raw_vec = RawVec::from_layout(value.layout);
//                 raw_vec.capacity = 1;
//                 raw_vec.len = 1;
//                 raw_vec.data = alloc::alloc(value.layout) as *mut c_void;
//                 assert_ne!(raw_vec.data,std::ptr::null_mut(),"Out of memory");
//                 let bytes = std::slice::from_raw_parts(ptr as *mut u8, value.layout.size());
//                 for i in 0..bytes.len(){
//                     (raw_vec.data as *mut u8).byte_add(i).write(bytes[i]);
//                 }
//                 std::ptr::copy_nonoverlapping(src, dst, count);

//             }
//             // value.layout
//         }

//         Self {
//             stack
//         }
//     }
// }
// pub struct ResourceStack<'a>(&'a mut Resources);

// const fn assert_non_zero_size<T>() {
//     assert!(size_of::<T>() != 0, "Generic type T must not be zero-sized");
// }

// impl<'a> ResourceStack<'a> {
//     pub fn amend_with<T:Resource>(&mut self,element:T,with_fn:impl FnOnce(&mut ResourceStack)) {
//         self.0.add::<T>(element);
//         with_fn(self);
//         self.0.remove_last::<T>();
//     }
//     pub fn get_resource<T:Resource>(&self)->Option<&T> {
//         self.0.get_last()
//     }
// }

// impl Resources {

//     fn get_last<T:Resource>(&self)->Option<&T>{
//         let k = self.stack.get(&TypeId::of::<T>())?;
//         let slice = unsafe { std::slice::from_raw_parts(k.data as *const T, k.len) };
//         slice.last()
//     }

//     fn remove_last<T:Resource>(&mut self) {
//         let k = self.stack.entry(TypeId::of::<T>()).or_insert_with(||RawVec::new::<T>());
//         if k.len != 0 {
//             k.len -= 1;
//         }
//     }

//     fn add<T:Resource>(&mut self, element: T) {
//         //this should give a error at compile time
//         //thus making the `alloc::alloc` call safe
//         const {
//             assert_non_zero_size::<T>();
//         }
//         let k = self.stack.entry(TypeId::of::<T>()).or_insert_with(||RawVec::new::<T>());
//         if k.len == 0 {
//             // SAFETY: see `assert_non_zero_size::<T>();`
//             k.data = unsafe { alloc::alloc(Layout::new::<T>()) } as *mut c_void;
//             k.capacity = 1;
//             //TODO: this should be marked cold
//             assert_ne!(k.data,std::ptr::null_mut(),"Out of memory")
//         }else if k.len == k.capacity {
//             let layout = Layout::new::<T>();
//             k.capacity *= 2;
//             k.data = unsafe {
//                 // SAFETY:
//                 // - `k.data` is a non null ptr allocated only by `alloc::alloc`
//                 // - Layout is the same by the generics mechanism
//                 // - `k.len != 0`
//                 alloc::realloc(k.data as *mut u8, layout, layout.size()*k.capacity)
//             } as *mut c_void;
//             //TODO: this should be marked cold
//             assert_ne!(k.data,std::ptr::null_mut(),"Out of memory")
//         }
//         unsafe {
//             // SAFETY:
//             // ptr and resulting ptr in bounds of the allocated space for the *T arena
//             (k.data as *mut T).add(k.len).write(element)
//         };
//         k.len += 1;
//     }
// }

// impl Drop for Resources {
//     fn drop(&mut self) {
//         //we have to deallocate the vectors here
//         for (_,value) in self.stack.iter() {
//             if value.capacity != 0 {
//                 //SAFETY:
//                 //- Layout guaranteed to be the same as when allocated
//                 //- Allocated by the same allocator
//                 unsafe { alloc::dealloc(value.data as *mut u8, value.layout) };
//             }
//         }
//     }
// }
