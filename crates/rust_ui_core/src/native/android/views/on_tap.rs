use std::mem;

use android2_android::{view::{View, ViewGroup}, widget::RelativeLayout};

use crate::{android_println, layout::{ComputableLayout, Position}, native::{ActivityExtension, android::callback::CallbackBlock, helper::Retained}, prelude::frame::FrameView, retain};

pub struct RenderedOnTapView<Child>(Child, Retained<ViewGroup<'static>>);

pub struct ViewRef<'a>(&'a View<'a>);
impl<'a> AsRef<View<'static>> for ViewRef<'a> {
    fn as_ref(&self) -> &View<'static> {
        unsafe { mem::transmute(&self.0) }
    }
}

impl<Child:ComputableLayout> ComputableLayout for RenderedOnTapView<Child> {
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        // android_println!("set size {:?}",to);
        self.0.set_size(to);
        super::delegate_set_size(&*self.1, to);
    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        self.0.set_position(Position::default());
        super::delegate_set_position(&*self.1, to);
    }

    fn destroy(&mut self) {
        self.0.destroy();
        super::delegate_destroy(&*self.1);
    }
    fn preferred_size(&self, in_frame: &crate::prelude::Size<f64>) -> crate::prelude::Size<Option<f64>> {
        self.0.preferred_size(in_frame)
    }
}

impl<T: crate::layout::RenderObject> crate::layout::RenderObject
    for crate::modifiers::on_tap::OnTapView<T>
{
    type Output = RenderedOnTapView<T::Output>;

    fn render(&self, mut data: crate::native::RenderData) -> Self::Output {
        let env = &mut data.jni;
        let cb = self.1.replace(Box::new(|| panic!()));
        let block = CallbackBlock::new(env, move |_|{
            cb()
        });
        let container = RelativeLayout::new_0(data.instance.context(), env);
        let view_group:&ViewGroup = container.as_ref(); 
        let view: &View = view_group.as_ref();
        view.set_on_click_listener(block.as_ref(), env);
        data.parent.add_view_0(&view,env);
        let retained_container: Retained<ViewGroup<'static>> = retain!(container,env);
        data.parent = retained_container.clone();
        RenderedOnTapView(self.0.render(data), retained_container)
    }
}
