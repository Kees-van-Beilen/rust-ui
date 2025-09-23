use rust_ui::{
    modifiers::{
        frame::{Frame, FrameModifier},
        on_tap::OnTapModifier,
    },
    prelude::*,
};

fn main() {
    rust_ui::native::launch_application_with_view(VStack {
        spacing: 0.0,
        children: (
            Spacer,
            ImageView::new("./assets/demo/pfp.png")
                .fit()
                .frame(Frame::new(100.0, 100.0))
                .on_tap(|| {
                    println!("clicked the image");
                }),
            Spacer,
        ),
    });
}
