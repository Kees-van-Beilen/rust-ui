#![feature(more_qualified_paths,default_field_values)]
use rust_ui::prelude::*;



#[ui(main)]
struct RootView {
    body:_ = view!{
        VStack {
            Spacer()
            ImageView("./assets/demo/pfp.png")
                .fit()
                .frame(Frame::new(100.0,100.0))
                .on_tap(||{
                    println!("clicked the image")
                })
            Spacer()
        }
    }
}