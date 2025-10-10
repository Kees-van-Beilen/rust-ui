use std::mem;

use crate::{android_println, get_env, layout::{ComputableLayout, RenderObject}, native::android::ENV};

pub struct NativeColorView(jni::sys::jobject);

impl RenderObject for crate::views::ColorView {
    type Output=NativeColorView;

    fn render<'a,'jni>(&self, data: crate::native::RenderData<'a,'jni>) -> Self::Output {
        let raw_ptr= ENV.with(move |e|{
            let k = e.borrow();
            let env = unsafe { k.as_mut().unwrap() };

            let view = android2_android::view::View::new_0(&data.context, env);
            let comps = self.0.to_srgba();
            let final_color = 0xff000000u32 as i32 | (((comps.red*255.0) as i32) << 16) | (((comps.green*255.0) as i32) << 8) | (((comps.blue*255.0) as i32) << 0);

            let mut jni = unsafe { env.unsafe_clone() };
            data.parent.add_view_0(&view, &mut jni);
            view.set_background_color(final_color, env);

            // NativeColorView(std::ptr::null_mut())
            view.as_ref().as_raw()
        });
        let view_clone_unsafe:android2_android::view::View = unsafe {
                mem::transmute_copy(&raw_ptr)
        };
        // data.parent.add_view_0(&view_clone_unsafe, env);
        NativeColorView(view_clone_unsafe.as_ref().as_raw())
    }
}
impl ComputableLayout for NativeColorView {
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        let this: android2_android::view::View = unsafe {mem::transmute(self.0)};
        let env = get_env!();
        android_println!("set size color {:?}",to);

        let layout = this.get_layout_params(env);
        env.set_field(&layout, "width", "I", (to.width as i32).into()).unwrap();
        env.set_field(&layout, "height", "I", (to.height as i32).into()).unwrap();
        // let params = this.get_layout_params(env);
        // let params = android2_android::widget::relative_layout::LayoutParams::new_1(to.width as i32, to.height as i32, env);
        // this.set_layout_params(unsafe{mem::transmute(params)}, env);
    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        let this: android2_android::view::View = unsafe {mem::transmute(self.0)};
        let env = get_env!();

        this.set_x(to.x as f32, env);
        this.set_y(to.y as f32, env);
    }

    fn destroy(&mut self) {
        let this: android2_android::view::View = unsafe {mem::transmute(self.0)};
        android_println!("hier gaat iets fout");
        todo!("idk")
    }
}