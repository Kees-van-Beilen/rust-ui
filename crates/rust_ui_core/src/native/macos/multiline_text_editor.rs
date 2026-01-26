use std::cell::RefCell;

use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_app_kit::{NSTextDelegate, NSTextView, NSTextViewDelegate};
use objc2_foundation::{NSNotification, NSObject, NSObjectProtocol, ns_string};

use crate::{layout::{ComputableLayout, RenderObject}, view::state::{PartialAnyBinding, PartialBindingBox}, views::textfield::TextEditor};

pub struct RustTextViewIVars {
    binding:RefCell<PartialBindingBox<String>>
}

define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `Delegate` does not implement `Drop`.
    #[unsafe(super = NSTextView)]
    #[thread_kind = MainThreadOnly]
    #[name = "RustTextView"]
    #[ivars = RustTextViewIVars]
    pub struct RustTextView;

    // unsafe impl NSObjectProtocol for RustTextViewDelegate {
        
    // }
    impl RustTextView {
        #[unsafe(method(didChangeText))]
        fn did_change_text(&self){
            unsafe {
                let _:() = msg_send![super(self), didChangeText];
            }
            // let mtm = self.mtm();
            let vars = self.ivars();
            // println!("text did change");

            unsafe {
                // let obj = notification.object().unwrap();
                let str:Retained<objc2_foundation::NSString> = self.string();
                let rust_str = str.to_string();
                println!("changed text to: {}",&rust_str);
                vars.binding.borrow_mut().update_value(rust_str);
            }
            // (vars.binding.borrow_mut()).update_value(value);

        }
    }
    // unsafe impl NSTextViewDelegate for RustTextViewDelegate {

    // }
);

impl RustTextView {
    pub unsafe fn new(
        mtm: MainThreadMarker,
        binding: PartialBindingBox<String>,
    ) -> Retained<RustTextView> {
        let this = Self::alloc(mtm).set_ivars(RustTextViewIVars { binding:RefCell::new(binding) });
        msg_send![super(this), init]
        // t.ivars().binding.swap(&RefCell::new(Some(binding)));
    }
}


impl RenderObject for TextEditor {
    type Output = NativeTextEditor;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {


        let identity = self.identity.expect("forgot to set identity on TextEditor");
        let view = data.persistent_storage.borrow_mut().get_or_register_gc(identity, ||{
            let binding = self.text_binding.clone_box();
            let mtm = MainThreadMarker::new().unwrap();
            // let view = unsafe { NSTextView::new(mtm) };
            let view = unsafe { RustTextView::new(mtm, binding) };

            
            unsafe { data.real_parent.addSubview(&view) };

            (view.clone(),move ||unsafe {view.removeFromSuperview();})
        }).clone();
        data.persistent_storage.borrow_mut().garbage_collection_mark_used(identity);
        // ns_string!()
        let str = objc2_foundation::NSString::from_str(self.text_binding.get().as_str());

        unsafe { view.setString(&str) };
        NativeTextEditor{
            ns_view:view
        }
    }
    fn set_identity(mut self, identity: usize) -> Self {
        self.identity = Some(identity);
        self
    }
}

pub struct NativeTextEditor{
    ns_view:Retained<RustTextView>
}

impl ComputableLayout for NativeTextEditor {
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        let view = &self.ns_view;
        unsafe { view.setFrameSize(to.into()) };
        // self.ns_view.setFrame(frame);
    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        super::nsview_setposition(&self.ns_view, to);
    }

    fn destroy(&mut self) {
        
    }
}