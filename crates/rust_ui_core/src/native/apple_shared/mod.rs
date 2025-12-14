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

impl From<NSSize> for crate::layout::Size<Option<f64>> {
    fn from(value: NSSize) -> Self {
        Self {
            width: Some(value.width),
            height: Some(value.height),
        }
    }
}



#[derive(Clone,Copy)]
struct TrustMeBro<A>(A);
unsafe impl<A> Send for TrustMeBro<A> {}
unsafe impl<A> Sync for TrustMeBro<A> {}

pub fn create_task_flush<A:Send,C:Fn(A)>(sync:C)->impl Fn(A)+Send+Sync{
    // let sync2:B  = unsafe { std::mem::transmute::<C,C>(sync) };
    let sync2 = TrustMeBro(sync);
    move |data:A| {
        dispatch2::run_on_main( |mtm|{
            let r = &sync2;
            (r.0)(data)
        })
    }
}