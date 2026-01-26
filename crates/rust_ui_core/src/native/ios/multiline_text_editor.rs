use std::cell::RefCell;

use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_ui_kit::{UIScrollViewDelegate, UITextView, UITextViewDelegate};
use objc2_foundation::{NSObject, NSObjectProtocol};

use crate::{layout::{ComputableLayout, RenderObject}, view::state::PartialBindingBox, views::textfield::TextEditor};
// use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, rc::Retained, runtime::ProtocolObject};
// use objc2_app_kit::{NSTextDelegate, NSTextView, NSTextViewDelegate};
// use objc2_foundation::{NSNotification, NSObject, NSObjectProtocol, ns_string};

// use crate::{layout::{ComputableLayout, RenderObject}, view::state::{PartialAnyBinding, PartialBindingBox}, views::textfield::TextEditor};

pub struct RustTextViewDelegateIVars {
    binding:RefCell<PartialBindingBox<String>>
}

define_class!(
    #[unsafe(super=NSObject)]
    #[thread_kind = MainThreadOnly]
    #[name = "RustUITextViewDelegate"]
    #[ivars=RustTextViewDelegateIVars]
    pub struct RustTextViewDelegate;

    unsafe impl NSObjectProtocol for RustTextViewDelegate {}
    unsafe impl UIScrollViewDelegate for RustTextViewDelegate {}
    unsafe impl UITextViewDelegate  for RustTextViewDelegate {

        #[unsafe(method(textViewDidChange:))]
        fn text_view_did_change(&self, text_view: &UITextView) -> () {
            // panic!("hoihoi");
            let vars = self.ivars();
            unsafe {
                // let obj = notification.object().unwrap();
                let str:Retained<objc2_foundation::NSString> = text_view.text();
                let rust_str = str.to_string();
                vars.binding.borrow_mut().update_value(rust_str);
            }
            // (vars.binding.borrow_mut()).update_value(value);

        }
    }

);
// define_class!(
//     // SAFETY:
//     // - The superclass NSObject does not have any subclassing requirements.
//     // - `Delegate` does not implement `Drop`.
//     #[unsafe(super = NSTextView)]
//     #[thread_kind = MainThreadOnly]
//     #[name = "RustTextView"]
//     #[ivars = RustTextViewIVars]
//     pub struct RustTextView;

//     // unsafe impl NSObjectProtocol for RustTextViewDelegate {
        
//     // }
//     impl RustTextView {
//         #[unsafe(method(didChangeText))]
//         fn did_change_text(&self){
//             unsafe {
//                 let _:() = msg_send![super(self), didChangeText];
//             }
//             // let mtm = self.mtm();
//             let vars = self.ivars();
//             // println!("text did change");

//             unsafe {
//                 // let obj = notification.object().unwrap();
//                 let str:Retained<objc2_foundation::NSString> = self.string();
//                 let rust_str = str.to_string();
//                 println!("changed text to: {}",&rust_str);
//                 vars.binding.borrow_mut().update_value(rust_str);
//             }
//             // (vars.binding.borrow_mut()).update_value(value);

//         }
//     }
//     // unsafe impl NSTextViewDelegate for RustTextViewDelegate {

//     // }
// );
pub struct RustTextView {
    ui_text_view:Retained<UITextView>
}
impl RustTextViewDelegate {
    pub unsafe fn new(
        mtm: MainThreadMarker,
        binding: PartialBindingBox<String>,
    ) -> Retained<RustTextViewDelegate> {
        let this = Self::alloc(mtm).set_ivars(RustTextViewDelegateIVars { binding:RefCell::new(binding) });
        msg_send![super(this), init]
    }
}


impl RenderObject for TextEditor {
    type Output = NativeTextEditor;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {


        let identity = self.identity.expect("forgot to set identity on TextEditor");
        let (view,delegate) = data.persistent_storage.borrow_mut().get_or_register_gc(identity, ||{
            let binding = self.text_binding.clone_box();
            let mtm = MainThreadMarker::new().unwrap();
            // let view = unsafe { NSTextView::new(mtm) };
            let view = unsafe { UITextView::new(mtm) };
            let delegate = unsafe { RustTextViewDelegate::new(mtm, binding) };
            unsafe { view.setDelegate(Some(ProtocolObject::from_ref(&*delegate))) };

            unsafe { data.real_parent.addSubview(&view) };

            ((view.clone(),delegate),move ||unsafe {view.removeFromSuperview();})
        }).clone();
        data.persistent_storage.borrow_mut().garbage_collection_mark_used(identity);
        // ns_string!()
        let str = objc2_foundation::NSString::from_str(self.text_binding.get().as_str());

        unsafe { view.setText(Some(&str)) };
        NativeTextEditor{
            ns_view:view,
            delegate
        }
    }
    fn set_identity(mut self, identity: usize) -> Self {
        self.identity = Some(identity);
        self
    }
}

pub struct NativeTextEditor{
    ns_view:Retained<UITextView>,
    delegate:Retained<RustTextViewDelegate>
}

impl ComputableLayout for NativeTextEditor {
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        let view = &self.ns_view;
        let mut frame = view.frame();
        frame.size = to.into();
        view.setFrame(frame);
    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        let view = &self.ns_view;
        let mut frame = view.frame();
        frame.origin = to.into();
        view.setFrame(frame);

    }

    fn destroy(&mut self) {
        
    }
}