// use crate::{layout::{ComputableLayout, RenderObject}, prelude::sheet::SheetModalPresenterView, view::state::PartialBinding};

use std::cell::RefCell;

use crate::{
    layout::{ComputableLayout, Position, RenderObject, Size},
    native::RenderData,
    prelude::sheet::SheetModalPresenterView,
    view::{persistent_storage::PersistentStorageRef, resources::Resources},
};
use objc2::{
    DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, rc::Retained,
    runtime::ProtocolObject,
};
use objc2_app_kit::{NSApp, NSBackingStoreType, NSWindow, NSWindowDelegate, NSWindowStyleMask};
use objc2_foundation::{NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize};

pub struct RustWindowDelegateIVars {
    sheet: RefCell<Box<dyn ComputableLayout>>,
    window: Retained<NSWindow>,
}
define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `Delegate` does not implement `Drop`.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[name = "RustWindowDelegate"]
    #[ivars = RustWindowDelegateIVars]
    pub struct RustWindowDelegate;

    // SAFETY: `NSObjectProtocol` has no safety requirements.
    unsafe impl NSObjectProtocol for RustWindowDelegate {}

    // SAFETY: `NSWindowDelegate` has no safety requirements.
    unsafe impl NSWindowDelegate for RustWindowDelegate {
        #[unsafe(method(windowDidResize:))]
        fn window_will_resize(&self, _notification: &NSNotification) {
            self.resize();
        }
    }
);
impl RustWindowDelegate {
    pub fn resize(&self) {
        let ivars = self.ivars();
        let window = &ivars.window;
        let view = window.contentView().unwrap();
        let frame = view.frame();
        let size: Size<f64> = frame.size.into();
        let mut k = ivars.sheet.borrow_mut();
        k.set_size(size);
        k.set_position(Position { x: 0.0, y: 0.0 });
    }
    pub fn new_attached_to_window(
        sheet: Box<dyn ComputableLayout>,
        window: &Retained<NSWindow>,
    ) -> Retained<RustWindowDelegate> {
        let mtm = MainThreadMarker::new().unwrap();
        let init = RustWindowDelegate::alloc(mtm).set_ivars(RustWindowDelegateIVars {
            sheet: RefCell::new(sheet),
            window: window.clone(),
        });
        let r: Retained<RustWindowDelegate> = unsafe { msg_send![super(init), init] };
        window.setDelegate(Some(ProtocolObject::from_ref(&*r)));
        r
    }
}

type PersistData<'a> = Option<(
    Retained<NSWindow>,
    RenderData<'a>,
    Retained<RustWindowDelegate>,
)>;

impl<Sheet: RenderObject + 'static, View: RenderObject> RenderObject
    for SheetModalPresenterView<View, Sheet>
{
    type Output = View::Output;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        // // let identity =  self.sheet.unwrap()

        // data.persistent_storage.borrow().get(identity)

        // NativeModalPresenter {
        //     binding: self,
        //     child: todo!(),
        // }
        let show_sheet = *self.binding.get();

        let render = self.view.render(data.clone());
        if let Some((sheet_generator, identity)) = &self.sheet {
            // println!("sheet rerender {show_sheet} {identity}");
            // data.persistent_storage.borrow().get::<Option<(
            //     Retained<NSWindow>,
            // )>>(identity);

            let mtm = MainThreadMarker::new().unwrap();
            {
                let borrow = data.persistent_storage.borrow();
                if let Some(Some((window, render_data, del))) = borrow.get::<PersistData>(*identity)
                {
                    if show_sheet {
                        let sheet = sheet_generator();
                        let mut rendered = sheet.render(render_data.clone());
                        rendered.set_size(NSSize::new(300.0, 300.0).into());
                        rendered.set_position(Position::default());
                        del.ivars().sheet.replace(Box::new(rendered)).destroy();
                    } else {
                        println!("close window");
                        let main_window = NSApp(mtm).windows().firstObject().unwrap();
                        unsafe { main_window.endSheet(&del.ivars().window) };
                        let _ = window;
                        let _ = render_data;
                        let _ = del;
                        drop(borrow);
                        data.persistent_storage
                            .borrow_mut()
                            .insert::<PersistData>(*identity, None);
                    }
                } else {
                    if show_sheet {
                        let sheet = sheet_generator();
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
                        let new_data = RenderData {
                            real_parent: window.contentView().unwrap(),
                            stack: crate::view::resources::ResourceStack::Owned(
                                Resources::default(),
                            ),
                            persistent_storage: PersistentStorageRef::default(),
                        };
                        let main_window = NSApp(mtm).windows().firstObject().unwrap();
                        let mut rendered = sheet.render(new_data.clone());
                        rendered.set_size(NSSize::new(300.0, 300.0).into());
                        rendered.set_position(Position::default());
                        unsafe { main_window.beginSheet_completionHandler(&window, None) };
                        let del =
                            RustWindowDelegate::new_attached_to_window(Box::new(rendered), &window);
                        drop(borrow);
                        data.persistent_storage
                            .borrow_mut()
                            .insert::<PersistData>(*identity, Some((window, new_data, del)));
                    } else {
                        // println!("lost context");
                    }
                }
            }
            render
        } else {
            panic!("oops")
        }
    }
}
