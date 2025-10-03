use objc2::{AnyThread, MainThreadMarker, rc::Retained};
// use objc2_app_kit::{NSImageScaling, NSImageView, NSScrollerStyle};
use objc2_quartz_core::{CALayer, kCAGravityResizeAspect, kCAGravityResizeAspectFill};
use objc2_ui_kit::{UIScrollView, UIView};

use crate::{
    view::persistent_storage::PersistentStorageRef,
    views::{Axis, ScrollBehavior},
};

// use crate::{layout::Size, native::macos::nsview_setposition, views::{Axis, ScrollBehavior}};

pub struct NativeScrollView<Child: crate::layout::ComputableLayout> {
    ns_view: Retained<UIScrollView>,
    content_view: Retained<UIView>,
    axis: Axis,
    child: Child,
}

pub struct ScrollViewStorage {
    storage: PersistentStorageRef,
    ns_view: Retained<UIScrollView>,
    content_view: Retained<UIView>,
}

impl<T: crate::layout::RenderObject> crate::layout::RenderObject for crate::views::ScrollView<T> {
    type Output = NativeScrollView<T::Output>;

    fn set_identity(mut self, identity: usize) -> Self {
        self.identity = identity;
        self
    }

    fn render(&self, mut data: crate::native::RenderData) -> Self::Output {
        let identity = self.identity;
        let mut bm = data.persistent_storage.borrow_mut();

        let view = bm.get_or_register_gc(identity, || unsafe {
            let mtm = MainThreadMarker::new().unwrap();

            // let view = objc2_app_kit::NSScrollView::new(mtm);
            let view = UIScrollView::new(mtm);
            data.real_parent.addSubview(&view);
            //the document view is a view who's size is the preferred size

            let content_view = UIView::new(mtm);
            data.real_parent = content_view.clone();

            view.addSubview(&content_view);
            (
                ScrollViewStorage {
                    // child: self.child.render(data),
                    ns_view: view.clone(),
                    // axis: self.axis,
                    content_view,
                    storage: Default::default(),
                },
                move || view.removeFromSuperview(),
            )
        });

        let ns_view = view.ns_view.clone();
        let content_view = view.content_view.clone();
        data.real_parent = content_view.clone();
        let storage = view.storage.clone();
        bm.garbage_collection_mark_used(identity);
        drop(bm);
        data.persistent_storage = storage.clone();

        let child = self.child.render(data);

        NativeScrollView {
            ns_view,
            content_view,
            axis: self.axis,
            child,
        }
    }
}

impl<T: crate::layout::ComputableLayout> crate::layout::ComputableLayout for NativeScrollView<T> {
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        // unsafe {
        let mut frame = self.ns_view.frame();
        frame.size = to.into();
        self.ns_view.setFrame(frame);
        // };

        let mut child_size = to;
        if self.axis.x == ScrollBehavior::Scroll || self.axis.y == ScrollBehavior::Scroll {
            let preferred_size = self.child.preferred_size(&to);
            match (self.axis.x, preferred_size.width) {
                (ScrollBehavior::Scroll, Some(width)) if width > to.width => {
                    child_size.width = width
                }
                _ => {}
            }

            match (self.axis.y, preferred_size.height) {
                (ScrollBehavior::Scroll, Some(height)) if height > to.height => {
                    child_size.height = height
                }
                _ => {}
            }
        }

        unsafe {
            // child_size.height += 100.0;
            let mut frame = self.content_view.frame();
            frame.size = child_size.into();
            // println!("RUST_UI SIZE: {:?} {:?}",frame.size,to);

            self.content_view.setFrame(frame);
            self.ns_view.setContentSize(frame.size);
        }
        self.child.set_size(child_size);

        // println!("Scroll View Size {:?}, content size {:?}",to,child_size);
    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        let mut frame = self.ns_view.frame();
        frame.origin = to.into();
        self.ns_view.setFrame(frame);
        // self.content_view.setFrame(frame);
        // nsview_setposition(&self.ns_view, to);
        // unsafe { self.ns_view.setFrameOrigin(to.into()) };
    }

    fn destroy(&mut self) {
        unsafe {
            self.child.destroy();
        };
    }
}
