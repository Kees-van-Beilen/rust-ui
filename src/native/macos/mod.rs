//a simple native view_wrapper

mod app;
mod button;

use crate::layout::ComputableLayout;
use objc2_app_kit::NSView;
use objc2_foundation::{NSPoint, NSSize};

//all nsview reps auto participate in the layout manager
//todo: this file should be split up
trait NSViewRepresentable {
    fn ns_view(&self) -> &NSView;
}
impl<T: NSViewRepresentable> ComputableLayout for T {
    fn set_size(&mut self, to: crate::layout::Size<f64>) {
        let view = self.ns_view();
        unsafe { view.setFrameSize(to.into()) };
    }

    fn set_position(&mut self, to: crate::layout::Position<f64>) {
        let view = self.ns_view();
        let y = unsafe { view.superview() }.unwrap().frame().size.height
            - to.y
            - view.frame().size.height;

        unsafe { view.setFrameOrigin(NSPoint { x: to.x, y: y }) };
    }

    fn destroy(&mut self) {
        let view = self.ns_view();
        unsafe { view.removeFromSuperview() };
    }
}

pub mod native {
    // use std::rc::Weak;

    use std::{borrow::Cow, cell::RefCell, rc::Rc};

    //views
    use objc2::{DefinedClass, MainThreadMarker, rc::Retained, runtime::ProtocolObject};
    use objc2_app_kit::{
        NSApplication, NSApplicationActivationPolicy, NSBezelStyle, NSFontWeight,
        NSFontWeightBlack, NSFontWeightBold, NSFontWeightHeavy, NSFontWeightLight,
        NSFontWeightMedium, NSFontWeightRegular, NSFontWeightSemibold, NSFontWeightThin,
        NSFontWeightUltraLight, NSTextAlignment, NSTextField, NSView,
    };
    use objc2_core_graphics::CGColorCreateSRGB;
    use objc2_foundation::{NSPoint, NSString};
    use objc2_quartz_core::CALayer;

    use crate::{
        layout::{self, ComputableLayout, Position, RenderObject, Size},
        view::{
            mutable::MutableViewRerender,
            resources::{Resource, ResourceStack},
        },
        views::{FontFamily, FontSize, FontWeight},
    };

    use super::{NSViewRepresentable, app::Delegate, button::RustButton};

    pub struct MutableView {
        children: Box<dyn ComputableLayout>,
        parent: Retained<NSView>,
        layout_size: layout::Size<f64>,
        stack: crate::view::resources::Resources,
    }
    impl<T: crate::view::mutable::MutableView> MutableViewRerender for Rc<RefCell<T>> {
        fn rerender(&self) {
            //This entire rerender logic is a piece of shit
            //the entire idea of these mutable views have to
            //be redesigned.
            let mut data = self.borrow_mut();
            if let Some(k) = &mut data.get_mut_attached() {
                let render_data = {
                    let mut b = k.borrow_mut();
                    b.children.destroy();
                    let render_data = RenderData {
                        real_parent: b.parent.clone(),
                        //TODO: fix this clone to a ref
                        stack: crate::view::resources::ResourceStack::Owned(b.stack.clone()),
                    };
                    render_data
                };
                drop(data);
                let _ = self.render(render_data);
            }
        }
    }
    impl<T: crate::view::mutable::MutableView> RenderObject for Rc<RefCell<T>> {
        type Output = Rc<RefCell<crate::native::MutableView>>;

        fn render(&self, data: crate::native::RenderData) -> Self::Output {
            let r = T::children(self.clone()).render(data.clone());
            let view = Rc::new(RefCell::new(MutableView {
                children: Box::new(r),
                layout_size: layout::Size {
                    width: 0.0,
                    height: 0.0,
                },
                parent: data.real_parent,
                stack: data.stack.as_ref().clone(),
            }));
            let mut m = self.borrow_mut();
            let mut attached = m.get_mut_attached();
            if let Some(k) = &mut attached {
                k.swap(&view);
                k.set_size(view.borrow().layout_size);
                k.set_position(Position { x: 0.0, y: 0.0 });
            } else {
                *attached = Some(view.clone());
            }
            view
        }
    }

