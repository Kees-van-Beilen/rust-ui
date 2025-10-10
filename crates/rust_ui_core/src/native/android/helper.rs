use std::ffi::CStr;

use crate::native::android::ENV;


pub const RUST_UI_TAG:&CStr = match CStr::from_bytes_with_nul(b"rust-ui\0") {
    Ok(e) => e,
    Err(_) => panic!(),
};

unsafe extern "C" {
    //__android_log_write(int prio, const char *tag, const char *text)
    pub fn __android_log_write(prio:std::ffi::c_int,tag:*const std::ffi::c_char, text: *const std::ffi::c_char);
}
#[macro_export]

macro_rules! get_env {
    () => {
        ENV.with(|e|{
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

