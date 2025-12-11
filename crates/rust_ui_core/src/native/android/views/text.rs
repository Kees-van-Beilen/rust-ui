use std::{any::type_name, mem};

use android2_android::{graphics::Typeface, view::View, widget::TextView};
use android2_java::lang::CharSequence;

use crate::{android_println, layout::{ComputableLayout, RenderObject, Size}, native::{ActivityExtension, android::views::{delegate_destroy, delegate_set_position, delegate_set_size}, helper::{Retained, get_env}}, retain, views::{FontSize, FontWeight}};

impl RenderObject for crate::views::Text {
    type Output=NativeTextView;

    fn render(&self, mut data: crate::native::RenderData) -> Self::Output {
        let env = &mut data.jni;
        let view = TextView::new_0(data.instance.context(), env);
        // view.set_line_break_style(arg0, env);
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


    fn preferred_size(&self, in_frame: &crate::prelude::Size<f64>) -> crate::prelude::Size<Option<f64>> {
        android_println!("|>start {} {:?}",type_name::<Self>(),in_frame);
        let env = unsafe { get_env() };
        let view:&View = self.0.as_ref();
        /*
        private static final int MODE_SHIFT = 30;
        private static final int MODE_MASK  = 0x3 << MODE_SHIFT;
         public static final int AT_MOST     = 2 << MODE_SHIFT;
        public static int makeMeasureSpec(@IntRange(from = 0, to = (1 << MeasureSpec.MODE_SHIFT) - 1) int size,
                                          @MeasureSpecMode int mode) {
            if (sUseBrokenMakeMeasureSpec) {
                return size + mode;
            } else {
                return (size & ~MODE_MASK) | (mode & MODE_MASK);
            }
        }

         */
        let max_width:i32 = (in_frame.width as i32) & !(0x3<<30) | ((2<<30)&(0x3<<30));
        let max_height:i32 = (in_frame.height as i32) & !(0x3<<30) | ((2<<30)&(0x3<<30));
        view.measure(max_width,max_height, env);
        let height = view.get_measured_height(env);
        let width = view.get_measured_width(env);
        android_println!("|>end {} {width} {height}",type_name::<Self>());
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