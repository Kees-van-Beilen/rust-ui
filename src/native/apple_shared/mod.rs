//shared library for apple devices

use objc2_foundation::{NSPoint, NSSize};

impl Into<NSSize> for crate::layout::Size<f64> {
    fn into(self) -> NSSize {
        NSSize {
            width: self.width,
            height: self.height,
        }
    }
}
impl Into<NSPoint> for crate::layout::Position<f64> {
    fn into(self) -> NSPoint {
        NSPoint {
            x: self.x,
            y: self.y,
        }
    }
}
impl From<NSSize> for crate::layout::Size<f64> {
    fn from(value: NSSize) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}
