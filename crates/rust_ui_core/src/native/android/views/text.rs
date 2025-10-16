use std::mem;

use android2_android::{graphics::Typeface, view::View, widget::TextView};
use android2_java::lang::CharSequence;

use crate::{android_println, get_env, layout::{ComputableLayout, RenderObject, Size}, native::{android::views::{delegate_destroy, delegate_set_position, delegate_set_size}, helper::Retained, ActivityExtension}, retain, views::{FontSize, FontWeight}};

impl RenderObject for crate::views::Text {
    type Output=NativeTextView;

    fn render(&self, mut data: crate::native::RenderData) -> Self::Output {
        let env = &mut data.jni;
        let view = TextView::new_0(data.instance.context(), env);
        // let str = jni::strings::JNIString::from(self.content);
        let str = env.new_string(&self.content).unwrap();
        let str:android2_java::lang::String = unsafe { mem::transmute(str) };
        view.set_text_0(str.as_ref(), env);
        //https://developer.android.com/reference/android/view/Gravity#CENTER
        view.set_gravity(17, env);
        
        match data.stack.get_resource() {
            Some(FontWeight::Bold) |
            Some(FontWeight::Semibold) => {
                let typeface = view.get_typeface(env);
                view.set_typeface_0(&typeface, 1, env);
            }
            _=>{}
        }
        if let Some(FontSize(size)) = data.stack.get_resource() {
            view.set_text_size_0(*size as f32, env);
        }
        // let typeface = Typeface::

        data.parent.add_view_0(view.as_ref(), env);
        NativeTextView(retain!(view,env))
    }
}

pub struct NativeTextView(Retained<TextView<'static>>);

impl ComputableLayout for NativeTextView {


    fn preferred_size(&self, _in_frame: &crate::prelude::Size<f64>) -> crate::prelude::Size<Option<f64>> {
        let env = get_env!();
        let view:&View = self.0.as_ref();
        view.measure(0, 0, env);
        let height = view.get_measured_height(env);
        let width = view.get_measured_width(env);
        Size {
            width: Some(width as f64),
            height: Some(height as f64),
        }
    }
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        delegate_set_size(&*self.0, to);
    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        delegate_set_position(&*self.0, to);

    }

    fn destroy(&mut self) {
        delegate_destroy(&*self.0);
    }
}