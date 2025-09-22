
use crate::layout::{ComputableLayout, Position, RenderObject, Size};
use objc2::rc::Retained;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send};
use objc2_foundation::{MainThreadMarker, NSObject, NSObjectProtocol,NSSize};
use objc2_ui_kit::{
    UIApplication, UIApplicationDelegate, UIApplicationLaunchOptionsKey, UIContentContainer, UIViewController, UIViewControllerTransitionCoordinator, UIWindow
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
        #[unsafe(method(resize:))]
        fn objc_resize(&self,to:NSSize) {
            self.resize_to(to);
        }

        #[unsafe(method(bridge_window))]
        fn bridge_window(&self) -> &UIWindow {
            &self.ivars().window.get().unwrap()
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

// use objc2::ffi::id
#[derive(Default)]
pub struct RustViewControllerIVars {
    pub on_disappear: OnceCell<Box<dyn Fn()>>
}

define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `Delegate` does not implement `Drop`.
    #[unsafe(super = UIViewController)]
    #[thread_kind = MainThreadOnly]
    #[name = "RustViewController"]
    #[ivars = RustViewControllerIVars]
    pub struct RustViewController;

    // SAFETY: `NSObjectProtocol` has no safety requirements.
    unsafe impl NSObjectProtocol for RustViewController {}
    impl  RustViewController {
        #[unsafe(method_id(init))]
        fn init(this: Allocated<Self>)->Retained<Self>{
            let s = this.set_ivars(RustViewControllerIVars::default());
            unsafe { msg_send![super(s), init] }
        }
        #[unsafe(method(viewDidDisappear:))]
        fn did_disappear(&self,animated:bool){
            let Some(func) = self.ivars().on_disappear.get() else {return};
            func()
        }

       

    }


    // SAFETY: `NSApplicationDelegate` has no safety requirements.
    unsafe impl UIContentContainer for RustViewController {
        // SAFETY: The signature is correct.
         #[unsafe(method(viewWillTransitionToSize:withTransitionCoordinator:))]
        fn view_will_transition_to_size(&self, size: NSSize,_tc:objc2::ffi::id) {
            self.resize(size);
        }
    }


);
impl RustViewController {
    pub fn resize(&self,to:NSSize){
        let mtm = MainThreadMarker::new().unwrap();
        let shared = UIApplication::sharedApplication(mtm);
        unsafe { 
            let del = shared.delegate().unwrap();
            let _:() = msg_send![&del, resize:to];
        }
    }
}

pub unsafe fn create_window(mtm: MainThreadMarker) -> Retained<UIWindow> {
    let window = unsafe { UIWindow::init(UIWindow::alloc(mtm)) };
    // window.resize
    let mtm = MainThreadMarker::new().unwrap();
    let controller:Retained<RustViewController> = msg_send!(RustViewController::alloc(mtm), init);
    window.setRootViewController(Some(&controller));
    return window;
}
impl Delegate {
    //Mainthread only
    pub fn render<T: RenderObject>(&self, object: T)
    where
        T::Output: 'static,
    {
        let window = self.ivars().window.get().unwrap();
        let view = window.rootViewController().unwrap().view().unwrap();
        let stack = crate::view::resources::ResourceStack::Owned(Default::default());
        let persistent_storage = Default::default();
        let root: Box<dyn ComputableLayout> =
            Box::new(object.render(super::native::RenderData { real_parent: view, stack,persistent_storage }));
        let _ = ROOT_VIEW.with(|v| v.replace(Some(root)));

    }
    //Mainthread only
    pub fn resize(&self) {
        let window = self.ivars().window.get().unwrap();
        let view = window.rootViewController().unwrap().view().unwrap();
        let frame = view.frame();
        let size: Size<f64> = frame.size.into();
        ROOT_VIEW.with(|e| {
            let mut k = e.borrow_mut();
            let view = k.as_mut().unwrap();
            view.set_size(size.into());
            view.set_position(Position { x: 0.0, y: 0.0 });
        });
    }
    //Mainthread only
    pub fn resize_to(&self,to:NSSize) {
        // let window = self.ivars().window.get().unwrap();
        // let view = window.rootViewController().unwrap().view().unwrap();
        // let frame = view.frame();
        // let size: Size<f64> = frame.size.into();
        ROOT_VIEW.with(|e| {
            let mut k = e.borrow_mut();
            let view = k.as_mut().unwrap();
            view.set_size(to.into());
            view.set_position(Position { x: 0.0, y: 0.0 });
        });
    }
}
