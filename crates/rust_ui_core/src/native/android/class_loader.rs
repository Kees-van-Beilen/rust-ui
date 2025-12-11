use std::mem;

use android2_android::{app::Dialog, content::Context, view::View};
use android2_java::lang::{Class, reflect::Constructor};
use jni::{JNIEnv, objects::JObject};

use crate::android_println;

/// Use this to load classes from external libraries. Like material UI
/// 
/// Note that classes uses `.` instead of `/` in namespace separation
/// i.e. `java.lang.String` instead of `java/lang/String`
fn load<'a>(cls:&str,context:&Context<'_>,env:&mut JNIEnv<'a>) -> Class<'a>{
    //the auto generated java method's lifetimes are overconstrained this is needed
    let context:&Context<'_> = unsafe {
        mem::transmute(context)
    };
    let loader = context.get_class_loader(env);
    let str = env.new_string(cls).unwrap();
    let str: android2_java::lang::String = unsafe {
        mem::transmute(str)
    };
    // let a = android_path_to_rust!(java/lang/String);
    let class = loader.load_class_0(&str, env);
    return class;
}

macro_rules! android_path_to_rust {
    (java $(/$x:ident)*) => {
        android2_java$(::$x)*
    };
    (android $(/$x:ident)*) => {
        android2_android$(::$x)*
    };
}

#[repr(transparent)]
pub struct BottomSheetDialog<'local>(JObject<'local>);

impl<'a> AsRef<JObject<'a>> for BottomSheetDialog<'a> {
    fn as_ref(&self) -> &JObject<'a> {
        &self.0
    }
}

impl<'local> BottomSheetDialog<'local> {
    pub fn new(context:&Context,env:&mut JNIEnv<'local>)->Self{
        let args = env.new_object_array(1, "android/content/Context", context).unwrap();
        let cls = load("com.google.android.material.bottomsheet.BottomSheetDialog",context, env);
        let constructors = cls.get_constructors(env);
        let l = env.get_array_length(&constructors).unwrap();
        android_println!("constructors: {}",l);
        let constructor = Constructor::from(env.get_object_array_element(constructors, 0).unwrap());
        let instance: JObject<'_> = unsafe { mem::transmute(constructor.new_instance(&args, env))};
        Self(instance)
    }

    pub fn set_content_view(&self,view:impl AsRef<View<'local>>,env:&mut JNIEnv){
        env.call_method(&self.0, "setContentView", "(Landroid/view/View;)V", &[(&view.as_ref()).into()]).unwrap();
    }
    pub fn dialog(&self)->&Dialog<'local> {
        unsafe {
            mem::transmute::<&BottomSheetDialog,&Dialog>(self)
        }
    }

}