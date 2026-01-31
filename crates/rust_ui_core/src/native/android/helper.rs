use std::{ffi::CStr, marker::PhantomData, mem, ops::Deref, ptr::null_mut};

use android2_android::view::View;
use jni::objects::JObject;

use crate::native::ENV;

pub const RUST_UI_TAG: &CStr = match CStr::from_bytes_with_nul(b"rust-ui\0") {
    Ok(e) => e,
    Err(_) => panic!(),
};

pub struct Retained<T: AsRef<JObject<'static>>> {
    pub global: jni::objects::GlobalRef,
    pub __marker: PhantomData<T>,
}
impl<A: AsRef<JObject<'static>>> Clone for Retained<A> {
    fn clone(&self) -> Self {
        Self {
            global: self.global.clone(),
            __marker: self.__marker.clone(),
        }
    }
}
impl AsRef<View<'static>> for Retained<View<'static>> {
    fn as_ref(&self) -> &View<'static> {
        &self
    }
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
impl<T: AsRef<JObject<'static>>> Deref for Retained<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self.global.as_obj()) }
    }
}

unsafe extern "C" {
    //__android_log_write(int prio, const char *tag, const char *text)
    pub fn __android_log_write(
        prio: std::ffi::c_int,
        tag: *const std::ffi::c_char,
        text: *const std::ffi::c_char,
    );
}

#[macro_export]
macro_rules! retain {
    ($x:expr,$env:expr) => {{
        let global = $env.new_global_ref($x).unwrap();
        $crate::native::helper::Retained {
            global,
            __marker: ::std::marker::PhantomData,
        }
    }};
}

/// Get access to the jni env. Accessing the env from an invalid context results in a panic
///
/// SAFETY:
///  - requires main thread
///  - you may not use the reference longer than the lifetime of your code body (no storing the env)
pub unsafe fn get_env() -> &'static mut jni::JNIEnv<'static> {
    unsafe {
        ENV.as_mut().expect("there is currently no java context available. This error can happen when there is a layout update (or interaction with android) that happens in for instance async code (or code that runs not as a result of an android callback) Take a look at all your unsafe code blocks they might be the main cause")
    }
}

pub fn with_env(env: jni::JNIEnv, callback: impl FnOnce(jni::JNIEnv)) {
    unsafe {
        let prev_env = ENV;
        ENV = &env as *const jni::JNIEnv as *mut jni::JNIEnv;
        callback(env.unsafe_clone());
        ENV = prev_env;
        drop(env);
    }
}

pub fn try_with_env(env: jni::JNIEnv, callback: impl FnOnce(jni::JNIEnv)) {
    if !unsafe { ENV.is_null() } {
        with_env(env, callback);
    }
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
