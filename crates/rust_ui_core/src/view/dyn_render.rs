use clone_dyn::dependency::clone_dyn_meta;

use crate::{
    layout::{ComputableLayout, Position, RenderObject, Size},
    native::RenderData,
};

pub struct DynGroup(Box<dyn DynRender>);
pub struct DynRendered(Box<dyn ComputableLayout>);

#[ clone_dyn_meta::clone_dyn ]
pub trait DynRender {
    fn render_dyn(&self, data: RenderData) -> Box<dyn ComputableLayout>;
}
impl<T: RenderObject+Clone> DynRender for T
where
    <T as RenderObject>::Output: 'static,
{
    fn render_dyn(&self, data: RenderData) -> Box<dyn ComputableLayout> {
        Box::new(self.render(data))
    }
}
impl DynGroup {
    pub fn new<T: RenderObject+Clone + 'static>(obj: T) -> Self {
        DynGroup(Box::new(obj))
    }
    pub fn cloned(&self)->Self{
        let b = clone_dyn::clone(&self.0);
        Self(b)
        
    }
}

pub struct DynInstance;

impl DynInstance {
    pub fn new(obj: &DynGroup) -> DynGroup {
        obj.cloned()
    }
}

impl ComputableLayout for DynRendered {
    fn preferred_size(&self, in_frame: &Size<f64>) -> Size<Option<f64>> {
        self.0.preferred_size(in_frame)
    }
    fn v_tables(&self) -> &[&dyn ComputableLayout] {
        self.0.v_tables()
    }
    fn v_tables_len(&self) -> usize {
        self.0.v_tables_len()
    }
    fn v_tables_mut(&mut self) -> &mut [&mut dyn ComputableLayout] {
        self.0.v_tables_mut()
    }
    fn set_size(&mut self, to: Size<f64>) {
        self.0.set_size(to);
    }

    fn set_position(&mut self, to: Position<f64>) {
        self.0.set_position(to);
    }

    fn destroy(&mut self) {
        self.0.destroy();
    }
}

impl RenderObject for DynGroup {
    type Output = DynRendered;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        DynRendered(self.0.render_dyn(data))
    }
}