    impl ComputableLayout for Rc<RefCell<MutableView>> {
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

    pub struct Text {
        nsview: Retained<NSTextField>,
        size: Size<f64>,
        position: Position<f64>,
    }

    fn font_weight_to_ns_font_weight(weight: FontWeight) -> NSFontWeight {
        unsafe {
            match weight {
                FontWeight::Ultralight => NSFontWeightUltraLight,
                FontWeight::Thin => NSFontWeightThin,
                FontWeight::Light => NSFontWeightLight,
                FontWeight::Regular => NSFontWeightRegular,
                FontWeight::Medium => NSFontWeightMedium,
                FontWeight::Semibold => NSFontWeightSemibold,
                FontWeight::Bold => NSFontWeightBold,
                FontWeight::Heavy => NSFontWeightHeavy,
                FontWeight::Black => NSFontWeightBlack,
            }
        }
    }

    pub struct Button(Retained<RustButton>);
    impl RenderObject for crate::views::Button {
        type Output = Button;

        fn render(&self, data: crate::native::RenderData) -> Self::Output {
            let mtm = MainThreadMarker::new().unwrap();
            let view = unsafe {
                let cb = self.callback.replace(Box::new(|| panic!()));
                let view = RustButton::new(mtm, cb);
                let str = NSString::from_str(&self.label);
                // view.setStringValue(&str);
                view.setTitle(&str);
                // view.setBezelStyle(NSBezelStyle::Te);
                view.sizeToFit();
                data.real_parent.addSubview(&view);
                view
            };
            Button(view)
        }
    }

    impl RenderObject for crate::views::Text {
        type Output = Text;

        fn render(&self, data: crate::native::RenderData) -> Self::Output {
            let mtm = MainThreadMarker::new().unwrap();
            let view = unsafe {
                use objc2_app_kit::NSFont;
                let view = NSTextField::new(mtm);
                let str = NSString::from_str(&self.content);
                view.setStringValue(&str);
                // view.setFont(font);
                let font_family = data
                    .stack
                    .get_resource::<FontFamily>()
                    .unwrap_or(&crate::views::FontFamily::SystemUI);
                let font_size = data
                    .stack
                    .get_resource::<FontSize>()
                    .copied()
                    .unwrap_or(FontSize(NSFont::systemFontSize()));
                let font_weight = data
                    .stack
                    .get_resource::<FontWeight>()
                    .copied()
                    .unwrap_or(FontWeight::Regular);
                match font_family {
                    FontFamily::SystemUI => {
                        let font = NSFont::systemFontOfSize_weight(
                            font_size.0,
                            font_weight_to_ns_font_weight(font_weight),
                        );
                        view.setFont(Some(&font));
                    }
                    FontFamily::Custom(_) => todo!(),
                }
                // NSFontWeightRegular
                // objc2_app_kit::NSFont::
                view.setEditable(false);
                view.setDrawsBackground(false);
                view.setBordered(false);
                view.setBezeled(false);
                view.sizeToFit();
                // NSFontWeightBlack
                // objc2_app_kit::NSFont::systemFontOfSize_weight(font_size, weight)
                // view.setFont(font);
                view.setAlignment(NSTextAlignment::Center);
                data.real_parent.addSubview(&view);
                view
            };
            Text {
                nsview: view,
                size: Default::default(),
                position: Default::default(),
            }
        }
    }
    // Text uses its own layout computation
    // to center vertically
    impl Text {
        fn do_layout(&self) {
            let to = self.size;
            let size = unsafe { self.nsview.sizeThatFits(to.into()) };

            let y = (self.size.height - size.height) * 0.5 + self.position.y;
            let x = (to.width - size.width) * 0.5 + self.position.x;
            // println!("real y {y}");
            let y = unsafe { self.nsview.superview() }
                .unwrap()
                .frame()
                .size
                .height
                - y
                - size.height;
            // println!("real y {y}");
            unsafe { self.nsview.setFrameOrigin(NSPoint { x, y }) };
        }
    }
    impl ComputableLayout for Text {
        fn set_size(&mut self, to: crate::layout::Size<f64>) {
            self.size = to;
            // unsafe { self.nsview.setFrameSize(to.into()) };
            let size = unsafe { self.nsview.sizeThatFits(to.into()) };
            unsafe { self.nsview.setFrameSize(size) };
            self.do_layout();
            //now position the view
        }

