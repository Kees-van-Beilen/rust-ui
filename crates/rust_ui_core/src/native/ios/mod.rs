mod app;
mod button;
mod click_view;
mod image;
mod scroll_view;
mod sheet;
mod text_field;

use crate::{icon::Icon, layout::ComputableLayout};
use objc2::rc::Retained;
use objc2_foundation::NSString;
use objc2_ui_kit::{UIImage, UIView};

//all nsview reps auto participate in the layout manager
//todo: this file should be split up
trait UIViewRepresentable {
    fn ui_view(&self) -> &UIView;
}
impl<T: UIViewRepresentable> ComputableLayout for T {
    fn set_size(&mut self, to: crate::layout::Size<f64>) {
        let view = self.ui_view();
        let mut frame = view.frame();
        frame.size = to.into();
        view.setFrame(frame);
    }

    fn set_position(&mut self, to: crate::layout::Position<f64>) {
        let view = self.ui_view();
        let mut frame = view.frame();
        frame.origin = to.into();
        view.setFrame(frame);
    }

    fn destroy(&mut self) {
        let view = self.ui_view();
        unsafe { view.removeFromSuperview() };
    }
}

impl Into<Retained<UIImage>> for Icon {
    fn into(self) -> Retained<UIImage> {
        match self {
            Icon::System(id) => unsafe {
                UIImage::systemImageNamed(&NSString::from_str(id)).unwrap()
            },
        }
    }
}

pub mod native {
    pub use super::image::*;
    use super::{
        UIViewRepresentable,
        app::{Delegate, ON_LAUNCH_SIGNAL},
    };
    use crate::{
        icon::Icon,
        layout::{self, ComputableLayout, Position, RenderObject, Size},
        view::{
            mutable::MutableViewRerender,
            persistent_storage::PersistentStorageRef,
            resources::{Resource, ResourceStack, Resources},
        },
        views::{FontFamily, FontSize, FontWeight, TextAlignment, tabbar::RenderedTabData},
    };
    use block2::RcBlock;
    use objc2::{ClassType, MainThreadMarker, MainThreadOnly, rc::Retained};
    use objc2_core_graphics::{CGColor, CGRectZero};
    use objc2_foundation::{NSArray, NSBundle, NSFileManager, NSString};
    use objc2_ui_kit::{
        NSLineBreakMode, NSTextAlignment, UIAction, UIApplication, UIButton, UIColor,
        UIControlState, UIFontWeight, UIFontWeightBlack, UIFontWeightBold, UIFontWeightHeavy,
        UIFontWeightLight, UIFontWeightMedium, UIFontWeightRegular, UIFontWeightSemibold,
        UIFontWeightThin, UIFontWeightUltraLight, UIImage, UILabel, UITab, UITabBarController,
        UIView, UIViewController,
    };
    use std::{cell::RefCell, ptr::NonNull, rc::Rc};

    pub struct MutableView {
        children: Box<dyn ComputableLayout>,
        parent: Retained<UIView>,
        layout_size: layout::Size<f64>,
        stack: crate::view::resources::Resources,
        persistent_storage: PersistentStorageRef,
        layout_position: Position<f64>,
    }

