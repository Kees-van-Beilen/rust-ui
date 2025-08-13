use objc2::{rc::Retained, AnyThread, MainThreadMarker};
use objc2_quartz_core::{kCAGravityResizeAspect, kCAGravityResizeAspectFill, CALayer};

use crate::native::macos::NSViewRepresentable;

pub struct NativeImageHandle(Retained<objc2_app_kit::NSImage>);

impl NativeImageHandle {
    pub fn from_path(path:impl ToString)->Self{
        let path = path.to_string();
        let file_name =  objc2_foundation::NSString::from_str(&path);
        unsafe { 
            NativeImageHandle(
                objc2_app_kit::NSImage::initWithContentsOfFile(objc2_app_kit::NSImage::alloc(), &file_name).unwrap()
            )
        }
    }
}

pub struct ImageView {
    ns_view:Retained<objc2_app_kit::NSView>
}

impl crate::layout::RenderObject for crate::views::ImageView {
    type Output=ImageView;

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        let view = unsafe {
            let mtm = MainThreadMarker::new().unwrap();

            let view = objc2_app_kit::NSView::new(mtm) ;
            let layer = CALayer::new();
            layer.setContentsGravity(match self.scaling_mode {
                crate::views::ImageScalingMode::Fit => kCAGravityResizeAspect,
                crate::views::ImageScalingMode::Fill => kCAGravityResizeAspectFill,
            });
            // layer.set
            layer.setContents(Some(match &self.image_handle {
                crate::views::ImageHandle::Native(native_image_handle) => &native_image_handle.0,
            }));
            view.setLayer(Some(&layer));
            view.setWantsLayer(true);
            view.setClipsToBounds(true);

            // view.setFi
            data.real_parent.addSubview(&view);
            view
        };
        ImageView {
            ns_view: view,
        }
    }
}

impl NSViewRepresentable for ImageView {
    fn ns_view(&self) -> &objc2_app_kit::NSView {
        &self.ns_view
    }
}

