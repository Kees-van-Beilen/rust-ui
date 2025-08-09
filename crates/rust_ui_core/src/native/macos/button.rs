//custom button class
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, rc::Retained, sel};
use objc2_app_kit::NSButton;
use objc2_foundation::{MainThreadMarker, NSObjectProtocol};

pub struct RustButtonIvars {
    /// Callback to a in rust defined function
    /// TODO: objc2 doesn't support generics in classes
    callback: Box<dyn Fn()>,
}
define_class!(
    #[unsafe(super = NSButton)]
    #[thread_kind = MainThreadOnly]
    #[name = "RustButton"]
    #[ivars = RustButtonIvars]
    pub struct RustButton;

    // SAFETY: `NSObjectProtocol` has no safety requirements.
    unsafe impl NSObjectProtocol for RustButton {}

    impl RustButton {
        ///SAFETY: obj-c function selector is correct (takes no args)
        #[unsafe(method(clicked))]
        fn clicked(&self) {
            (self.ivars().callback)();
        }
    }

);
impl RustButton {
    pub fn new(mtm: MainThreadMarker, callback: Box<dyn Fn()>) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(RustButtonIvars { callback });
        // SAFETY: The signature of `NSObject`'s `init` method is correct.
        let s: Retained<RustButton> = unsafe { msg_send![super(this), init] };
        //SAFETY: this wil always work as a weak ref to self is always valid
        unsafe { s.setTarget(Some(&s)) };
        //SAFETY: obj-c function selector is correct (takes no args)
        unsafe { s.setAction(Some(sel!(clicked))) };
        s
    }
}
