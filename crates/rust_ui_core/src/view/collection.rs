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
impl_collection!(A 0, B 1, C 2, D 3, E 4, F 5);
impl_collection!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
impl_collection!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
impl_collection!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8);
impl_collection!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9);
impl_collection!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10);
impl_collection!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11);
impl_collection!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12);
impl_collection!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13);
impl_collection!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14);
impl_collection!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15);