use std::fmt::Debug;

//custom view class that captures a on click event
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, rc::Retained, sel};
// use objc2_app_kit::NSView;
// use objc2_app_kit::NSEvent;
use objc2_ui_kit::{UITapGestureRecognizer, UIView};
use objc2_foundation::{MainThreadMarker, NSObjectProtocol};

pub struct ClickableContainerIVars {
    /// Callback to a in rust defined function
    /// TODO: objc2 doesn't support generics in classes
    callback: Box<dyn Fn()>,
}
define_class!(
    #[unsafe(super = UIView)]
    #[thread_kind = MainThreadOnly]
    #[name = "RustTapGestureReg"]
    #[ivars = ClickableContainerIVars]
    pub struct ClickableContainer;

    // SAFETY: `NSObjectProtocol` has no safety requirements.
    unsafe impl NSObjectProtocol for ClickableContainer {}

    impl ClickableContainer {
        ///SAFETY: obj-c function selector is correct (takes no args)
        /// - (void) mouseDown: (NSEvent*) theEvent;
        #[unsafe(method(touch:))]
        fn touch(&self,_event:&UITapGestureRecognizer) {
            (self.ivars().callback)();
        }
    }

);

impl Debug for ClickableContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClickableContainer").field("__superclass", &self.__superclass).field("__phantom", &self.__phantom).finish()
    }
}
// pub struct ClickableContainer {
//     root:Retained<UIView>,
//     callback:Box<dyn Fn()>
// }
impl ClickableContainer {
    pub fn new(mtm: MainThreadMarker, callback: Box<dyn Fn()>) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(ClickableContainerIVars { callback });
        // SAFETY: The signature of `NSObject`'s `init` method is correct.
        let s: Retained<ClickableContainer> = unsafe { msg_send![super(this), init] };
        let g = unsafe  {
            let g = UITapGestureRecognizer::new(mtm);
            g.addTarget_action(&s, sel!(touch:));
            g
        };
        s.addGestureRecognizer(&g);
        s
    }
}

impl<T:crate::layout::ComputableLayout> crate::layout::ComputableLayout for RenderedOnTapView<T> {
    fn preferred_size(&self, in_frame: &crate::prelude::Size<f64>) -> crate::prelude::Size<Option<f64>> {
        self.0.preferred_size(in_frame)
    }
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        self.0.set_size(to);
        let mut frame = self.1.frame();
        frame.size = to.into();
        self.1.setFrame(frame);
    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        let mut frame = self.1.frame();
        frame.origin = to.into();
        self.1.setFrame(frame);
    }

    fn destroy(&mut self) {
        self.0.destroy();
        unsafe { self.1.removeFromSuperview() }
    }
}

pub struct RenderedOnTapView<Child>(Child,Retained<ClickableContainer>);

impl<T:crate::layout::RenderObject> crate::layout::RenderObject for crate::modifiers::on_tap::OnTapView<T>{
    type Output=RenderedOnTapView<T::Output>;

    fn render(&self, mut data: crate::native::RenderData) -> Self::Output {
        let container = ClickableContainer::new(unsafe { MainThreadMarker::new_unchecked() },self.1.replace(Box::new(||panic!())));
        unsafe { data.real_parent.addSubview(&container) };
        data.real_parent = container.clone().into_super();
        RenderedOnTapView(self.0.render(data),container)
    }
}