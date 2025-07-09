#![feature(more_qualified_paths,default_field_values)]
use rust_ui::prelude::*;



#[ui(main)]
struct RootView {
    body:_ = view!{
        VStack {
            ColorView(Color::oklch(0.68, 0.2, 29.65))
            HStack {
                spacing: Some(10.0),
                ColorView(Color::oklch(0.85, 0.21, 148.24))
                ColorView(Color::oklch(0.47, 0.22, 263.65))
            }
        }
    }
}