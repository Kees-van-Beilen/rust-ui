use std::{cell::RefCell, rc::Rc};

use rust_ui::{
    prelude::*,
    view::mutable::{MutableView, MutableViewRerender},
};

/*
In this example we are going to make the same counter as in the counter example
however this time the button will be wrapped in a custom view. That way we can
test the idea of reactive `bindings`.

Notice that without macros this code looks really ugly.

*/

pub struct MyButtonView {
    counter: Rc<RefCell<MyView>>,
    view: Option<std::rc::Rc<std::cell::RefCell<rust_ui::native::MutableView>>>,
}
impl MutableView for MyButtonView {
    fn children(data: Rc<RefCell<Self>>) -> impl layout::RenderObject + 'static {
        Button {
            label: "Click Me".to_string(),
            callback: {
                let data_r = data.clone();
                RefCell::new(Box::new(move || {
                    let data = data_r.borrow_mut();
                    {
                        data.counter.borrow_mut().counter += 1;
                        data.counter.rerender();
                    }
                }))
            },
        }
    }

    fn get_attached(&self) -> &Option<Rc<RefCell<rust_ui::native::MutableView>>> {
        &self.view
    }

    fn get_mut_attached(&mut self) -> &mut Option<Rc<RefCell<rust_ui::native::MutableView>>> {
        &mut self.view
    }
}

#[derive(Default)]
pub struct MyView {
    counter: i32,
    view: Option<std::rc::Rc<std::cell::RefCell<rust_ui::native::MutableView>>>,
}

impl MutableView for MyView {
    fn children(
        data: std::rc::Rc<std::cell::RefCell<Self>>,
    ) -> impl layout::RenderObject + 'static {
        HStack {
            spacing: 0.0,
            children: (
                Spacer,
                Rc::new(RefCell::new(MyButtonView {
                    counter: data.clone(),
                    view: None,
                })),
                Text {
                    content: format!("clicked {} time(s)", data.borrow().counter),
                },
                Spacer,
            ),
        }
    }

    fn get_attached(
        &self,
    ) -> &Option<std::rc::Rc<std::cell::RefCell<rust_ui::native::MutableView>>> {
        &self.view
    }

    fn get_mut_attached(
        &mut self,
    ) -> &mut Option<std::rc::Rc<std::cell::RefCell<rust_ui::native::MutableView>>> {
        &mut self.view
    }
}

fn main() {
    rust_ui::native::launch_application_with_view(Rc::new(RefCell::new(MyView::default())));
}
