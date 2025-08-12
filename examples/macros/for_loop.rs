#![feature(more_qualified_paths,default_field_values)]
use rust_ui::prelude::*;


const COLORS:&[Color] = &[
    Color::WHITE,
    Color::BLACK
];

#[ui(main)]
struct RootView {
    body:_ = view!{
        
       VStack {
            Spacer()
            HStack {
                spacing:Some(21.0),


                Spacer()
                for color in COLORS.iter() {
                        ColorView(*color)
                            .frame(Frame::new(100.0,100.0))
                }
                Spacer()
            }
            Spacer()
        }
    }
}
