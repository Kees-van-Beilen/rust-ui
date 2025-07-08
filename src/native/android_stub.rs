pub mod native {
    use crate::view::resources::Resource;
    #[derive(Clone)]
    pub struct RenderData { }
    impl RenderData {
        pub fn ament_with<T: Resource, F, K>(&mut self, element: T, with_fn: F) -> K
        where for<'b> F: FnOnce(RenderData) -> K,
        {
            todo!()
        }
    }
    pub struct MutableView {
    }
}
