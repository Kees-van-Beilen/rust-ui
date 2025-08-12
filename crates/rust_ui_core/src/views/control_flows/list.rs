use crate::layout::{ComputableLayout, RenderObject};

/*

HStack {
    for item in  0..10 {
        Text(format!("item number {item}"))
    }
}

<..>::new(<..>{
    ListView::new({
        let mut collection = Vec::new();
        for item in 0..10 {
            collection.push({
                Text::new(format!("item number {item}"))
            })
        }
        collection
    })
    ..Default::default()
})

*/

pub struct ListView<A:RenderObject> {
    elements:Vec<A>
}

impl<A:RenderObject> ListView<A> {
    pub fn new(elements:Vec<A>)->Self{
        Self {
            elements,
        }
    }
}
pub struct RenderedListView<A:ComputableLayout> {
    elements:Vec<A>
}

impl<A:ComputableLayout> ComputableLayout for RenderedListView<A> {
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        todo!()
    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        todo!()
    }

    fn destroy(&mut self) {
        todo!()
    }
    fn write_v_tables<'a,'b>(&'a self,buf:&'b mut Vec<&'a dyn ComputableLayout>) {
        buf.extend(self.elements.iter().map(|e|e as & dyn ComputableLayout));
    }
    fn write_v_tables_mut<'a,'b>(&'a mut self,buf:&'b mut Vec<&'a mut dyn ComputableLayout>) {
        buf.extend(self.elements.iter_mut().map(|e|e as &mut dyn ComputableLayout));
    }
    fn v_tables_len(&self) -> usize {
        self.elements.len()
    }
}

impl <A:RenderObject> RenderObject for ListView<A> {
    type Output=RenderedListView<A::Output>;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        RenderedListView{
            elements: self.elements.iter().map(|e|e.render(data.clone())).collect(),
        }
    }
}