use std::{cell::OnceCell, mem};

use jni::{descriptors::Desc, NativeMethod};

use crate::{android_println, native::{ACTIVITY, ENV, helper::{Retained, with_env}}, retain};

const CLASSES:&[u8] = include_bytes!("./classes.dex");

thread_local! {
    static CALLBACK_CONSTRUCTOR: OnceCell<Retained<android2_java::lang::reflect::Constructor<'static>>> = OnceCell::new();
    static CALLBACK_ON_DISMISS_CONSTRUCTOR: OnceCell<Retained<android2_java::lang::reflect::Constructor<'static>>> = OnceCell::new();
    static TEXT_WATCHER_CONSTRUCTOR: OnceCell<Retained<android2_java::lang::reflect::Constructor<'static>>> = OnceCell::new();
}

pub fn get_constructor(mut env:&mut jni::JNIEnv,name:&str,methods:&[NativeMethod])->Retained<android2_java::lang::reflect::Constructor<'static>>{
    let object = android2_java::lang::Object::new(env);
    let cls = object.get_class(env);
    let cls_loader = cls.get_class_loader(env);
    let bb = unsafe { env.new_direct_byte_buffer(CLASSES.as_ptr() as *mut u8, CLASSES.len()).unwrap() };
    let bb:android2_java::nio::ByteBuffer = unsafe { mem::transmute(bb) };
    let in_memory_class_loader = android2_dalvik::system::InMemoryDexClassLoader::new_2(&bb, &cls_loader, &mut env);
    let in_memory_class_loader:android2_dalvik::system::BaseDexClassLoader = in_memory_class_loader.into();
    let in_memory_class_loader:android2_java::lang::ClassLoader = in_memory_class_loader.into();
    let str = env.new_string(name).unwrap();
    let str:android2_java::lang::String = unsafe { mem::transmute(str) };

    let rust_test_cls = in_memory_class_loader.load_class_0(&str,&mut env);
    let rust_test_constructors = rust_test_cls.get_constructors(&mut env);
    let jclass:jni::objects::JClass = unsafe { mem::transmute(rust_test_cls) };
    let constructor = env.get_object_array_element(rust_test_constructors, 0).unwrap();
    let constructor:android2_java::lang::reflect::Constructor = unsafe { mem::transmute(constructor) };
    // Desc::<jni::objects::JClass>::lookup(self, _)
    match env.register_native_methods(jclass, methods) {
        Ok(_) => {},
        Err(native_method) => {android_println!("errortje: {}",native_method.to_string())},
    }
    // let empty_array = jni::objects::JObjectArray::default();
    // let rust_test_class_instance = constructor.new_instance(&empty_array, &mut env);
    retain!(constructor,env)
}


pub fn init_callback_class<'jni,'a:'jni>(mut env:&mut jni::JNIEnv<'jni>)->Retained<android2_java::lang::reflect::Constructor<'static>>{
    get_constructor(env,"RustUIOnClickCallback",&[
        jni::NativeMethod {
            name: "onClick".into(),
            sig: "(Landroid/view/View;)V".into(),
            fn_ptr: native_callback as *mut std::ffi::c_void,
        }
    ])
}

pub fn init_on_dismiss<'jni,'a:'jni>(mut env:&mut jni::JNIEnv<'jni>)->Retained<android2_java::lang::reflect::Constructor<'static>>{
    get_constructor(env,"RustUIOnDismiss",&[
        jni::NativeMethod {
            name: "onDismiss".into(),
            sig: "(Landroid/content/DialogInterface;)V".into(),
            fn_ptr: native_callback_on_dismiss as *mut std::ffi::c_void,
        }
    ])
}

pub fn init_text_watcher_class<'jni,'a:'jni>(mut env:&mut jni::JNIEnv<'jni>)->Retained<android2_java::lang::reflect::Constructor<'static>>{
    get_constructor(env,"RustUITextWatcher",&[
        jni::NativeMethod {
            name: "afterTextChanged".into(),
            sig: "(Landroid/text/Editable;)V".into(),
            fn_ptr: text_change as *mut std::ffi::c_void,
        }
    ])
}

