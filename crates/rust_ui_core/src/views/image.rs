pub enum ImageHandle {
    Native(crate::native::NativeImageHandle)
}

pub enum ImageScalingMode {
    Fit,
    Fill
}

pub struct ImageView{
    pub(crate) image_handle:ImageHandle,
    pub(crate) scaling_mode:ImageScalingMode
}

impl ImageView {
    pub fn new(path:impl ToString)->Self{
        ImageView {
            image_handle:ImageHandle::Native(crate::native::NativeImageHandle::from_path(path)),
            scaling_mode:ImageScalingMode::Fit
        }
    }

    pub fn fit(mut self)->Self {
        self.scaling_mode = ImageScalingMode::Fit;
        self
    }
    pub fn fill(mut self)->Self {
        self.scaling_mode = ImageScalingMode::Fill;
        self
    }
}