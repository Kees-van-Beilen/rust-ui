// pub fn appl

use crate::layout::{ComputableLayout, RenderObject, Size};
use objc2::rc::Retained;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send};
use objc2_foundation::{MainThreadMarker, NSObject, NSObjectProtocol};
use objc2_ui_kit::{
    UIApplication, UIApplicationDelegate, UIApplicationLaunchOptionsKey, UIColor, UIViewController,
    UIWindow,
};
use std::cell::{Cell, OnceCell, RefCell};
//These are globals meant for the main thread only. We do this because we are not the one
//who initialize the ApplicationDelegate so we have no control over its init variables.
thread_local! {
    pub static ROOT_VIEW: RefCell<Option<Box<dyn ComputableLayout>>> = RefCell::new(None);
    pub static ON_LAUNCH_SIGNAL: Cell<Option<Box<dyn FnOnce(&Delegate)>>> = Cell::new(None);
}

#[derive(Default)]
pub struct AppDelegateIvars {
    pub window: OnceCell<Retained<UIWindow>>,
}

use objc2::rc::Allocated;
use objc2::runtime::AnyObject;
use objc2_foundation::NSDictionary;

define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `Delegate` does not implement `Drop`.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[name = "Delegate"]
    #[ivars = AppDelegateIvars]
    pub struct Delegate;

    // SAFETY: `NSObjectProtocol` has no safety requirements.
    unsafe impl NSObjectProtocol for Delegate {}
    impl  Delegate {
        #[unsafe(method_id(init))]
        fn init(this: Allocated<Self>)->Retained<Self>{
            let s = this.set_ivars(AppDelegateIvars::default());
            unsafe { msg_send![super(s), init] }
        }
    }

    // SAFETY: `NSApplicationDelegate` has no safety requirements.
    unsafe impl UIApplicationDelegate for Delegate {
        // SAFETY: The signature is correct.
        #[unsafe(method(application:didFinishLaunchingWithOptions:))]
        fn did_finish_launching(&self, _application: &UIApplication,_options:Option<&NSDictionary<UIApplicationLaunchOptionsKey, AnyObject>>)->bool {
            let mtm = self.mtm();
            let window = unsafe {create_window(mtm) };
            window.makeKeyAndVisible();
            self.ivars().window.set(window).unwrap();
            let signal = ON_LAUNCH_SIGNAL.with(|e|e.take()).unwrap();
            signal(self);
            self.resize();
            true
        }
    }


);

pub unsafe fn create_window(mtm: MainThreadMarker) -> Retained<UIWindow> {
    let window = UIWindow::init(UIWindow::alloc(mtm));
    // window.setBackgroundColor(Some(&UIColor::blueColor()));
    window.setRootViewController(Some(&UIViewController::new(mtm)));

    return window;
}
impl Delegate {
    ///Mainthread only
    pub fn render<T: RenderObject>(&self, object: T)
    where
        T::Output: 'static,
    {
        let window = self.ivars().window.get().unwrap();
        let view = window.rootViewController().unwrap().view().unwrap();
        let stack = crate::view::resources::ResourceStack::Owned(Default::default());
        let root: Box<dyn ComputableLayout> =
            Box::new(object.render(super::native::RenderData { real_parent: view, stack }));
        let _ = ROOT_VIEW.with(|v| v.replace(Some(root)));

        // self.ivars().root.replace(Some(root));
    }
    ///Mainthread only
    pub fn resize(&self) {
        // NSObject::init(this)
        let window = self.ivars().window.get().unwrap();
        let view = window.rootViewController().unwrap().view().unwrap();
        let frame = view.frame();
        let size: Size<f64> = frame.size.into();
        ROOT_VIEW.with(|e| {
            let mut k = e.borrow_mut();
            k.as_mut().unwrap().set_size(size);
        });
    }
}
