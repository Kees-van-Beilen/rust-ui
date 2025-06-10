// pub fn appl

use std::cell::{Cell, OnceCell, RefCell};

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSBackingStoreType,
    NSWindow, NSWindowDelegate, NSWindowStyleMask,
};
use objc2_foundation::{
    MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize,
    ns_string,
};

use crate::layout::{ComputableLayout, Position, RenderObject, Size};
use crate::view::resources::Resources;

#[derive(Default)]
pub struct AppDelegateIvars {
    //for now ugly boxes are used as obj-c classes cannot contain generics
    //tho technically they don't have to have a know size.
    pub window: OnceCell<Retained<NSWindow>>,
    //root rendered element
    pub root: RefCell<Option<Box<dyn ComputableLayout>>>,
    //the signal get called when the application reaches the ready state
    //this could be cleaner tbh.
    pub signal: Cell<Option<Box<dyn FnOnce(&Delegate)>>>,
}

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

    // SAFETY: `NSApplicationDelegate` has no safety requirements.
    unsafe impl NSApplicationDelegate for Delegate {
        // SAFETY: The signature is correct.
        #[unsafe(method(applicationDidFinishLaunching:))]
        fn did_finish_launching(&self, notification: &NSNotification) {
            let mtm = self.mtm();

            let app = unsafe { notification.object() }
                .unwrap()
                .downcast::<NSApplication>()
                .unwrap();

            // SAFETY: We disable releasing when closed below.
            let window = unsafe {
                NSWindow::initWithContentRect_styleMask_backing_defer(
                    NSWindow::alloc(mtm),
                    NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(300.0, 300.0)),
                    NSWindowStyleMask::Titled
                        | NSWindowStyleMask::Closable
                        | NSWindowStyleMask::Miniaturizable
                        | NSWindowStyleMask::Resizable,
                    NSBackingStoreType::Buffered,
                    false,
                )
            };
            // SAFETY: Disable auto-release when closing windows.
            // This is required when creating `NSWindow` outside a window
            // controller.
            unsafe { window.setReleasedWhenClosed(false) };

            // Set various window properties.
            window.setTitle(ns_string!("A window"));

            window.center();
            // unsafe { window.setContentMinSize(NSSize::new(300.0, 300.0)) };
            window.setDelegate(Some(ProtocolObject::from_ref(self)));

            // Show the window.
            window.makeKeyAndOrderFront(None);

            // Store the window in the delegate.
            self.ivars().window.set(window).unwrap();

            app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

            // Activate the application.
            // Required when launching unbundled (as is done with Cargo).
            #[allow(deprecated)]
            app.activateIgnoringOtherApps(true);

            if let Some(s) = self.ivars().signal.take() {
                s(&self);
            }
            //calculate initial layout
            self.resize();
        }
    }

    // SAFETY: `NSWindowDelegate` has no safety requirements.
    unsafe impl NSWindowDelegate for Delegate {
        #[unsafe(method(windowDidResize:))]
        fn window_will_resize(&self, _notification: &NSNotification) {
            self.resize();
        }

        #[unsafe(method(windowWillClose:))]
        fn window_will_close(&self, _notification: &NSNotification) {
            // Quit the application when the window is closed.
            unsafe { NSApplication::sharedApplication(self.mtm()).terminate(None) };
        }
    }
);

impl Delegate {
    pub fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(AppDelegateIvars::default());
        // SAFETY: The signature of `NSObject`'s `init` method is correct.
        unsafe { msg_send![super(this), init] }
    }

    pub fn render<T: RenderObject>(&self, object: T)
    where
        T::Output: 'static,
    {
        let window = self.ivars().window.get().unwrap();
        let view = window.contentView().unwrap();
        let root_res = Resources::default();
        let root: Box<dyn ComputableLayout> = Box::new(object.render(super::native::RenderData {
            real_parent: view,
            stack: crate::view::resources::ResourceStack::Owned(root_res),
        }));
        self.ivars().root.replace(Some(root));
    }
    pub fn resize(&self) {
        let window = self.ivars().window.get().unwrap();
        let view = window.contentView().unwrap();
        let frame = view.frame();
        let size: Size<f64> = frame.size.into();
        let mut k = self.ivars().root.borrow_mut();
        let b = k.as_mut().unwrap();
        b.set_size(size);
        b.set_position(Position { x: 0.0, y: 0.0 });
    }
}
