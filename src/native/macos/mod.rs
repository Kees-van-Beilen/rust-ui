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
        unsafe { view.setFrameOrigin(to.into()) };
    }

    fn destroy(&mut self) {
        let view = self.ns_view();
        unsafe { view.removeFromSuperview() };
    }
}
impl Into<NSSize> for crate::layout::Size<f64> {
    fn into(self) -> NSSize {
        NSSize {
            width: self.width,
            height: self.height,
        }
    }
}
impl Into<NSPoint> for crate::layout::Position<f64> {
    fn into(self) -> NSPoint {
        NSPoint {
            x: self.x,
            y: self.y,
        }
    }
}
impl From<NSSize> for crate::layout::Size<f64> {
    fn from(value: NSSize) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

pub mod native {
    // use std::rc::Weak;

    //views
    use objc2::{DefinedClass, MainThreadMarker, rc::Retained, runtime::ProtocolObject};
    use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSTextField, NSView};
    use objc2_core_graphics::CGColorCreateSRGB;
    use objc2_foundation::NSString;
    use objc2_quartz_core::CALayer;

    use crate::layout::RenderObject;

    use super::{NSViewRepresentable, app::Delegate, button::RustButton};

    pub struct Text(Retained<NSTextField>);

    pub struct Button(Retained<RustButton>);
    impl RenderObject for crate::views::Button {
        type Output = Button;

        fn render(&self, data: crate::native::RenderData) -> Self::Output {
            let mtm = MainThreadMarker::new().unwrap();
            let view = unsafe {
                let cb = self.callback.replace(Box::new(|| panic!()));
                let view = RustButton::new(mtm, cb);
                let str = NSString::from_str(&self.label);
                view.setStringValue(&str);
                view.sizeToFit();
                println!("render buttototntotno");
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
                let view = NSTextField::new(mtm);
                let str = NSString::from_str(&self.0);
                view.setStringValue(&str);
                view.setEditable(false);
                view.setDrawsBackground(false);
                view.setBordered(false);
                view.setBezeled(false);
                view.sizeToFit();
                data.real_parent.addSubview(&view);
                view
            };
            Text(view)
        }
    }
    impl NSViewRepresentable for Text {
        fn ns_view(&self) -> &NSView {
            &self.0
        }
    }
    impl NSViewRepresentable for Button {
        fn ns_view(&self) -> &NSView {
            &self.0
        }
    }
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
    pub struct RenderData {
        pub real_parent: Retained<NSView>,
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
