#![feature(more_qualified_paths,default_field_values)]
use rust_ui::prelude::*;


#[ui]
struct IncrementorButton {
    #[binding] counter: i32,
    body:_ = view!{
        Button("Click me!") || {
            *counter.get_mut() += 1;
        }
    }
}


#[ui(main)]

struct RootView {
    #[state] counter: i32 = 0,
    body:_ = view!{
        HStack {
            spacing:Some(10.0),
            Spacer()
            IncrementorButton{
                counter:counter.as_binding(data.clone()),
            }
            Text(format!("clicked {counter} time(s)"))
            Spacer()
        }
    }
}



