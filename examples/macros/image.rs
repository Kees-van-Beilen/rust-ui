#![feature(more_qualified_paths, default_field_values)]
use rust_ui::prelude::*;

const RED: Color = Color::oklch(0.6, 0.22, 16.94);

#[ui]
struct ImageDetailView {
    mode:ImageScalingMode = ImageScalingMode::Fill,
    body:_ = view!{
        VStack {
            spacing:Some(10.0),
            Text(match mode {
                ImageScalingMode::Fit => "Fit",
                ImageScalingMode::Fill => "Fill",
            })
            ImageView("cat.png")
                .set_scaling_mode(*mode)
                .background {
                    ColorView(Color::BLACK)
                }
                .margin(Margin::all(3.0))
                .background {
                    ColorView(RED)
                }
                .frame(Frame::new(150.0, 150.0))
        }
    }
}

#[ui(main)]
struct RootView {


    body:_ = view!{
        VStack {
            Spacer()
            HStack {
                Spacer()
                ImageDetailView{
                    mode:Some(ImageScalingMode::Fit)
                }
                Spacer()
                ImageDetailView{
                    mode:Some(ImageScalingMode::Fill)
                }
                Spacer()

            }
            Spacer()
        }.with_font_size(21.0)
    }
}
