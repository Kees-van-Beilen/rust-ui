use std::any::{type_name, Any};

use crate::{android_println, native::helper::get_env};

mod text;
mod color;
mod button;
mod textfield;

pub fn delegate_set_size<T:AsRef<android2_android::view::View<'static>>+Any>(this:&T,to:crate::layout::Size<f64>){
    android_println!("set size del {}",type_name::<T>());
    let env = unsafe { get_env() };
    let layout = this.as_ref().get_layout_params(env);
    match env.set_field(&layout, "width", "I", (to.width as i32).into()) {
            Ok(_) => {},
            Err(e) => {crate::android_println!("Could not set layout width (child is floating) {e}")},
    }
    match  env.set_field(&layout, "height", "I", (to.height as i32).into()) {
        Ok(_) => {},
        Err(e) => {crate::android_println!("Could not set layout height (child is floating)  {e}")},
    }
}

pub fn delegate_set_position(this:impl AsRef<android2_android::view::View<'static>>,to:crate::layout::Position<f64>){
    let env = unsafe { get_env() };
    this.as_ref().set_x(to.x as f32, env);
    this.as_ref().set_y(to.y as f32, env);
}
pub fn delegate_destroy(this:impl AsRef<android2_android::view::View<'static>>){
     let env = unsafe { get_env() };
    let parent = this.as_ref().get_parent(env);
    let frame_layout:android2_android::view::ViewGroup = unsafe { std::mem::transmute(parent) };
    frame_layout.remove_view(this.as_ref(), env);
}