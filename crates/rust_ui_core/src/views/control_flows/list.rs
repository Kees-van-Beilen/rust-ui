//! The implementation of the rust-ui for loop
use crate::layout::{ComputableLayout, RenderObject};

/// The list view here is unhappily named, is does not create a list
/// instead it represents a collection of children and places the in 
/// the hierarchy as if this view didn't exist
/// Example:
/// ```text
/// HStack
///  - Text
///  - ListView
///    - Text
///    - Button
/// ```
/// Becomes
/// ```text
/// HStack
///  - Text
///  - Text
///  - Button
/// ```
pub struct ListView<A: RenderObject> {
    elements: Vec<A>,
}

impl<A: RenderObject> ListView<A> {
    /// construct a new list
    pub fn new(elements: Vec<A>) -> Self {
        Self { elements }
    }
}


/// rendered variant of list
pub struct RenderedListView<A: ComputableLayout> {
    elements: Vec<A>,
}

impl<A: ComputableLayout> ComputableLayout for RenderedListView<A> {
    fn set_size(&mut self, _to: crate::prelude::Size<f64>) {
        //this method should not be called.
    }

    fn set_position(&mut self, _to: crate::prelude::Position<f64>) {
        //this method should not be called.
    }

    fn destroy(&mut self) {
        for child in self.elements.iter_mut() {
            child.destroy();
        }
    }
    fn write_v_tables<'a, 'b>(&'a self, buf: &'b mut Vec<&'a dyn ComputableLayout>) {
        buf.extend(self.elements.iter().map(|e| e as &dyn ComputableLayout));
    }
    fn write_v_tables_mut<'a, 'b>(&'a mut self, buf: &'b mut Vec<&'a mut dyn ComputableLayout>) {
        buf.extend(
            self.elements
                .iter_mut()
                .map(|e| e as &mut dyn ComputableLayout),
        );
    }
    fn v_tables_len(&self) -> usize {
        self.elements.len()
    }
}

impl<A: RenderObject> RenderObject for ListView<A> {
    type Output = RenderedListView<A::Output>;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        RenderedListView {
            elements: self
                .elements
                .iter()
                .map(|e| e.render(data.clone()))
                .collect(),
        }
    }
}
