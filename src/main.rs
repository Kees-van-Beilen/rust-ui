use std::{cell::RefCell, rc::Rc};

use layout::{ComputableLayout, RenderObject};
use native::RenderData;
use objc2::rc::Retained;
use objc2_app_kit::NSView;
use views::{Button, HStack, Text};

mod layout;
mod native;
mod views;

/*
currently  the  main file contains  an  experiment on how to implement reactive data,
what kind of model to use etc.  Also to find out how to properly detect view mutation
and handle callbacks that reference the view data.
*/

struct MyViewData {
    clicks: i32,
    attached: Option<Rc<RefCell<CustomView>>>,
}

impl RenderObject for Rc<RefCell<MyViewData>> {
    type Output = Rc<RefCell<CustomView>>;

    fn render(&self, data: native::RenderData) -> Self::Output {
        let view = Rc::new(RefCell::new(CustomView {
            children: Box::new(my_view(self.clone()).render(data.clone())),
            layout_size: layout::Size {
                width: 0.0,
                height: 0.0,
            },
            parent: data.real_parent,
        }));
        let mut m = self.borrow_mut();
        if let Some(k) = &mut m.attached {
            k.swap(&view);
            k.set_size(view.borrow().layout_size);
        } else {
            m.attached = Some(view.clone());
        }
        view
    }
}
impl MyViewData {
    pub fn rerender(m_data: &Rc<RefCell<MyViewData>>) {
        let mut data = m_data.borrow_mut();
        if let Some(k) = &mut data.attached {
            let render_data = {
                k.borrow_mut().children.destroy();
                RenderData {
                    real_parent: k.borrow().parent.clone(),
                }
            };
            drop(data);
            let _ = m_data.render(render_data);
        }
    }
}

struct CustomView {
    children: Box<dyn ComputableLayout>,
    parent: Retained<NSView>,
    layout_size: layout::Size<f64>,
}
impl ComputableLayout for Rc<RefCell<CustomView>> {
    fn set_size(&mut self, to: layout::Size<f64>) {
        self.borrow_mut().layout_size = to;
        self.borrow_mut().children.set_size(to);
    }

    fn set_position(&mut self, to: layout::Position<f64>) {
        self.borrow_mut().children.set_position(to);
    }

    fn destroy(&mut self) {
        self.borrow_mut().children.destroy();
    }
}

fn my_view(data: Rc<RefCell<MyViewData>>) -> impl RenderObject {
    let clicks = { data.borrow().clicks };
    HStack {
        spacing: 0.0,
        children: (
            Text(format!("clicked {} times!", clicks)),
            Button {
                label: "Click me".to_string(),
                callback: {
                    let data = data.clone();
                    RefCell::new(Box::new(move || {
                        {
                            let mut r_data = data.borrow_mut();
                            r_data.clicks += 1;
                            println!("clicked {} times", r_data.clicks);
                        }
                        MyViewData::rerender(&data);
                    }))
                },
            },
        ),
    }
}
fn main() {
    let data = Rc::new(RefCell::new(MyViewData {
        clicks: 0,
        attached: None,
    }));

    native::launch_application_with_view(data);
}
