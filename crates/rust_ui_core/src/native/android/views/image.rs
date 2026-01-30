use std::mem;

use android2_android::{
    content::Context,
    graphics::BitmapFactory,
    widget::{self, image_view::ScaleType},
};
use jni::objects::JValue;

use crate::{
    android_println,
    layout::{ComputableLayout, RenderObject},
    native::{
        ACTIVITY, ActivityExtension,
        helper::{Retained, get_env},
    },
    retain,
};

pub struct NativeImageHandle(Retained<android2_android::graphics::Bitmap<'static>>);

impl NativeImageHandle {
    pub fn from_path(path: impl ToString) -> Self {
        // let path = path.to_string();
        // let file_name = objc2_foundation::NSString::from_str(&path);
        // BitmapFactory::new()
        let context: Context = unsafe { mem::transmute(ACTIVITY) };
        let env = unsafe { get_env() };
        let assets = context.get_assets(env);
        let path = env.new_string(path.to_string()).unwrap();
        let obj: &jni::objects::JObject = path.as_ref();
        // android_println!("obj -> {:?}",obj);
        let path: android2_java::lang::String = unsafe { mem::transmute(path) };
        let file = match env.call_method(
            &assets,
            "open",
            "(Ljava/lang/String;)Ljava/io/InputStream;",
            &[(&path).into()],
        ) {
            Ok(e) => e,
            Err(err) => {
                let e = env.exception_occurred().unwrap();
                let cls = env.get_object_class(e).unwrap();

                // android_println!("jva error {:?} {:?}",err,cls);
                panic!("ds")
            }
        };
        // crate::java::io::InputStream::from(res.l().unwrap())
        // let file = assets.open_0(&path, env);

        let bitmap_raw = env
            .call_static_method(
                "android/graphics/BitmapFactory",
                "decodeStream",
                "(Ljava/io/InputStream;)Landroid/graphics/Bitmap;",
                &[(&file).into()],
            )
            .unwrap();

        let bitmap: android2_android::graphics::Bitmap =
            unsafe { mem::transmute(bitmap_raw.l().unwrap()) };

        NativeImageHandle(retain!(bitmap, env))
        // android2_android::graphics::Bitmap::from(

        // unsafe {
        //     NativeImageHandle(
        //         // objc2_app_kit::NSImage::initWithContentsOfFile(
        //         //     objc2_app_kit::NSImage::alloc(),
        //         //     &file_name,
        //         // )
        //         // .unwrap(),
        //     )
        // }
    }
}

pub struct ImageView {
    view: Retained<android2_android::widget::ImageView<'static>>,
}

impl RenderObject for crate::views::ImageView {
    type Output = ImageView;

    fn render(&self, mut data: crate::native::RenderData) -> Self::Output {
        let jni = &mut data.jni;
        let view = android2_android::widget::ImageView::new_0(data.instance.context(), jni);
        data.parent.add_view_0(view.as_ref(), jni);
        match &self.image_handle {
            crate::views::ImageHandle::Native(native_image_handle) => {
                view.set_image_bitmap(&native_image_handle.0, jni);
            }
        }

        match self.scaling_mode {
            crate::views::ImageScalingMode::Fit => {}
            crate::views::ImageScalingMode::Fill => {
                let field = ScaleType::from(
                    jni.get_static_field(
                        "android/widget/ImageView$ScaleType",
                        "CENTER_CROP",
                        "Landroid/widget/ImageView$ScaleType;",
                    )
                    .unwrap()
                    .l()
                    .unwrap(),
                );
                view.set_scale_type(&field, jni);
            }
        }

        // ScaleType::new(arg0, arg1, env)
        ImageView {
            view: retain!(view, jni),
        }
    }
}

impl ComputableLayout for ImageView {
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        super::delegate_set_size(&*self.view, to);
    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        super::delegate_set_position(&*self.view, to);
    }

    fn destroy(&mut self) {
        super::delegate_destroy(&*self.view);
    }
}
