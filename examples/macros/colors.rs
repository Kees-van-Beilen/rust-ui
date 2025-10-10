#![feature(more_qualified_paths, default_field_values)]
use rust_ui::prelude::*;
use rust_ui_core::android_println;

#[ui(main)]
struct RootView {
    body:_ = view!{
        VStack {
            ColorView(Color::oklch(0.68, 0.2, 29.65))
            HStack {
                spacing: Some(10.0),
                ColorView(Color::oklch(0.85, 0.21, 148.24))
                ColorView(Color::oklch(0.47, 0.22, 263.65))
            }
        }
    }
}

// #[unsafe(no_mangle)]
// pub extern "system" fn Java_com_example_myapplication_MainActivity_mainEntry<'local>(
//     env: ::rust_ui::native::jni::JNIEnv<'local>,
//     instance: ::rust_ui::native::jni::objects::JObject<'local>,
// ) {
//     // ::rust_ui::native::helper
//     android_println!("end my suffering");
//     // ::rust_ui::native::launch_application_with_view(RootView::new(<<RootView as ::rust_ui::PartialInitialisable>::PartialInit as ::std::default::Default>::default()),env,instance,)
// }