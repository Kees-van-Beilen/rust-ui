#![feature(more_qualified_paths,default_field_values)]
use rust_ui::prelude::*;



#[ui(main)]
struct RootView {
    #[state] counter: i32 = 0,
    body:_ = view!{
        HStack {
            spacing:Some(10.0),
            Spacer()
            Button("Click me!") || {
                *counter.get_mut() += 1;
            }
            Text(format!("clicked {} time(s)",counter.get()))
            Spacer()
        }
    }
}

