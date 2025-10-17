use std::cell::RefCell;

pub mod helper;
pub mod callback;
mod views;






pub mod native {
    //global env was a bad idea. but I don't see any other way.
    //the idea is that whenever java calls back into native, we update the env.
    thread_local! {
        pub static ENV:RefCell<*mut jni::JNIEnv<'static>> = RefCell::new(std::ptr::null_mut());
        
    }
    //maybe this wil work better
    pub static mut ACTIVITY:*mut jni::sys::jobject = unsafe { mem::transmute(std::ptr::null_mut() as *mut jni::sys::jobject) };

    pub use super::helper;
    use crate::{
        android_println, get_env, layout::{ComputableLayout, Position, RenderObject, Size}, native::helper::Retained, retain, view::{
            persistent_storage::PersistentStorageRef,
            resources::{Resource, ResourceStack, Resources},
        }
    };
    // pub use crate::native::android::ENV;
    use android2_android::{app::{Activity, Fragment}, content::{Context, ContextWrapper}, view::{ContextThemeWrapper, ViewGroup}};
    pub use jni;
    use jni::objects::JObject;
    use std::{cell::RefCell, mem, rc::Rc};


    ///
    /// Note for the android implementors:
    /// - You cannot create a local frame this is UB see https://github.com/jni-rs/jni-rs/issues/392
    /// - So all objects must live! (By using retained, or by being a child of a retained)
    // #[derive(Clone)]
    pub struct RenderData<'a, 'jni> {
        pub stack: ResourceStack<'a>,
        pub persistent_storage: PersistentStorageRef,
        pub parent:Retained<android2_android::view::ViewGroup<'static>>,
        pub instance:android2_android::app::Activity<'static>,
        pub jni:jni::JNIEnv<'jni>
    }

    pub trait ActivityExtension {
        fn context(&self)->&Context;
    }
    impl<'jni> ActivityExtension for android2_android::app::Activity<'jni> {
        fn context(&self)->&Context {
            let a: &ContextThemeWrapper = self.as_ref();
            let b: &ContextWrapper = a.as_ref();
            b.as_ref()
        }
    }

    impl<'a, 'jni> Clone for RenderData<'a, 'jni> {
        fn clone(&self) -> Self {
            Self {
                stack: self.stack.clone(),
                persistent_storage: self.persistent_storage.clone(),
                parent: self.parent.clone(),
                instance:unsafe { mem::transmute_copy(&self.instance) },
                jni:unsafe { self.jni.unsafe_clone() }
            }
        }
    }

    impl RenderData<'_, '_> {

     
        pub fn ament_with<T: Resource, F, K>(&mut self, element: T, with_fn: F) -> K
        where
            for<'b> F: FnOnce(RenderData) -> K,
        {
            self.stack.amend_with(element, |stack_e| {
                let d = RenderData {
                    stack: ResourceStack::Borrow(stack_e),
                    persistent_storage: self.persistent_storage.clone(),
                    parent:self.parent.clone(),
                    instance:unsafe { mem::transmute_copy(&self.instance) },
                    jni:unsafe{ self.jni.unsafe_clone()}
                };

                with_fn(d)
            })
        }
    }
    pub struct MutableView {
        layout_size: Size<f64>,
        layout_position: Position<f64>,
        children: Box<dyn ComputableLayout>,
        stack: crate::view::resources::Resources,
        persistent_storage: PersistentStorageRef,
        parent: Retained<ViewGroup<'static>>
    }

