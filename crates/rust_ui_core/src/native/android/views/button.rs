use std::{
    any::type_name,
    mem,
    sync::{Arc, Mutex},
};

use android2_android::{
    graphics::Typeface,
    view::View,
    widget::{Button, TextView},
};
use android2_java::lang::CharSequence;

use crate::{
    android_println,
    layout::{ComputableLayout, RenderObject, Size},
    native::{
        ActivityExtension,
        android::{
            callback::CallbackBlock,
            views::{delegate_destroy, delegate_set_position, delegate_set_size},
        },
        helper::{Retained, get_env},
    },
    retain,
    views::{FontSize, FontWeight},
};

impl RenderObject for crate::views::Button {
    type Output = NativeButtonView;

    fn render(&self, mut data: crate::native::RenderData) -> Self::Output {
        let env = &mut data.jni;
        let view = Button::new_0(data.instance.context(), env);
        // let str = jni::strings::JNIString::from(self.content);
        let str = env.new_string(&self.label).unwrap();
        let str: android2_java::lang::String = unsafe { mem::transmute(str) };
        let text_view: &TextView = view.as_ref();

        text_view.set_text_0(str.as_ref(), env);
        //https://developer.android.com/reference/android/view/Gravity#CENTER
        text_view.set_gravity(17, env);

        match data.stack.get_resource() {
            Some(FontWeight::Bold) | Some(FontWeight::Semibold) => {
                let typeface = text_view.get_typeface(env);
                text_view.set_typeface_0(&typeface, 1, env);
            }
            _ => {}
        }
        if let Some(FontSize(size)) = data.stack.get_resource() {
            text_view.set_text_size_0(*size as f32, env);
        }
        let view_view: &View = text_view.as_ref();
        // android_println!("try set callback block");
        let cb = self.callback.replace(Box::new(|| panic!()));

        // Todo: thread safety lol
        let callback_block = CallbackBlock::new(env, move |_| {
            // self.callback
            cb();
            // (cb.lock().unwrap())()
            // c();
        });
        // android_println!("set callback block");
        view_view.set_on_click_listener(callback_block.as_ref(), env);

        // env.define_class("RustUITest", loader, buf)
        // env.register_native_methods(class, methods)
        // let typeface = Typeface::

        data.parent.add_view_0(text_view.as_ref(), env);
        NativeButtonView(retain!(view, env))
    }
}

pub struct NativeButtonView(Retained<Button<'static>>);

impl ComputableLayout for NativeButtonView {
    fn preferred_size(
        &self,
        _in_frame: &crate::prelude::Size<f64>,
    ) -> crate::prelude::Size<Option<f64>> {
        android_println!("|>start2 {}", type_name::<Self>());
        let env = unsafe { get_env() };
        let obj_bind = self.0.global.as_obj();

        let text_view: TextView = unsafe { mem::transmute_copy(obj_bind) };
        let view: &View = text_view.as_ref();
        // android_println!("{:?}",env.find_class("name"));
        android_println!("|>measure {:?}", unsafe {
            mem::transmute_copy::<_, jni::sys::jobject>(view)
        });
        view.measure(0, 0, env);
        android_println!("|>measure {:?}", unsafe {
            mem::transmute_copy::<_, jni::sys::jobject>(view)
        });

        let height = view.get_measured_height(env);
        let width = view.get_measured_width(env);
        android_println!("|>end");
        Size {
            width: Some(width as f64),
            height: Some(height as f64),
        }
    }
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        let text_view: &TextView = self.0.as_ref();
        delegate_set_size(text_view, to);
    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        let text_view: &TextView = self.0.as_ref();
        delegate_set_position(text_view, to);
    }

    fn destroy(&mut self) {
        let text_view: &TextView = self.0.as_ref();
        delegate_destroy(text_view);
    }
}
