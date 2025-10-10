use std::cell::RefCell;

pub mod helper;
mod views;

thread_local! {
    pub static ENV:RefCell<*mut jni::JNIEnv<'static>> = RefCell::new(std::ptr::null_mut());
}

pub mod native {
    pub use super::helper;
    use crate::{
        android_println, layout::{ComputableLayout, Position, RenderObject, Size}, native::android::ENV, view::{
            persistent_storage::PersistentStorageRef,
            resources::{Resource, ResourceStack},
        }
    };
    pub use jni;
    use std::{cell::RefCell, mem, rc::Rc};
    // #[derive(Clone)]
    pub struct RenderData<'a, 'jni> {
        pub stack: ResourceStack<'a>,
        pub persistent_storage: PersistentStorageRef,
        pub parent: android2_android::view::ViewGroup<'jni>,
        pub context: android2_android::content::Context<'jni>
    }
    impl<'a, 'jni> Clone for RenderData<'a, 'jni> {
        fn clone(&self) -> Self {
            Self {
                stack: self.stack.clone(),
                persistent_storage: self.persistent_storage.clone(),
                parent: android2_android::view::ViewGroup::from(unsafe {
                    jni::objects::JObject::from_raw(self.parent.as_ref().as_raw())
                }),
                context:unsafe { mem::transmute_copy(&self.context) }
            }
        }
    }

    impl RenderData<'_, '_> {
        pub fn ament_with<T: Resource, F, K>(&mut self, element: T, with_fn: F) -> K
        where
            for<'b> F: FnOnce(RenderData) -> K,
        {
            todo!()
        }
    }
    pub struct MutableView {
        layout_size: Size<f64>,
        layout_position: Position<f64>,
        children: Box<dyn ComputableLayout>,
        stack: crate::view::resources::Resources,
        persistent_storage: PersistentStorageRef,
    }

    impl<V> crate::view::mutable::MutableViewRerender for ::std::rc::Rc<::std::cell::RefCell<V>> {
        fn rerender(&self) {
            todo!()
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
            let view = T::children(self.clone());
            let child = view.render(data);
            Rc::new(RefCell::new(MutableView {
                layout_size: Size::splat(0.0),
                layout_position: Position{x:0.0,y:0.0},
                children: Box::new(child),
                stack: Default::default(),
                persistent_storage: Default::default(),
            }))
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
        let instance = android2_android::app::Activity::from(instance);
        let fragment:android2_android::app::Fragment = unsafe {mem::transmute_copy(&instance)};
        let resources = fragment.get_resources(&mut env);
        let metrics = resources.get_display_metrics(&mut env);
        let height = env.get_field(&metrics, "heightPixels", "I").unwrap().i().unwrap();
        let width = env.get_field(&metrics, "widthPixels", "I").unwrap().i().unwrap();
        // root.render(RenderData {
        //     stack: Default::default(),
        //     persistent_storage: Default::default()
        // });



        android_println!("start");

        let window = instance.get_window(&mut env);
        // let decor = window.get_decor_view(&mut env);
        // let width = decor.get_width(&mut env);
        // let height = decor.get_height(&mut env);
        // window.set_content_view_1(arg0, env);
        let native_root = android2_android::widget::RelativeLayout::new_0(&context, &mut env);
        let native_root_view = unsafe { mem::transmute_copy(&native_root) };
        instance.set_content_view_1(&native_root_view, &mut env);
        ENV.with(|e|*e.borrow_mut() = (&mut env as *mut jni::JNIEnv<'local>)as *mut jni::JNIEnv<'static>);
        android_println!("start render");
        let mut rendered_view = root.render(RenderData {
            stack: Default::default(),
            persistent_storage: Default::default(),
            parent: unsafe { mem::transmute_copy(&native_root) },
            context
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