        fn set_position(&mut self, to: crate::layout::Position<f64>) {
            self.position = to;
            self.do_layout();
            // unsafe { view.setFrameOrigin(to.into()) };
        }

        fn destroy(&mut self) {
            unsafe { self.nsview.removeFromSuperview() };
        }
        fn preferred_size(&self, in_frame: &Size<f64>) -> Option<Size<f64>> {
            let size = unsafe { self.nsview.sizeThatFits((*in_frame).into()) };
            Some(size.into())
        }
    }
    impl ComputableLayout for Button {
        fn preferred_size(&self, in_frame: &Size<f64>) -> Option<Size<f64>> {
            let mut size = unsafe { self.0.sizeThatFits((*in_frame).into()) };
            //small adjustment to the layout otherwise the button over/under flows
            size.width += 5.0;
            size.height += 5.0;
            Some(size.into())
        }
        fn set_size(&mut self, to: crate::layout::Size<f64>) {
            let view = &self.0;
            unsafe { view.setFrameSize(to.into()) };
        }

        fn set_position(&mut self, to: crate::layout::Position<f64>) {
            let view = &self.0;
            let y = unsafe { view.superview() }.unwrap().frame().size.height
                - to.y
                - view.frame().size.height;

            unsafe { view.setFrameOrigin(NSPoint { x: to.x, y: y }) };
        }

        fn destroy(&mut self) {
            let view = &self.0;
            unsafe { view.removeFromSuperview() };
        }
    }
    // impl NSViewRepresentable for Text {
    //     fn ns_view(&self) -> &NSView {
    //         &self.0
    //     }
    // }
    // impl NSViewRepresentable for Button {
    //     fn ns_view(&self) -> &NSView {
    //         &self.0
    //     }
    // }
    pub struct ColorView(Retained<NSView>);
    impl RenderObject for crate::views::ColorView {
        type Output = ColorView;

        fn render(&self, data: crate::native::RenderData) -> Self::Output {
            let v = self.0.to_srgba();
            let mtm = MainThreadMarker::new().unwrap();
            let view = unsafe {
                let view = NSView::new(mtm);
                let color =
                    CGColorCreateSRGB(v.red as f64, v.green as f64, v.blue as f64, v.alpha as f64);
                let layer = CALayer::layer();
                layer.setBackgroundColor(Some(&color));
                view.setLayer(Some(&layer));
                data.real_parent.addSubview(&view);
                view
            };
            ColorView(view)
        }
    }
    impl NSViewRepresentable for ColorView {
        fn ns_view(&self) -> &NSView {
            &self.0
        }
    }
    #[derive(Clone)]
    pub struct RenderData<'a> {
        pub real_parent: Retained<NSView>,
        pub stack: crate::view::resources::ResourceStack<'a>,
    }
    impl<'a> RenderData<'a> {
        pub fn ament_with<T: Resource, F, K>(&mut self, element: T, with_fn: F) -> K
        where
            for<'b> F: FnOnce(RenderData) -> K,
        {
            self.stack.amend_with(element, |stack_e| {
                let d = RenderData {
                    real_parent: self.real_parent.clone(),
                    stack: ResourceStack::Borrow(stack_e),
                };

                with_fn(d)
            })
        }
    }

    pub fn launch_application_with_view(root: impl RenderObject + 'static) {
        let mtm = MainThreadMarker::new().unwrap();

        let app = NSApplication::sharedApplication(mtm);
        let delegate = Delegate::new(mtm);
        app.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
        app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
        delegate
            .ivars()
            .signal
            .set(Some(Box::new(|del: &Delegate| del.render(root))));
        app.run();
    }
}
