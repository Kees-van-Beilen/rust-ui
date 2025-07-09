#![feature(more_qualified_paths,default_field_values)]
use rust_ui::prelude::*;



#[ui(main)]
struct RootView {
    body:_ = view!{
        Text("Hello world")
    }
}

