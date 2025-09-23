//custom view class that captures a on click event
use crate::native::macos::nsview_setposition;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, rc::Retained};
use objc2_app_kit::NSEvent;
use objc2_app_kit::NSView;
use objc2_foundation::{MainThreadMarker, NSObjectProtocol};

pub struct ClickableContainerIVars {
    /// Callback to a in rust defined function
    /// TODO: objc2 doesn't support generics in classes
    callback: Box<dyn Fn()>,
}
define_class!(
    #[unsafe(super = NSView)]
    #[thread_kind = MainThreadOnly]
    #[name = "RustButton"]
    #[ivars = ClickableContainerIVars]
    pub struct ClickableContainer;

    // SAFETY: `NSObjectProtocol` has no safety requirements.
    unsafe impl NSObjectProtocol for ClickableContainer {}

    impl ClickableContainer {
        ///SAFETY: obj-c function selector is correct (takes no args)
        /// - (void) mouseDown: (NSEvent*) theEvent;
        #[unsafe(method(mouseDown:))]
        fn mouse_down(&self,_event:&NSEvent) {
            (self.ivars().callback)();
        }
    }

);
impl ClickableContainer {
    pub fn new(mtm: MainThreadMarker, callback: Box<dyn Fn()>) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(ClickableContainerIVars { callback });
        // SAFETY: The signature of `NSObject`'s `init` method is correct.
        let s: Retained<ClickableContainer> = unsafe { msg_send![super(this), init] };
        s
    }
}

impl<T: crate::layout::ComputableLayout> crate::layout::ComputableLayout for RenderedOnTapView<T> {
    fn preferred_size(
        &self,
        in_frame: &crate::prelude::Size<f64>,
    ) -> crate::prelude::Size<Option<f64>> {
        self.0.preferred_size(in_frame)
    }
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        self.0.set_size(to);
        unsafe { self.1.setFrameSize(to.into()) };
    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        nsview_setposition(&self.1, to.into());
        // unsafe { self.1.setFrameOrigin(to.into()) };
    }

    fn destroy(&mut self) {
        self.0.destroy();
        unsafe { self.1.removeFromSuperview() }
    }
}

pub struct RenderedOnTapView<Child>(Child, Retained<ClickableContainer>);

impl<T: crate::layout::RenderObject> crate::layout::RenderObject
    for crate::modifiers::on_tap::OnTapView<T>
{
    type Output = RenderedOnTapView<T::Output>;

    fn render(&self, mut data: crate::native::RenderData) -> Self::Output {
        let container = ClickableContainer::new(
            unsafe { MainThreadMarker::new_unchecked() },
            self.1.replace(Box::new(|| panic!())),
        );
        unsafe { data.real_parent.addSubview(&container) };
        data.real_parent = container.clone().into_super();
        RenderedOnTapView(self.0.render(data), container)
    }
}
