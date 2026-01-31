//! this crate provides a view that may or may not exists. Mimicking the if-loop control flow
//! This is currently removed.

// use crate::layout::ComputableLayout;

// pub struct RenderedOption {

// }

// impl<A:ComputableLayout> ComputableLayout for Option<A> {

//     fn set_size(&mut self, to: crate::prelude::Size<f64>) {
//         panic!("cannot call set_size on a container")
//     }

//     fn set_position(&mut self, to: crate::prelude::Position<f64>) {
//         panic!("cannot call set_size on a container")
//     }

//     fn destroy(&mut self) {
//         if let Some(a) = self {
//             a.destroy();
//         }
//     }

//     fn v_tables_mut<'a>(&'a mut self) -> &'a mut [&mut dyn ComputableLayout] {
//         //the default is to not do so, as most layouts do not contain dynamic
//         if let Some(inner) = self {
//             let d:&mut dyn ComputableLayout = inner;
//             &mut [d]
//         }else{
//             &mut []
//         }
//     }

//     fn v_tables(&self) -> &[&dyn ComputableLayout] {
//         //the default is to not do so, as most layouts do not contain dynamic
//         &[]
//     }

//     fn v_tables_len(&self) -> usize {
//         0
//     }

//     fn preferred_size(&self, _in_frame: &crate::prelude::Size<f64>) -> crate::prelude::Size<Option<f64>> {
//         crate::prelude::Size::splat(None)
//     }

//     fn min_size(&self, _in_frame: &crate::prelude::Size<f64>) -> crate::prelude::Size<Option<f64>> {
//         crate::prelude::Size::splat(None)
//     }

//     fn max_size(&self, _in_frame: &crate::prelude::Size<f64>) -> crate::prelude::Size<Option<f64>> {
//         crate::prelude::Size::splat(None)
//     }
// }