pub extern "system" fn text_change<'jni>(
    mut env:jni::JNIEnv<'jni>,
    instance:jni::objects::JObject<'jni>,
    editable:android2_android::text::Editable<'jni>
) {
    android_println!("Text changed event");
    let mut env_clone = unsafe { env.unsafe_clone() };
    // unsafe {
    //     let context = view.get_context(&mut env);
    //     ACTIVITY = mem::transmute(context);
    // }
    let o: &jni::objects::JObject = editable.as_ref();
    
    let cs: &android2_java::lang::CharSequence = editable.as_ref();
    let str = cs.to_string(&mut env);
    let str:jni::objects::JString = unsafe { mem::transmute(str) };
    let str = env.get_string(&str).unwrap();
    let callback: std::sync::MutexGuard<'_, Box<dyn Fn(jni::JNIEnv,jni::strings::JavaStr)+Send>> = unsafe { env.get_rust_field(&instance, "rawFunctionBox").unwrap() };
   
    with_env(env_clone, |env|{callback(env,str)});

    // ENV.replace(&env_clone as *const jni::JNIEnv as *mut jni::JNIEnv);
    // callback(env_clone,str);
    drop(callback);
    android_println!("this being called is bad lol:{:p}",&env);
}

pub extern "system" fn native_callback_on_dismiss<'jni>(
    mut env:jni::JNIEnv<'jni>,
    instance:jni::objects::JObject<'jni>,
    dialog_interface:android2_android::content::DialogInterface<'jni>
) {
    let env_clone = unsafe { env.unsafe_clone() };
    // unsafe {
    //     let context = view.get_context(&mut env);
    //     ACTIVITY = mem::transmute(context);
    // }
    // android_println!("sdf:{:?}",env.find_class("test"));
    let callback: std::sync::MutexGuard<'_, Box<dyn Fn(jni::JNIEnv)+Send>> = unsafe { env.get_rust_field(&instance, "rawFunctionBox").unwrap() };
    with_env(env_clone, |env|{callback(env)});

    // callback(env_clone);
    // drop(callback);
    // android_println!("sdf:{:p}",&env);
    // android_println!("{:p}",&env_clone);

}

pub extern "system" fn native_callback<'jni>(
    mut env:jni::JNIEnv<'jni>,
    instance:jni::objects::JObject<'jni>,
    view:android2_android::view::View<'jni>
) {
    let env_clone = unsafe { env.unsafe_clone() };
    // unsafe {
    //     let context = view.get_context(&mut env);
    //     ACTIVITY = mem::transmute(context);
    // }
    // android_println!("sdf:{:?}",env.find_class("test"));
    let callback: std::sync::MutexGuard<'_, Box<dyn Fn(jni::JNIEnv)+Send>> = unsafe { env.get_rust_field(&instance, "rawFunctionBox").unwrap() };
    with_env(env_clone, |env|{callback(env)});

    // callback(env_clone);
    // drop(callback);
    // android_println!("sdf:{:p}",&env);
    // android_println!("{:p}",&env_clone);

}

#[repr(transparent)]
pub struct CallbackBlock<'local> {
    //Because this data lives on the java side, we have an explicit destructor
    inner:android2_java::lang::Object<'local>
}

impl<'a> AsRef<android2_android::view::view::OnClickListener<'a>> for CallbackBlock<'a> {
    fn as_ref(&self) -> &android2_android::view::view::OnClickListener<'a> {
        unsafe { mem::transmute::<&CallbackBlock,&android2_android::view::view::OnClickListener<'a>>(self) }
    }
}

#[repr(transparent)]
pub struct CallbackBlockOnDismiss<'local> {
    //Because this data lives on the java side, we have an explicit destructor
    inner:android2_java::lang::Object<'local>
}
impl<'a> AsRef<android2_android::content::dialog_interface::OnDismissListener<'a>> for CallbackBlockOnDismiss<'a> {
    fn as_ref(&self) -> &android2_android::content::dialog_interface::OnDismissListener<'a> {
        unsafe { mem::transmute::<&CallbackBlockOnDismiss,&android2_android::content::dialog_interface::OnDismissListener<'a>>(self) }
    }
}

