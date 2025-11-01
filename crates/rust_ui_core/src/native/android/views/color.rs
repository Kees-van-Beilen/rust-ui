use std::{marker::PhantomData, mem, ops::Deref};

use android2_android::view::{View, ViewGroup};
use jni::objects::JObject;

use crate::{android_println, layout::{ComputableLayout, RenderObject}, native::{ ActivityExtension, helper::{Retained, get_env}}, prelude::frame::FrameView, retain};

pub struct NativeColorView(Retained<View<'static>>);


impl RenderObject for crate::views::ColorView {
    type Output=NativeColorView;

    fn render<'a,'jni>(&self, mut data: crate::native::RenderData<'a,'jni>) -> Self::Output {
        // let view = {
        // let (context,env)  = data.context_jni();
        android_println!("start render");
        let env = &mut data.jni;
        let context = data.instance;
        let view: View<'jni> = View::new_0(context.context(), env);
        let comps = self.0.to_srgba();
        let final_color = 0xff000000u32 as i32 | (((comps.red*255.0) as i32) << 16) | (((comps.green*255.0) as i32) << 8) | (((comps.blue*255.0) as i32) << 0);
        view.set_background_color(final_color, env);
        data.parent.add_view_0(&view, env);
        android_println!("end render");
        NativeColorView(retain!(view,env))
    }
}
impl ComputableLayout for NativeColorView {
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        // let this = self.0.d
        android_println!("start size");
        let env = unsafe { get_env() };
        android_println!("got env");

        let layout = self.0.get_layout_params(env);


        match env.set_field(&layout, "width", "I", (to.width as i32).into()) {
            Ok(_) => {},
            Err(e) => {android_println!("error {e}")},
        }
        match  env.set_field(&layout, "height", "I", (to.height as i32).into()) {
            Ok(_) => {},
            Err(e) => {android_println!("error {e}")},
        }

        android_println!("end size");

        // let params = this.get_layout_params(env);
        // let params = android2_android::widget::relative_layout::LayoutParams::new_1(to.width as i32, to.height as i32, env);
        // this.set_layout_params(unsafe{mem::transmute(params)}, env);
    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        // let this: android2_android::view::View = unsafe {mem::transmute(self.0)};
        let env = unsafe { get_env() };

        self.0.set_x(to.x as f32, env);
        self.0.set_y(to.y as f32, env);
    }

    fn destroy(&mut self) {
        // let this: android2_android::view::View = unsafe {mem::transmute(self.0)};
        let env = unsafe { get_env() };
        let parent = self.0.get_parent(env);
        let frame_layout:ViewGroup = unsafe { mem::transmute(parent) };
        frame_layout.remove_view(self.0.deref(), env);
        // android_println!("hier gaat iets fout");
        // todo!("idk")
    }
}