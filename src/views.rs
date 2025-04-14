//builtin views

use std::cell::RefCell;

use crate::layout::{ComputableLayout, Position, RenderObject, Size};

//This impl is temporary we need to write a macro for this all.
pub trait KZTupleSlice<T> {
    type Output: AsRef<[T]>;
    fn as_array(self) -> Self::Output;
}

impl<T: Sized> KZTupleSlice<T> for (T,) {
    type Output = [T; 1];

    fn as_array(self) -> Self::Output {
        [self.0]
    }
}
impl<T: Sized> KZTupleSlice<T> for (T, T) {
    type Output = [T; 2];

    fn as_array(self) -> Self::Output {
        [self.0, self.1]
    }
}

pub struct Text(pub String);
pub struct Button {
    pub label: String,
    pub callback: RefCell<Box<dyn Fn()>>,
}
//this becomes a native view
pub struct ColorView(pub bevy_color::Color);
// pub struct ColorView(pub bevy_color::Color);

pub trait LayoutCollection {
    fn with_v_tables(&mut self, f: impl FnOnce(&mut [&mut dyn ComputableLayout]));
}
pub trait ViewCollection {
    type RenderOutput: LayoutCollection;
    fn render(&self, data: crate::native::RenderData) -> Self::RenderOutput;
}
impl<A: RenderObject> ViewCollection for (A,) {
    type RenderOutput = (A::Output,);

    fn render(&self, data: crate::native::RenderData) -> Self::RenderOutput {
        (self.0.render(data),)
    }
}
impl<A: RenderObject, B: RenderObject> ViewCollection for (A, B) {
    type RenderOutput = (A::Output, B::Output);

    fn render(&self, data: crate::native::RenderData) -> Self::RenderOutput {
        (self.0.render(data.clone()), self.1.render(data))
    }
}

impl<A: ComputableLayout> LayoutCollection for (A,) {
    fn with_v_tables(&mut self, f: impl FnOnce(&mut [&mut dyn ComputableLayout])) {
        let mut a = (&mut self.0 as &mut dyn ComputableLayout,).as_array();
        f(&mut a);
    }
}
impl<A: ComputableLayout, B: ComputableLayout> LayoutCollection for (A, B) {
    fn with_v_tables(&mut self, f: impl FnOnce(&mut [&mut dyn ComputableLayout])) {
        let mut a = (
            &mut self.0 as &mut dyn ComputableLayout,
            &mut self.1 as &mut dyn ComputableLayout,
        )
            .as_array();
        f(&mut a);
    }
}

//this doesn't become a native view
pub struct HStack<T: ViewCollection> {
    pub spacing: f64,
    pub children: T,
}
pub struct RenderedHStack<T: LayoutCollection> {
    spacing: f64,
    children: T,
}


impl<T: ViewCollection> RenderObject for HStack<T> {
    type Output = RenderedHStack<T::RenderOutput>;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        RenderedHStack {
            spacing: self.spacing,
            children: self.children.render(data),
        }
    }
}

impl<T: LayoutCollection> ComputableLayout for RenderedHStack<T> {
    fn set_size(&mut self, to: crate::layout::Size<f64>) {
        self.children.with_v_tables(|e| {
            let len = e.len();
            let space = self.spacing * 0.5;
            let width = to.width / len as f64 - space;
            let mut pos = Position { x: 0.0, y: 0.0 };
            for i in e {
                (*i).set_size(Size {
                    width,
                    height: to.height,
                });
                (*i).set_position(pos);
                pos.x += width + self.spacing;
            }
        });
    }

    fn set_position(&mut self, _to: crate::layout::Position<f64>) {
        todo!()
    }

    fn destroy(&mut self) {
        self.children.with_v_tables(|tables| {
            for child in tables {
                child.destroy();
            }
        });
    }
}
