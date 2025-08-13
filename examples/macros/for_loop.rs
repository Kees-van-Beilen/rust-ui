#![feature(more_qualified_paths,default_field_values)]
use rust_ui::prelude::*;


const COLORS:&[Color] = &[
    Color::WHITE,
    Color::BLACK
];

#[ui(main)]
struct RootView {
    #[state] text:String,
    #[state] list:Vec<String>,

    body:_ = view!{
        
       VStack {
            spacing: Some(10.0),
            HStack {
                TextField(bind!(text))
                Button("Add to list") || {
                    list.get_mut().push(text.get().clone());
                    text.get_mut().clear();
                }
            }


            ScrollView {
                y:Some(ScrollBehavior::Scroll),

                VStack {
                    spacing: Some(3.0),
                    for item in list.get().iter() {
                        Text(item)
                            .margin(Margin::all(13.0))
                            .background {
                                ColorView(Color::BLACK)
                            }
                    }
                }
            }


            

        }
    }
}
