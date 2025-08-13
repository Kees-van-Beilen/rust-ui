use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::NSScrollerStyle;

use crate::{
    native::macos::nsview_setposition,
    views::{Axis, ScrollBehavior},
};

pub struct NativeScrollView<Child: crate::layout::ComputableLayout> {
    ns_view: Retained<objc2_app_kit::NSScrollView>,
    content_view: Retained<objc2_app_kit::NSView>,
    axis: Axis,
    child: Child,
}

impl<T: crate::layout::RenderObject> crate::layout::RenderObject for crate::views::ScrollView<T> {
    type Output = NativeScrollView<T::Output>;

    fn render(&self, mut data: crate::native::RenderData) -> Self::Output {
        unsafe {
            let mtm = MainThreadMarker::new().unwrap();

            let view = objc2_app_kit::NSScrollView::new(mtm);
            view.setDrawsBackground(false);
            view.setScrollerStyle(NSScrollerStyle::Overlay);
            if self.axis.y == ScrollBehavior::Scroll {
                view.setHasVerticalScroller(true);
            }
            if self.axis.x == ScrollBehavior::Scroll {
                view.setHasHorizontalScroller(true);
            }
            data.real_parent.addSubview(&view);
            //the document view is a view who's size is the preferred size

            let content_view = objc2_app_kit::NSView::new(mtm);
            data.real_parent = content_view.clone();

            view.setDocumentView(Some(&content_view));

            NativeScrollView {
                child: self.child.render(data),
                ns_view: view,
                axis: self.axis,
                content_view,
            }
        }
    }
}

impl<T: crate::layout::ComputableLayout> crate::layout::ComputableLayout for NativeScrollView<T> {
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        unsafe { self.ns_view.setFrameSize(to.into()) };

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

        unsafe { self.content_view.setFrameSize(child_size.into()) }
        self.child.set_size(child_size);
    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        nsview_setposition(&self.ns_view, to);
    }

    fn destroy(&mut self) {
        unsafe { self.ns_view.removeFromSuperview() };
    }
}
