// use crate::{layout::{ComputableLayout, RenderObject}, prelude::sheet::SheetModalPresenterView, view::state::PartialBinding};

use std::cell::{Cell, RefCell};

use crate::{
    layout::{ComputableLayout, Position, RenderObject, Size},
    native::{ios::app::{create_window, RustViewController}, RenderData},
    prelude::sheet::SheetModalPresenterView,
    view::{persistent_storage::PersistentStorageRef, resources::Resources},
};
use block2::{Block, DynBlock};
use objc2::{define_class, msg_send, rc::Retained, runtime::ProtocolObject, DefinedClass, MainThreadMarker, MainThreadOnly};
// use objc2_app_kit::{NSApp, NSBackingStoreType, NSWindow, NSWindowDelegate, NSWindowStyleMask};
use objc2_foundation::{NSNotification, NSObjectProtocol, NSPoint, NSRect, NSSize,NSObject};
use objc2_ui_kit::{UIApplication, UIApplicationDelegate, UIColor, UIWindow};



// pub struct RustWindowDelegateIVars {
//     sheet:RefCell<Box<dyn ComputableLayout>>,
//     window:Retained<UIWindow>
// }
// define_class!(
//     // SAFETY:
//     // - The superclass NSObject does not have any subclassing requirements.
//     // - `Delegate` does not implement `Drop`.
//     #[unsafe(super = NSObject)]
//     #[thread_kind = MainThreadOnly]
//     #[name = "RustWindowDelegate"]
//     #[ivars = RustWindowDelegateIVars]
//     pub struct RustWindowDelegate;

//     // SAFETY: `NSObjectProtocol` has no safety requirements.
//     unsafe impl NSObjectProtocol for RustWindowDelegate {}

//      // SAFETY: `NSWindowDelegate` has no safety requirements.
//     unsafe impl UIWindow for RustWindowDelegate {
//         #[unsafe(method(windowDidResize:))]
//         fn window_will_resize(&self, _notification: &NSNotification) {
//             self.resize();
//         }
//     }

// );
// impl RustWindowDelegate {
//     pub fn resize(&self) {
//         let ivars = self.ivars();
//         let window = &ivars.window;
//         let view = window.contentView().unwrap();
//         let frame = view.frame();
//         let size: Size<f64> = frame.size.into();
//         let mut k = ivars.sheet.borrow_mut();
//         k.set_size(size);
//         k.set_position(Position { x: 0.0, y: 0.0 });
//     }
//     pub fn new_attached_to_window(sheet:Box<dyn ComputableLayout>,window:&Retained<UIWindow>)->Retained<RustWindowDelegate>{
//         let mtm = MainThreadMarker::new().unwrap();
//         let init = RustWindowDelegate::alloc(mtm).set_ivars(RustWindowDelegateIVars{
//             sheet: RefCell::new(sheet),
//             window: window.clone(),
//         });
//         let r:Retained<RustWindowDelegate> = unsafe { msg_send![super(init), init] };
//         window.setDelegate(Some(ProtocolObject::from_ref(&*r)));
//         r
//     }
// }