    impl<V: crate::view::mutable::MutableView + 'static> crate::view::mutable::MutableViewRerender for ::std::rc::Rc<::std::cell::RefCell<V>> {
        fn rerender(&self) {
            let mut data = self.borrow_mut();

            if let Some(k) = &mut data.get_mut_attached() {
                let render_data = {
                    let mut b = k.borrow_mut();
                    b.children.destroy();
                    let render_data = RenderData {
                        parent:b.parent.clone(),
                        jni:unsafe { get_env!().unsafe_clone() },
                        instance:unsafe { mem::transmute(ACTIVITY) },
                        // real_parent: b.parent.clone(),
                        // parent:
                        // persistent_storage:
                        //TODO: fix this clone to a ref
                        stack: crate::view::resources::ResourceStack::Owned(b.stack.clone()),
                        persistent_storage: b.persistent_storage.clone(),
                    };
                    render_data
                };
                android_println!("happy path");
                drop(data);
                let _ = self.render(render_data);
            } else {
                android_println!("unhappy path");
                drop(data);
            }
        }
    }
    impl ComputableLayout for Rc<RefCell<MutableView>> {
        fn set_size(&mut self, to: Size<f64>) {
            self.borrow_mut().layout_size = to;
            self.borrow_mut().children.set_size(to);
        }

        fn set_position(&mut self, to: Position<f64>) {
            self.borrow_mut().layout_position = to;
            self.borrow_mut().children.set_position(to);
        }

        fn destroy(&mut self) {
            self.borrow_mut().children.destroy();
            // self.borrow_mut().parent = unsafe { Retained::from_raw(objc2::ffi::Nil) }.unwrap().downcast().unwrap();
        }
        fn preferred_size(&self, in_frame: &Size<f64>) -> Size<Option<f64>> {
            self.borrow().children.preferred_size(in_frame)
        }
    }
    impl<T: crate::view::mutable::MutableView + 'static> RenderObject for Rc<RefCell<T>> {
        type Output = Rc<RefCell<crate::native::MutableView>>;

        fn render(&self, data: crate::native::RenderData) -> Self::Output {
            // todo!()
            // let view = T::children(self.clone());
            // let parent = data.parent.clone();
            // let child = view.render(data);
            // Rc::new(RefCell::new(MutableView {
            //     layout_size: Size::splat(0.0),
            //     layout_position: Position{x:0.0,y:0.0},
            //     children: Box::new(child),
            //     stack: Default::default(),
            //     persistent_storage: Default::default(),
            //     parent: parent
            // }))
            type Store<T> = (Resources, PersistentStorageRef, Rc<RefCell<T>>);
            let identity = self.borrow().get_identity();
            let mut borrow = data.persistent_storage.borrow_mut();
            let mut resume_storage = true;
            // let mut did_swap = false;
            // let mut did_try_swap = false;
            let (res, storage, self_container) =
                borrow.get_or_init_with::<Store<T>>(identity, || {
                    resume_storage = false;
                    (
                        data.stack.as_ref().clone(),
                        PersistentStorageRef::default(),
                        self.clone(),
                    )
                });

            //we need to copy the state from the last
            if !Rc::ptr_eq(self, self_container) {
                // this code will execute iff the something else is rerendering this view in the same
                // frame that this view's state is updated.
                // this happens when a view updates a binding and a state variable at the same time
                self_container
                    .borrow()
                    .clone_bindings(&mut self.borrow_mut());
            }

            let new_data = RenderData {
                // real_parent: data.real_parent,
                parent:data.parent,
                jni:unsafe { get_env!().unsafe_clone() },
                instance:unsafe {
                    mem::transmute(ACTIVITY)
                },
                stack: ResourceStack::Owned(res.clone()),
                persistent_storage: storage.clone(),
            };
            drop(borrow);
            new_data
                .persistent_storage
                .borrow_mut()
                .garbage_collection_unset_all();
            let r = T::children(self.clone()).render(new_data.clone());
            new_data
                .persistent_storage
                .borrow_mut()
                .garbage_collection_cycle();

            let view = Rc::new(RefCell::new(MutableView {
                children: Box::new(r),
                layout_size: crate::layout::Size {
                    width: 0.0,
                    height: 0.0,
                },

                parent: new_data.parent,
                stack: match new_data.stack {
                    ResourceStack::Owned(resources) => resources,
                    ResourceStack::Borrow(resources) => resources.clone(),
                },
                persistent_storage: data.persistent_storage,
                layout_position: crate::layout::Position::default(),
            }));

            let mut m = self.borrow_mut();
            let mut attached = m.get_mut_attached();
            if let Some(k) = &mut attached {
                k.swap(&view);
                k.set_size(view.borrow().layout_size);
                k.set_position(view.borrow().layout_position);
            } else {
                *attached = Some(view.clone());
            }

            m.get_attached().clone().unwrap()
        }
        fn set_identity(self, identity: usize) -> Self {
            self.borrow_mut().set_identity(identity);
            self
        }
    }

    pub struct NativeImageHandle {}

    impl NativeImageHandle {
        pub fn from_path(path: impl ToString) -> Self {
            todo!()
        }
    }

    pub fn launch_application_with_view<'local>(
        root: impl RenderObject + 'static,
        mut env: jni::JNIEnv<'local>,
        instance: jni::objects::JObject<'local>,
    ) {
        let context = android2_android::content::Context::from(unsafe {
            jni::objects::JObject::from_raw(instance.as_raw())
        });
        let object: &android2_java::lang::Object = context.as_ref();
        let instance = android2_android::app::Activity::from(instance);
        
        let resources = context.get_resources(&mut env);
        let metrics = resources.get_display_metrics(&mut env);
        let height = env.get_field(&metrics, "heightPixels", "I").unwrap().i().unwrap();
        let width = env.get_field(&metrics, "widthPixels", "I").unwrap().i().unwrap();
        // root.render(RenderData {
        //     stack: Default::default(),
        //     persistent_storage: Default::default()
        // });  

        //register custom panic handler
        std::panic::set_hook(Box::new(|info| {
            android_println!("Rust panic: {info}");
        }));
        // env.clo

        // let cls = object.get_class(&mut env);
        // let cls_loader = cls.get_class_loader(&mut env);
        // let bb = unsafe { env.new_direct_byte_buffer(CLASSES.as_ptr() as *mut u8, CLASSES.len()).unwrap() };
        // let bb:android2_java::nio::ByteBuffer = unsafe { mem::transmute(bb) };
        // let in_memory_class_loader = android2_dalvik::system::InMemoryDexClassLoader::new_2(&bb, &cls_loader, &mut env);
        // let in_memory_class_loader:android2_dalvik::system::BaseDexClassLoader = in_memory_class_loader.into();
        // let in_memory_class_loader:android2_java::lang::ClassLoader = in_memory_class_loader.into();
        // let str = env.new_string("RustUIOnClickCallback").unwrap();
        // let str:android2_java::lang::String = unsafe { mem::transmute(str) };

        // let rust_test_cls = in_memory_class_loader.load_class_0(&str,&mut env);
        // let rust_test_constructors = rust_test_cls.get_constructors(&mut env);
        // let constructor = env.get_object_array_element(rust_test_constructors, 0).unwrap();
        // let constructor:android2_java::lang::reflect::Constructor = unsafe { mem::transmute(constructor) };
        // let empty_array = jni::objects::JObjectArray::default();
        // let rust_test_class_instance = constructor.new_instance(&empty_array, &mut env);

        android_println!("start");

        let window = instance.get_window(&mut env);
        // let decor = window.get_decor_view(&mut env);
        // let width = decor.get_width(&mut env);
        // let height = decor.get_height(&mut env);
        // window.set_content_view_1(arg0, env);
        let native_root = android2_android::widget::RelativeLayout::new_0(&context, &mut env);
        let native_root_group: &ViewGroup = native_root.as_ref();
        let native_root_view = native_root_group.as_ref();
        let retained_root = retain!(native_root_view,env);
        instance.set_content_view_1(&native_root_view, &mut env);
        // let instance_object: &jni::objects::JObject = instance.as_ref();
        // let inst_o: &jni::objects::JObject = instance.as_ref();
        
        unsafe {ACTIVITY = mem::transmute(instance)};

        ENV.with(|e|*e.borrow_mut() = (&mut env as *mut jni::JNIEnv<'local>)as *mut jni::JNIEnv<'static>);
        android_println!("start render");
        let mut rendered_view = root.render(RenderData {
            stack: Default::default(),
            persistent_storage: Default::default(),
            parent: retained_root,
            instance:unsafe{mem::transmute(ACTIVITY)},
            jni:env
        });
        android_println!("start set size");

        rendered_view.set_size(Size {
            width: width as f64,
            height: height as f64,
        });
        android_println!("set size {width} {height}");

        rendered_view.set_position(Position { x: 0.0, y: 0.0 });


        // root.render(data)

        // native_root_view.set_background_color(0xff0000, &mut env);
        // let native_root = instance.set_content_view_0(arg0, env);
        android_println!("Hello rust ui");
    }
}
