use std::{ffi::CStr, marker::{ PhantomData}, mem, ops::Deref};

use jni::objects::JObject;




pub const RUST_UI_TAG:&CStr = match CStr::from_bytes_with_nul(b"rust-ui\0") {
    Ok(e) => e,
    Err(_) => panic!(),
};

#[derive(Clone)]
pub struct Retained<T:AsRef<JObject<'static>>>{
    pub global:jni::objects::GlobalRef,
    pub __marker:PhantomData<T>,
}

// impl<'local,T:From<JObject<'local>>+AsRef<JObject<'static>>> Retained<T>  {
//     pub fn new<'a,'jni>(this:T,env:&'a mut jni::JNIEnv<'jni>) -> Self{
//         let global: jni::objects::GlobalRef = env.new_global_ref(this).unwrap();
//         Self {
//             global,
//             __marker: PhantomData,
//         }
//     }
// }
impl<T:AsRef<JObject<'static>>> Deref for Retained<T> {
    type Target=T;

    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self.global.as_obj()) }
    }
}


unsafe extern "C" {
    //__android_log_write(int prio, const char *tag, const char *text)
    pub fn __android_log_write(prio:std::ffi::c_int,tag:*const std::ffi::c_char, text: *const std::ffi::c_char);
}

#[macro_export]
macro_rules! retain {
    ($x:expr,$env:expr) => {
        {
            let global = $env.new_global_ref($x).unwrap();
            $crate::native::helper::Retained {
                global,
                __marker: ::std::marker::PhantomData,
            }
        }
    };
}
/// You can only get access to the env during a layout update
#[macro_export]
macro_rules! get_env {
    () => {
        $crate::native::ENV.with(|e|{
            let k = e.borrow();
            unsafe { k.as_mut().unwrap() }
        })
    };
}

#[macro_export]
macro_rules! android_println {
    ($($x:tt)*) => {
        {
            let str =  format!($($x)*);
            let message = ::std::ffi::CString::new(str).unwrap();
            unsafe {
                $crate::native::helper::__android_log_write(4,$crate::native::helper::RUST_UI_TAG.as_ptr(),message.as_ptr());
            }
        }
       
    };
}

