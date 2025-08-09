use objc2::{rc::Retained, AnyThread, MainThreadMarker};
// use objc2_app_kit::{NSImageScaling, NSImageView};
use objc2_quartz_core::{kCAGravityResizeAspect, kCAGravityResizeAspectFill, CALayer};
use objc2_ui_kit::{UIImage, UIImageView, UIView, UIViewContentMode};

use crate::native::ios::UIViewRepresentable;

// use crate::native::macos::NSViewRepresentable;

pub struct NativeImageHandle(Retained<UIImage>);

impl NativeImageHandle {
    pub fn from_path(path:impl ToString)->Self{
        let path = path.to_string();
        let file_name =  objc2_foundation::NSString::from_str(&path);
        unsafe { 
            NativeImageHandle(
                UIImage::initWithContentsOfFile(UIImage::alloc(), &file_name).unwrap_or(UIImage::new())
                // objc2_app_kit::NSImage::initWithContentsOfFile(objc2_app_kit::NSImage::alloc(), &file_name).unwrap()
            )
        }
    }
}

pub struct ImageView {
    ns_view:Retained<UIImageView>
}

impl crate::layout::RenderObject for crate::views::ImageView {
    type Output=ImageView;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        let view = unsafe {
            let mtm = MainThreadMarker::new().unwrap();

            let view = UIImageView::new(mtm);
            view.setImage(Some(match &self.image_handle {
                crate::views::ImageHandle::Native(native_image_handle) => &native_image_handle.0,
            }));
            view.setContentMode(match self.scaling_mode {
                crate::views::ImageScalingMode::Fit => UIViewContentMode::ScaleAspectFit,
                crate::views::ImageScalingMode::Fill => UIViewContentMode::ScaleAspectFill,
            });

            // view.setFi
            view.setClipsToBounds(true);
            data.real_parent.addSubview(&view);
            view
        };
        ImageView {
            ns_view: view,
        }
    }
}

impl UIViewRepresentable for ImageView {
    fn ui_view(&self) -> &UIView {
        &self.ns_view   
    }
}