    impl<T: crate::view::mutable::MutableView + 'static> MutableViewRerender for Rc<RefCell<T>> {
        fn rerender(&self) {
            let mut data = self.borrow_mut();
            if let Some(k) = &mut data.get_mut_attached() {
                let render_data = {
                    let mut b = k.borrow_mut();
                    b.children.destroy();
                    let render_data = RenderData {
                        real_parent: b.parent.clone(),

                        // persistent_storage:
                        //TODO: fix this clone to a ref
                        stack: crate::view::resources::ResourceStack::Owned(b.stack.clone()),
                        persistent_storage: b.persistent_storage.clone(),
                    };
                    render_data
                };
                drop(data);
                let _ = self.render(render_data);
            } else {
                drop(data);
            }
        }
    }

    fn font_weight_to_ui_font_weight(weight: FontWeight) -> UIFontWeight {
        unsafe {
            match weight {
                FontWeight::Ultralight => UIFontWeightUltraLight,
                FontWeight::Thin => UIFontWeightThin,
                FontWeight::Light => UIFontWeightLight,
                FontWeight::Regular => UIFontWeightRegular,
                FontWeight::Medium => UIFontWeightMedium,
                FontWeight::Semibold => UIFontWeightSemibold,
                FontWeight::Bold => UIFontWeightBold,
                FontWeight::Heavy => UIFontWeightHeavy,
                FontWeight::Black => UIFontWeightBlack,
            }
        }
    }

    impl<T: crate::view::mutable::MutableView + 'static> RenderObject for Rc<RefCell<T>> {
        type Output = Rc<RefCell<crate::native::MutableView>>;

        fn render(&self, data: crate::native::RenderData) -> Self::Output {
            // let r = T::children(self.clone()).render(data.clone());
            // let view = Rc::new(RefCell::new(MutableView {
            //     children: Box::new(r),
            //     layout_size: layout::Size {
            //         width: 0.0,
            //         height: 0.0,
            //     },
            //     parent: data.real_parent,
            //     stack: data.stack.as_ref().clone(),

            // }));
            // let mut m = self.borrow_mut();
            // let mut attached = m.get_mut_attached();
            // if let Some(k) = &mut attached {
            //     k.swap(&view);
            //     k.set_size(view.borrow().layout_size);
            //     k.set_position(Position { x: 0.0, y: 0.0 });
            // } else {
            //     *attached = Some(view.clone());
            // }
            // view

            type Store<T> = (Resources, PersistentStorageRef, Rc<RefCell<T>>);
            let identity = self.borrow().get_identity();
            let mut borrow = data.persistent_storage.borrow_mut();
            let mut resume_storage = true;
            // let mut did_swap = false;
            // let mut did_try_swap = false;
            let (res, storage, self_container) =
                borrow.get_or_init_with::<Store<T>>(identity, || {
                    resume_storage = false;
                    (
                        data.stack.as_ref().clone(),
                        PersistentStorageRef::default(),
                        self.clone(),
                    )
                });

            //we need to copy the state from the last
            if !Rc::ptr_eq(self, self_container) {
                // this code will execute iff the something else is rerendering this view in the same
                // frame that this view's state is updated.
                // this happens when a view updates a binding and a state variable at the same time
                self_container
                    .borrow()
                    .clone_bindings(&mut self.borrow_mut());
            }

            let new_data = RenderData {
                real_parent: data.real_parent,
                stack: ResourceStack::Owned(res.clone()),
                persistent_storage: storage.clone(),
            };
            drop(borrow);
            new_data
                .persistent_storage
                .borrow_mut()
                .garbage_collection_unset_all();
            let r = T::children(self.clone()).render(new_data.clone());
            new_data
                .persistent_storage
                .borrow_mut()
                .garbage_collection_cycle();

            let view = Rc::new(RefCell::new(MutableView {
                children: Box::new(r),
                layout_size: layout::Size {
                    width: 0.0,
                    height: 0.0,
                },

                parent: new_data.real_parent,
                stack: match new_data.stack {
                    ResourceStack::Owned(resources) => resources,
                    ResourceStack::Borrow(resources) => resources.clone(),
                },
                persistent_storage: data.persistent_storage,
                layout_position: layout::Position::default(),
            }));

            let mut m = self.borrow_mut();
            let mut attached = m.get_mut_attached();
            if let Some(k) = &mut attached {
                k.swap(&view);
                k.set_size(view.borrow().layout_size);
                k.set_position(view.borrow().layout_position);
            } else {
                *attached = Some(view.clone());
            }

            m.get_attached().clone().unwrap()
        }

        fn set_identity(self, identity: usize) -> Self {
            self.borrow_mut().set_identity(identity);
            self
        }
    }

    impl ComputableLayout for Rc<RefCell<MutableView>> {
        fn set_size(&mut self, to: layout::Size<f64>) {
            self.borrow_mut().layout_size = to;
            self.borrow_mut().children.set_size(to);
        }

        fn set_position(&mut self, to: layout::Position<f64>) {
            self.borrow_mut().layout_position = to;
            self.borrow_mut().children.set_position(to);
        }

        fn destroy(&mut self) {
            self.borrow_mut().children.destroy();
            // self.borrow_mut().parent = unsafe { Retained::from_raw(objc2::ffi::Nil) }.unwrap().downcast().unwrap();
        }
        fn preferred_size(&self, in_frame: &Size<f64>) -> Size<Option<f64>> {
            self.borrow().children.preferred_size(in_frame)
        }
    }

    pub struct Text(Retained<UILabel>);

    pub struct Button(Retained<UIButton>);
    impl RenderObject for crate::views::Button {
        type Output = Button;

        fn render(&self, data: crate::native::RenderData) -> Self::Output {
            let mtm = MainThreadMarker::new().unwrap();
            let view = unsafe {
                // let cb = self.callback.replace(Box::new(|| panic!()));
                // let view = RustButton::new(mtm, cb);
                // let clos = Box::new(||println!("test"));
                // let cb = self
                let cb = self.callback.replace(Box::new(|| panic!()));
                let callback: block2::RcBlock<dyn Fn(NonNull<UIAction>)> =
                    block2::RcBlock::new(move |_action: NonNull<UIAction>| cb());
                let ptr: *const block2::Block<dyn Fn(NonNull<UIAction>)> = &*callback;
                let ptr: *mut block2::Block<dyn Fn(NonNull<UIAction>)> =
                    ptr as *mut block2::Block<dyn Fn(NonNull<UIAction>)>;
                let action = UIAction::actionWithHandler(ptr, mtm);
                let view = UIButton::initWithFrame_primaryAction(
                    UIButton::alloc(mtm),
                    CGRectZero,
                    Some(&action),
                );
                let str = NSString::from_str(&self.label);
                view.setTitle_forState(Some(&str), UIControlState::Normal);
                view.sizeToFit();
                data.real_parent.addSubview(&view);
                view
            };
            Button(view)
        }
    }

    impl RenderObject for crate::views::Text {
        type Output = Text;

        fn render(&self, data: crate::native::RenderData) -> Self::Output {
            let mtm = MainThreadMarker::new().unwrap();
            let view = unsafe {
                let view = UILabel::new(mtm);
                let str = NSString::from_str(&self.content);
                view.setText(Some(&str));
                view.sizeToFit();

                let alignment = data
                    .stack
                    .get_resource::<TextAlignment>()
                    .copied()
                    .unwrap_or(TextAlignment::Center);
                view.setTextAlignment(match alignment {
                    TextAlignment::Leading => NSTextAlignment::Left,
                    TextAlignment::Center => NSTextAlignment::Center,
                    TextAlignment::Trailing => NSTextAlignment::Right,
                });
                view.setLineBreakMode(NSLineBreakMode::ByWordWrapping);
                view.setNumberOfLines(0);

                use objc2_ui_kit::UIFont;

                let font_family = data
                    .stack
                    .get_resource::<FontFamily>()
                    .unwrap_or(&crate::views::FontFamily::SystemUI);
                let font_size = data
                    .stack
                    .get_resource::<FontSize>()
                    .copied()
                    .unwrap_or(FontSize(UIFont::systemFontSize()));
                let font_weight = data
                    .stack
                    .get_resource::<FontWeight>()
                    .copied()
                    .unwrap_or(FontWeight::Regular);
                match font_family {
                    FontFamily::SystemUI => {
                        let font = UIFont::systemFontOfSize_weight(
                            font_size.0,
                            font_weight_to_ui_font_weight(font_weight),
                        );
                        view.setFont(Some(&font));
                    }
                    FontFamily::Custom(_) => todo!(),
                }

                data.real_parent.addSubview(&view);
                view
            };
            Text(view)
        }
    }

    impl ComputableLayout for Text {
        fn preferred_size(&self, in_frame: &layout::Size<f64>) -> Size<Option<f64>> {
            let frame = unsafe { self.0.sizeThatFits((*in_frame).into()) };
            frame.into()
        }
        fn set_size(&mut self, to: crate::layout::Size<f64>) {
            let view = &self.0;
            let mut frame = view.frame();
            frame.size = to.into();
            view.setFrame(frame);
        }

        fn set_position(&mut self, to: crate::layout::Position<f64>) {
            let view = &self.0;
            let mut frame = view.frame();
            frame.origin = to.into();
            view.setFrame(frame);
        }

        fn destroy(&mut self) {
            let view = &self.0;
            unsafe { view.removeFromSuperview() };
        }
    }
    impl ComputableLayout for Button {
        fn preferred_size(&self, in_frame: &layout::Size<f64>) -> Size<Option<f64>> {
            let frame = unsafe { self.0.sizeThatFits((*in_frame).into()) };
            frame.into()
        }
        fn set_size(&mut self, to: crate::layout::Size<f64>) {
            let view = &self.0;
            let mut frame = view.frame();
            frame.size = to.into();
            view.setFrame(frame);
        }

        fn set_position(&mut self, to: crate::layout::Position<f64>) {
            let view = &self.0;
            let mut frame = view.frame();
            frame.origin = to.into();
            view.setFrame(frame);
        }

        fn destroy(&mut self) {
            let view = &self.0;
            unsafe { view.removeFromSuperview() };
        }
    }
    pub struct ColorView(Retained<UIView>);
    impl RenderObject for crate::views::ColorView {
        type Output = ColorView;

        fn render(&self, data: crate::native::RenderData) -> Self::Output {
            let v = self.0.to_srgba();
            let mtm = MainThreadMarker::new().unwrap();
            let view = unsafe {
                let view = UIView::new(mtm);
                let color =
                    CGColor::new_srgb(v.red as f64, v.green as f64, v.blue as f64, v.alpha as f64);
                // let layer = CALayer::layer();
                // layer.setBackgroundColor(Some(&color));
                // view.setLayer(Some(&layer));
                view.setBackgroundColor(Some(&UIColor::colorWithCGColor(&color)));
                data.real_parent.addSubview(&view);
                view
            };
            ColorView(view)
        }
    }
    impl UIViewRepresentable for ColorView {
        fn ui_view(&self) -> &UIView {
            &self.0
        }
    }
    #[derive(Clone)]

    pub struct RenderData<'a> {
        pub real_parent: Retained<UIView>,
        pub stack: crate::view::resources::ResourceStack<'a>,
        pub persistent_storage: PersistentStorageRef,
    }

    impl<'a> RenderData<'a> {
        pub fn ament_with<T: Resource, F, K>(&mut self, element: T, with_fn: F) -> K
        where
            for<'b> F: FnOnce(RenderData) -> K,
        {
            self.stack.amend_with(element, |stack_e| {
                let d = RenderData {
                    real_parent: self.real_parent.clone(),
                    stack: ResourceStack::Borrow(stack_e),
                    persistent_storage: self.persistent_storage.clone(),
                };

                with_fn(d)
            })
        }
    }

    pub struct RenderedTabBar {
        _active_tab: usize,
        views: Vec<Box<dyn ComputableLayout>>,
    }
    impl<T: crate::views::tabbar::TabGroup> RenderObject for crate::views::tabbar::TabBar<T> {
        type Output = RenderedTabBar;

        fn render(&self, data: crate::native::RenderData) -> Self::Output {
            let mtm = MainThreadMarker::new().unwrap();
            let views = unsafe {
                let controller = UITabBarController::new(mtm);
                let window = data.real_parent.window().unwrap();
                let s_view = window.rootViewController().unwrap().view().unwrap();
                assert_eq!(
                    s_view, data.real_parent,
                    "TabBar must be a root view, it cannot be the child of any view"
                );
                window.setRootViewController(Some(&controller));
                let mut uitabs = Vec::<Retained<UITab>>::new();
                let mut i = 0;

                let views = self.children.for_each_render(|data: RenderedTabData| {
                    let vc = UIViewController::new(mtm);
                    let vc_clone = vc.clone();
                    let image = match data.icon {
                        Some(Icon::System(id)) => {
                            Some(UIImage::systemImageNamed(&NSString::from_str(id)).unwrap())
                        }
                        None => None,
                    };
                    let image_ref: Option<&UIImage> = match &image {
                        Some(r) => Some(r.as_ref()),
                        None => None,
                    };
                    let itm = UITab::initWithTitle_image_identifier_viewControllerProvider(
                        UITab::alloc(mtm),
                        &NSString::from_str(data.title),
                        image_ref,
                        &NSString::from_str(&i.to_string()),
                        Some(&*RcBlock::new(move |_tab: NonNull<UITab>| {
                            let rtn = (&*vc_clone).into();
                            rtn
                        })),
                    );

                    uitabs.push(itm);
                    i += 1;
                    RenderData {
                        real_parent: vc.view().unwrap(),

                        //Tab bar cannot contain any styling rule
                        stack: ResourceStack::Owned(Default::default()),
                        persistent_storage: todo!(),
                    }
                });
                let tabs = NSArray::from_retained_slice(&uitabs[..]);
                controller.setTabs(&tabs);
                views
            };
            RenderedTabBar {
                _active_tab: self.active_tab,
                views,
            }
        }
    }
    impl ComputableLayout for RenderedTabBar {
        fn set_size(&mut self, to: layout::Size<f64>) {
            self.views.iter_mut().for_each(|e| e.set_size(to));
        }

        fn set_position(&mut self, _to: layout::Position<f64>) {}

        fn destroy(&mut self) {
            self.views.iter_mut().for_each(|e| e.destroy());
        }
    }

    pub fn launch_application_with_view(root: impl RenderObject + 'static) {
        //setup the default filemanager
        unsafe {
            let default_file_manager = NSFileManager::defaultManager();
            let path = NSBundle::mainBundle().bundlePath();
            default_file_manager.changeCurrentDirectoryPath(&path);
        }

        let mtm = MainThreadMarker::new().unwrap();
        //this is required to register the class
        let name = Delegate::class().to_string();

        ON_LAUNCH_SIGNAL.set(Some(Box::new(|del: &Delegate| {
            del.render(root);
        })));

        UIApplication::main(None, Some(&NSString::from_str(&name)), mtm);
    }
}
