#![feature(more_qualified_paths,default_field_values)]
use rust_ui::prelude::*;



#[ui(main)]
struct RootView {

    #[state] current_img: &'static str = "./assets/demo/pfp.png",

    body:_ = view!{
        VStack {
            Spacer()
            ImageView(current_img.get())
                .fit()
                .frame(Frame::new(100.0,100.0))
                .on_tap || {
                    *current_img.get_mut() = "./assets/demo/last_day.png";
                    println!("clicked the image")
                }
            Spacer()
        }
    }
}