impl<Sheet: RenderObject+'static, View: RenderObject> RenderObject
    for SheetModalPresenterView<View, Sheet>
{
    type Output = View::Output;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        // // let identity =  self.sheet.unwrap()


        type PersistData<'a,Sheet:RenderObject+'static> = Option<(RenderData<'a>, Cell<Sheet::Output>)>;

        // data.persistent_storage.borrow().get(identity)

        // NativeModalPresenter {
        //     binding: self,
        //     child: todo!(),
        // }
        let show_sheet =  *self.binding.get();
            
        let render = self.view.render(data.clone());
        if let Some((sheet_generator, identity)) = &self.sheet {
            // println!("sheet rerender {show_sheet} {identity}");
            // data.persistent_storage.borrow().get::<Option<(
            //     Retained<NSWindow>,
            // )>>(identity);

            let mtm = MainThreadMarker::new().unwrap();
            {

            let borrow = data
                .persistent_storage
                .borrow();
            if let Some(Some((render_data,previous_sheet))) = borrow.get::<PersistData<Sheet>>(*identity)
            {
                if show_sheet {
                    let sheet = sheet_generator();
                    let mut rendered = sheet.render(render_data.clone());
                    rendered.set_size(NSSize::new(300.0, 300.0).into());
                    rendered.set_position(Position::default());
                    previous_sheet.replace(rendered).destroy();
                    // del.ivars().sheet.replace(Box::new(rendered)).destroy();

                }else {
                    
                    // println!("close window");
                    // let main_window = unsafe { UIApplication::sharedApplication(mtm).delegate().unwrap().window().unwrap() };
                    // main_window.rootViewController().unwrap().presentViewController_animated_completion(view_controller_to_present, flag, completion);
                    let main_window:&UIWindow = unsafe { 
                        msg_send![&UIApplication::sharedApplication(mtm).delegate().unwrap(), bridge_window] };
                    unsafe { main_window.rootViewController().unwrap().dismissViewControllerAnimated_completion(true, None) };
                    // let main_window = NSApp(mtm).windows().firstObject().unwrap();
                    // unsafe { main_window.endSheet(&del.ivars().window) };
                    // let _ = window;
                    // let _ = render_data;
                    // let _ = del;
                    drop(borrow);
                    data.persistent_storage.borrow_mut().insert::<PersistData<Sheet>>(*identity, None);
                }
            }else{
                if show_sheet {
                    let sheet = sheet_generator();

                    // let window = unsafe { create_window(mtm) };
                    let main_window:&UIWindow = unsafe { 
                        msg_send![&UIApplication::sharedApplication(mtm).delegate().unwrap(), bridge_window] };
                    
                    let root_controller = main_window.rootViewController().unwrap();

                    let controller:Retained<RustViewController> = unsafe { msg_send!(RustViewController::alloc(mtm), init) };
                    let binding_clone = self.binding.clone();
                    controller.ivars().on_disappear.set(Box::new(move ||{
                        binding_clone.update_value(false);
                    }));
                        
                    let bg = unsafe { UIColor::secondarySystemBackgroundColor() };
                    controller.view().unwrap().setBackgroundColor(Some(&bg));

                    
                    let render_data = RenderData {
                        real_parent: controller.view().unwrap(),
                        stack: Default::default(),
                        persistent_storage: Default::default(),
                    };

                    let mut render = sheet.render(render_data.clone());

                    
                    // DynBlock
                    // let binding_clone = self.binding.clone();
                    // let dismiss_block = block2::RcBlock::new(move ||{
                    //     binding_clone.update_value(false);
                    // });
                    unsafe { root_controller.presentViewController_animated_completion(&controller, true, None) };

                    let mut size: Size<f64> = unsafe { root_controller.view().unwrap().frame().size.into()};
                    size.height -= 100.0;
                    render.set_size(size);
                    render.set_position(Position::default());


                    // main_window.rootViewController().unwrap();
                    
                    
                    // let window = unsafe {
                    //     NSWindow::initWithContentRect_styleMask_backing_defer(
                    //         NSWindow::alloc(mtm),
                    //         NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(300.0, 300.0)),
                    //         NSWindowStyleMask::Titled
                    //             | NSWindowStyleMask::Closable
                    //             | NSWindowStyleMask::Miniaturizable
                    //             | NSWindowStyleMask::Resizable,
                    //         NSBackingStoreType::Buffered,
                    //         false,
                    //     )
                    // };
                    // let new_data = RenderData {
                    //     real_parent: window.contentView().unwrap(),
                    //     stack: crate::view::resources::ResourceStack::Owned(Resources::default()),
                    //     persistent_storage: PersistentStorageRef::default(),
                    // };
                    // let main_window =  NSApp(mtm).windows().firstObject() .unwrap();
                    // let mut rendered = sheet.render(new_data.clone());
                    // rendered.set_size(NSSize::new(300.0, 300.0).into());
                    // rendered.set_position(Position::default());
                    // unsafe { main_window.beginSheet_completionHandler(&window, None) };
                    // let del = RustWindowDelegate::new_attached_to_window(Box::new(rendered), &window);
                    // drop(borrow);
                    drop(borrow);
                    data.persistent_storage.borrow_mut().insert::<PersistData<Sheet>>(*identity, Some((render_data,Cell::new(render))));
                    // data.persistent_storage.borrow_mut().insert::<PersistData>(*identity, Some((window,new_data,del)));

                }else {
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
