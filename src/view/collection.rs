use crate::layout::{ComputableLayout, RenderObject};
use tuplex::IntoArray;

pub trait LayoutCollection {
    fn with_v_tables(&mut self, f: impl FnOnce(&mut [&mut dyn ComputableLayout]));
    fn with_v_tables_ref(&self, f: impl FnOnce(&[&dyn ComputableLayout]));
}
pub trait ViewCollection {
    type RenderOutput: LayoutCollection;
    fn render(&self, data: crate::native::RenderData) -> Self::RenderOutput;
}

macro_rules! impl_collection {
    ($($x:tt $y:tt),+) => {
        impl<$($x:  ComputableLayout),+> LayoutCollection for ($($x),+,) {
            fn with_v_tables(&mut self, f: impl FnOnce(&mut [&mut dyn ComputableLayout])) {
                let mut a = ($(&mut self.$y as &mut dyn ComputableLayout),+,).into_array();
                f(&mut a);
            }
            fn with_v_tables_ref(&self, f: impl FnOnce(&[&dyn ComputableLayout])) {
                let mut a = ($(&self.$y as &dyn ComputableLayout),+,).into_array();
                f(&mut a);
            }
        }
        impl<$($x:  RenderObject),+> ViewCollection for ($($x),+,) {
            type RenderOutput = ($($x::Output),+,);

            fn render(&self, data: crate::native::RenderData) -> Self::RenderOutput {
                ($(self.$y.render(data.clone())),+,)
            }
        }
    };
}

impl_collection!(A 0);
impl_collection!(A 0, B 1);
impl_collection!(A 0, B 1, C 2);
impl_collection!(A 0, B 1, C 2, D 3);
impl_collection!(A 0, B 1, C 2, D 3, E 4);
