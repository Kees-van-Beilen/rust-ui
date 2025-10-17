use std::{cell::OnceCell, mem};

use jni::descriptors::Desc;

use crate::{android_println, native::{helper::Retained, ACTIVITY, ENV}, retain};

const CLASSES:&[u8] = include_bytes!("./classes.dex");

thread_local! {
    static CALLBACK_CONSTRUCTOR: OnceCell<Retained<android2_java::lang::reflect::Constructor<'static>>> = OnceCell::new();
}


pub fn init_callback_class<'jni,'a:'jni>(mut env:&mut jni::JNIEnv<'jni>)->Retained<android2_java::lang::reflect::Constructor<'static>>{
    let object = android2_java::lang::Object::new(env);
    let cls = object.get_class(env);
    let cls_loader = cls.get_class_loader(env);
    let bb = unsafe { env.new_direct_byte_buffer(CLASSES.as_ptr() as *mut u8, CLASSES.len()).unwrap() };
    let bb:android2_java::nio::ByteBuffer = unsafe { mem::transmute(bb) };
    let in_memory_class_loader = android2_dalvik::system::InMemoryDexClassLoader::new_2(&bb, &cls_loader, &mut env);
    let in_memory_class_loader:android2_dalvik::system::BaseDexClassLoader = in_memory_class_loader.into();
    let in_memory_class_loader:android2_java::lang::ClassLoader = in_memory_class_loader.into();
    let str = env.new_string("RustUIOnClickCallback").unwrap();
    let str:android2_java::lang::String = unsafe { mem::transmute(str) };

    let rust_test_cls = in_memory_class_loader.load_class_0(&str,&mut env);
    let rust_test_constructors = rust_test_cls.get_constructors(&mut env);
    let jclass:jni::objects::JClass = unsafe { mem::transmute(rust_test_cls) };
    let constructor = env.get_object_array_element(rust_test_constructors, 0).unwrap();
    let constructor:android2_java::lang::reflect::Constructor = unsafe { mem::transmute(constructor) };
    // Desc::<jni::objects::JClass>::lookup(self, _)
    match env.register_native_methods(jclass, &[
            jni::NativeMethod {
                name: "onClick".into(),
                sig: "(Landroid/view/View;)V".into(),
                fn_ptr: native_callback as *mut std::ffi::c_void,
            }
        ]) {
        Ok(_) => {},
        Err(native_method) => {android_println!("errortje: {}",native_method.to_string())},
    }
    // let empty_array = jni::objects::JObjectArray::default();
    // let rust_test_class_instance = constructor.new_instance(&empty_array, &mut env);
    retain!(constructor,env)
}

pub extern "system" fn native_callback<'jni>(
    mut env:jni::JNIEnv<'jni>,
    instance:jni::objects::JObject<'jni>,
    view:android2_android::view::View<'jni>
) {
    let env_clone = unsafe { env.unsafe_clone() };
    unsafe {
        let context = view.get_context(&mut env);
        ACTIVITY = mem::transmute(context);
    }
    let callback: std::sync::MutexGuard<'_, Box<dyn Fn(jni::JNIEnv)+Send>> = unsafe { env.get_rust_field(&instance, "rawFunctionBox").unwrap() };
    ENV.replace(&env_clone as *const jni::JNIEnv as *mut jni::JNIEnv);
    callback(env_clone);
}

#[repr(transparent)]
pub struct CallbackBlock<'local> {
    //Because this data lives on the java side, we have an explicit destructor
    inner:android2_java::lang::Object<'local>
}

impl<'a> AsRef<android2_android::view::view::OnClickListener<'a>> for CallbackBlock<'a> {
    fn as_ref(&self) -> &android2_android::view::view::OnClickListener<'a> {
        unsafe { mem::transmute(self) }
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