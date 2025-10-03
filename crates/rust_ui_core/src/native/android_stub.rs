pub mod native {
    use crate::view::resources::Resource;
    #[derive(Clone)]
    pub struct RenderData {}
    impl RenderData {
        pub fn ament_with<T: Resource, F, K>(&mut self, element: T, with_fn: F) -> K
        where
            for<'b> F: FnOnce(RenderData) -> K,
        {
            todo!()
        }
    }
    pub struct MutableView {}

    impl<V> crate::view::mutable::MutableViewRerender for ::std::rc::Rc<::std::cell::RefCell<V>> {
        fn rerender(&self) {
            todo!()
        }

        fn enqueue_change(&self) {
            todo!()
        }

        fn flush_changes(&self) {
            todo!()
        }
    }

    pub struct NativeImageHandle {}

    impl NativeImageHandle {
        pub fn from_path(path: impl ToString) -> Self {
            todo!()
        }
    }
}
