use std::{cell::RefCell, rc::Rc};

use rust_ui::{
    prelude::*,
    view::mutable::{MutableView, MutableViewRerender},
};

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
                Button {
                    label: "Click Me".to_string(),
                    callback: {
                        let data_r = data.clone();
                        RefCell::new(Box::new(move || {
                            {
                                let mut data = data_r.borrow_mut();
                                data.counter += 1;
                            }
                            data_r.rerender();
                        }))
                    },
                },
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
