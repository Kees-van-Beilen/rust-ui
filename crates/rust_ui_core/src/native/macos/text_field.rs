use std::{cell::RefCell, os::raw::c_void, ptr::NonNull};

use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, rc::Retained};
use objc2_app_kit::{NSTextField, NSView};
use objc2_foundation::{NSComparisonResult, NSNotification, NSPoint, NSString};

use crate::{
    layout::{ComputableLayout, RenderObject}, native::macos::{get_foreground_color, order_view_in_front}, view::state::PartialBindingBox, views::TextField
};

pub struct RustTextFieldIVars {
    binding: RefCell<Option<PartialBindingBox<String>>>,
}

define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `Delegate` does not implement `Drop`.
    #[unsafe(super = NSTextField)]
    #[thread_kind = MainThreadOnly]
    #[name = "RustTextField"]
    #[ivars = RustTextFieldIVars]
    pub struct RustTextField;

    // SAFETY: `NSObjectProtocol` has no safety requirements.
    // unsafe impl NSObjectProtocol for Delegate {}
    // unsafe impl NSControlTextEditingDelegate for Delegate {}

    // SAFETY: `NSApplicationDelegate` has no safety requirements.
    impl RustTextField {
        // SAFETY: The signature is correct.
        #[unsafe(method(textDidChange:))]
        fn text_change(&self, _notification: &NSNotification) {
            let text = unsafe { self.stringValue() }.to_string();
            println!("text change: {:?}",text);
            if let Some(e) = self.ivars().binding.replace(None) {
                e.update_value(text);
            }

            // self.ivars().binding
            // self.ivars().binding
        }

    }

);

impl RustTextField {
    pub unsafe fn new(
        mtm: MainThreadMarker,
        binding: PartialBindingBox<String>,
    ) -> Retained<RustTextField> {
        let this = Self::alloc(mtm).set_ivars(RustTextFieldIVars { binding:RefCell::new(Some(binding)) });
        msg_send![super(this), init]
        // t.ivars().binding.swap(&RefCell::new(Some(binding)));
    }
}

pub struct NativeTextField {
    ns_view: Retained<RustTextField>,
}

impl RenderObject for TextField {
    type Output = NativeTextField;

    fn set_identity(mut self, identity: usize) -> Self {
        self.identity = Some(identity);
        self
    }

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        let identity = self
            .identity
            .expect("forgot to call set_identity on TextField");

        data.persistent_storage
            .borrow_mut()
            .garbage_collection_mark_used(identity);

        // let new_binding: *const () = &&*self.text_binding as * const _ as *const ();
        // println!("textfield box: {:?}",ptr);
        
        unsafe {
            if let Some(view) = data
            .persistent_storage
            .borrow()
            .get::<Retained<RustTextField>>(identity)
            {
                // let binding: *const () = &&*self.text_binding as * const _ as *const ();
                // data.real_parent.addSubview(view);
                let ns_view = view.clone();
                let color = get_foreground_color(&data.stack);
                // ns_view.setTextColor(Some(&color));
                let _ = view;
                let binding = self.text_binding.get();
                let binding_str = binding.as_str();
                ns_view.ivars().binding.replace(Some(self.text_binding.clone_box()));
                order_view_in_front(&ns_view);
                
                // let str = NSString::from_str(binding_str);
                // println!("text binding value: {}",&binding_str);
                // ns_view.setStringValue(&str);
                // ns_view.stringValue().
                NativeTextField { ns_view }
            } else {
                let mtm = MainThreadMarker::new().unwrap();
                // let bo = clone_dyn::clone_into_box(&self.text_binding);
                let color = get_foreground_color(&data.stack);
                let ns_view = RustTextField::new(mtm, self.text_binding.clone_box());
                ns_view.setTextColor(Some(&color));
                ns_view.setMaximumNumberOfLines(0);
                let str = NSString::from_str(self.text_binding.get().as_str());
                ns_view.setStringValue(&str);
                {
                    let ns_view = ns_view.clone();
                    data.persistent_storage
                        .borrow_mut()
                        .register_for_garbage_collection(identity, move || {
                            ns_view.removeFromSuperview();
                            println!("destroyed nstextfield");
                        });
                }
                println!("created nstextfield");
                data.real_parent.addSubview(&ns_view);
                data.persistent_storage
                    .borrow_mut()
                    .insert(identity, ns_view.clone());
                NativeTextField { ns_view }
            }
        }
    }
}

impl ComputableLayout for NativeTextField {
    fn set_size(&mut self, to: crate::layout::Size<f64>) {
        let view = &self.ns_view;
        unsafe { view.setFrameSize(to.into()) };
    }

    fn set_position(&mut self, to: crate::layout::Position<f64>) {
        let view = &self.ns_view;
        let Some(super_view) = (unsafe { view.superview() }) else {
            return;
        };
        let y = super_view.frame().size.height - to.y - view.frame().size.height;

        unsafe { view.setFrameOrigin(NSPoint { x: to.x, y: y }) };
    }
    fn preferred_size(
        &self,
        in_frame: &crate::prelude::Size<f64>,
    ) -> crate::prelude::Size<Option<f64>> {
        let mut s: crate::prelude::Size<Option<f64>> =
            unsafe { self.ns_view.sizeThatFits((*in_frame).into()) }.into();
        s.width = None;
        s
    }

    fn destroy(&mut self) {
        println!("should have been destroyed");
        // let view = self.ns_view;
        // unsafe { view.removeFromSuperview() };
    }
}
