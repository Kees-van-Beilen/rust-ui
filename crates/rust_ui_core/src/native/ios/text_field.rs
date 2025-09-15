use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, rc::Retained};
use objc2_foundation::{NSNotification, NSPoint, NSString};
use objc2_ui_kit::{UIControlEvents, UITextField, UITextFieldDelegate};

use crate::{
    layout::{ComputableLayout, RenderObject},
    view::state::PartialBindingBox,
    views::TextField,
};

pub struct RustTextFieldIVars {
    binding: PartialBindingBox<String>,
}

define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `Delegate` does not implement `Drop`.
    #[unsafe(super = UITextField)]
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
            let text = unsafe { self.text().map(|e|e.to_string()) }.unwrap_or_default();
            self.ivars().binding.update_value(text);
        }

    }

);

impl RustTextField {
    pub unsafe fn new(
        mtm: MainThreadMarker,
        binding: PartialBindingBox<String>,
    ) -> Retained<RustTextField> {
        let this = Self::alloc(mtm).set_ivars(RustTextFieldIVars { binding });
        let s: Retained<RustTextField> = msg_send![super(this), init];
        unsafe {
            s.addTarget_action_forControlEvents(
                Some(&s),
                objc2::sel!(textDidChange:),
                UIControlEvents::EditingChanged,
            )
        };
        s
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

        unsafe {
            if let Some(view) = data
                .persistent_storage
                .borrow()
                .get::<Retained<RustTextField>>(identity)
            {
                // data.real_parent.addSubview(view);
                let ns_view = view.clone();
                let _ = view;
                let str = NSString::from_str(self.text_binding.get().as_str());
                ns_view.setText(Some(&str));
                // ns_view.stringValue().
                NativeTextField { ns_view }
            } else {
                let mtm = MainThreadMarker::new().unwrap();
                // let bo = clone_dyn::clone_into_box(&self.text_binding);
                let ns_view = RustTextField::new(mtm, self.text_binding.clone_box());
                let str = NSString::from_str(self.text_binding.get().as_str());
                ns_view.setText(Some(&str));
                {
                    let ns_view = ns_view.clone();
                    data.persistent_storage
                        .borrow_mut()
                        .register_for_garbage_collection(identity, move || {
                            ns_view.removeFromSuperview();
                        });
                }
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
        let mut frame = view.frame();
        frame.size = to.into();
        view.setFrame(frame);
    }

    fn set_position(&mut self, to: crate::layout::Position<f64>) {
        let view = &self.ns_view;
        let mut frame = view.frame();
        frame.origin = to.into();
        view.setFrame(frame);
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
        // let view = self.ns_view;
        // unsafe { view.removeFromSuperview() };
    }
}