impl<'local> CallbackBlockOnDismiss<'local> {
    pub fn new<'jni:'local>(env:&mut jni::JNIEnv<'jni>,callback:impl Fn(jni::JNIEnv)+'static)->Self{
        CALLBACK_ON_DISMISS_CONSTRUCTOR.with(|constructor|{
            let constructor = constructor.get_or_init(||init_on_dismiss(env));
            let empty_array = jni::objects::JObjectArray::default();
            let instance = constructor.new_instance(&empty_array, env);
            unsafe { 
                let boxed: Box<dyn Fn(jni::JNIEnv<'_>) + 'static> = Box::new(callback);
                // SAFETY:
                // Android runs on a single thread, or at least I hope 🙏
                // There should be a better less error prone way of doing this.
                let fake_box:Box<dyn Fn(jni::JNIEnv) +Send +'static  > = mem::transmute(boxed);
                env.set_rust_field(&instance, "rawFunctionBox", fake_box).unwrap();
            };
            Self {
                inner:instance
            }
        })
    }
}
impl<'local> CallbackBlock<'local> {

    
    pub fn new<'jni:'local>(env:&mut jni::JNIEnv<'jni>,callback:impl Fn(jni::JNIEnv)+'static)->Self{
        CALLBACK_CONSTRUCTOR.with(|constructor|{
            let constructor = constructor.get_or_init(||init_callback_class(env));
            let empty_array = jni::objects::JObjectArray::default();
            let instance = constructor.new_instance(&empty_array, env);
            unsafe { 
                let boxed: Box<dyn Fn(jni::JNIEnv<'_>) + 'static> = Box::new(callback);
                // SAFETY:
                // Android runs on a single thread, or at least I hope 🙏
                // There should be a better less error prone way of doing this.
                let fake_box:Box<dyn Fn(jni::JNIEnv) +Send +'static  > = mem::transmute(boxed);
                env.set_rust_field(&instance, "rawFunctionBox", fake_box).unwrap();
            };
            Self {
                inner:instance
            }
        })
        // let constructor:Retained<android2_java::lang::reflect::Constructor<'static>> = 

        
    }
}

#[repr(transparent)]
pub struct TextChangeListener<'local> {
    //Because this data lives on the java side, we have an explicit destructor
    inner:android2_java::lang::Object<'local>
}

impl<'a> AsRef<android2_android::text::TextWatcher<'a>> for TextChangeListener<'a> {
    fn as_ref(&self) -> &android2_android::text::TextWatcher<'a> {
        unsafe { mem::transmute(self) }
    }
}

impl<'local> TextChangeListener<'local> {
    pub fn new<'jni:'local>(env:&mut jni::JNIEnv<'jni>,callback:impl Fn(jni::JNIEnv,jni::strings::JavaStr)+'static)->Self{
        TEXT_WATCHER_CONSTRUCTOR.with(|constructor|{
            let constructor = constructor.get_or_init(||init_text_watcher_class(env));
            let empty_array = jni::objects::JObjectArray::default();
            let instance = constructor.new_instance(&empty_array, env);
            unsafe { 
                let boxed: Box<dyn Fn(jni::JNIEnv<'_>,jni::strings::JavaStr) + 'static> = Box::new(callback);
                // SAFETY:
                // Android runs on a single thread, or at least I hope 🙏
                // There should be a better less error prone way of doing this.
                let fake_box:Box<dyn Fn(jni::JNIEnv) +Send +'static  > = mem::transmute(boxed);
                env.set_rust_field(&instance, "rawFunctionBox", fake_box).unwrap();
            };
            Self {
                inner:instance
            }
        })
        // let constructor:Retained<android2_java::lang::reflect::Constructor<'static>> = 

        
    }
}