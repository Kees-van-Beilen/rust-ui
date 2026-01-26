use std::{mem, ops::Deref};

use android2_android::{graphics::Typeface, view::View, widget::{EditText, TextView}};
use android2_java::lang::CharSequence;

use crate::{android_println, layout::{ComputableLayout, RenderObject, Size}, native::{ActivityExtension, android::{self, callback::TextChangeListener, views::{delegate_destroy, delegate_set_position, delegate_set_size}}, helper::{Retained, get_env}}, retain, views::{FontSize, FontWeight}};

impl RenderObject for crate::views::TextField {
    type Output=NativeTextView;

    fn set_identity(mut self, identity: usize) -> Self {
        self.identity = Some(identity);
        self
    }

    fn render(&self, mut data: crate::native::RenderData) -> Self::Output {
        
        // let typeface = Typeface::
        let mut borrow = data.persistent_storage.borrow_mut();
        let i = borrow.get_or_register_gc(self.identity.unwrap(), ||{
            let env = &mut data.jni;
            let view = EditText::new_0(data.instance.context(), env);
            // let str = jni::strings::JNIString::from(self.content);
            let str = env.new_string(&**self.text_binding.get()).unwrap();
            let str:android2_java::lang::String = unsafe { mem::transmute(str) };
            let text_view: &TextView = view.as_ref();

            //https://developer.android.com/reference/android/view/Gravity#CENTER
            text_view.set_gravity(17, env);
            let binding = self.text_binding.clone_box();
            let listener = TextChangeListener::new(env, move |_,s|{
                let str:String = s.into();
                binding.update_value(str);
                // android_println!("text: {}",str);
            });
            text_view.add_text_changed_listener(listener.as_ref(), env);
            match data.stack.get_resource() {
                Some(FontWeight::Bold) |
                Some(FontWeight::Semibold) => {
                    let typeface = text_view.get_typeface(env);
                    text_view.set_typeface_0(&typeface, 1, env);
                }
                _=>{}
            }
            if let Some(FontSize(size)) = data.stack.get_resource() {
                text_view.set_text_size_0(*size as f32, env);
            }
            android_println!("habhbah1 {:?}",unsafe {mem::transmute_copy::<_,*mut jni::sys::jobject>(text_view)});
            data.parent.add_view_0(text_view.as_ref(), env);
            let retained: Retained<TextView<'static>> = retain!(view,env);
            // android_println!("habhbah2 {:?}",unsafe {mem::transmute_copy::<_,*mut jni::sys::jobject>(&retained.deref())});
            // retained.set_text_0(str.as_ref(), env);
            // android_println!("error {:?}",env.exception_check());
            // android_println!("habhbah2 {:?}",retained.global.as_obj());
            // android_println!("habhbah4 {:?}",unsafe {mem::transmute_copy::<_,*mut jni::sys::jobject>(retained.deref())});


            (retained,||panic!())
        }).clone();
        let env = &mut data.jni;
        let str = env.new_string(&**self.text_binding.get()).unwrap();
        let str:android2_java::lang::String = unsafe { mem::transmute(str) };
        i.set_text_0(str.as_ref(), env);
        android_println!("habhbah3 {:?}",unsafe {mem::transmute_copy::<_,jni::sys::jobject>(i.deref())});
        // drop(borrow);
        borrow.garbage_collection_mark_used(self.identity.unwrap());

        NativeTextView(i)
    }
}

pub struct NativeTextView(pub (crate)Retained<TextView<'static>>);

impl ComputableLayout for NativeTextView {


    fn preferred_size(&self, _in_frame: &crate::prelude::Size<f64>) -> crate::prelude::Size<Option<f64>> {
        android_println!("hoi");
        let env = unsafe { get_env() };
        android_println!("habhbah ---> {:?}",unsafe {mem::transmute_copy::<_,jni::sys::jobject>(&*self.0)});
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
        // delegate_destroy(&*self.0);
    }
}