#![feature(more_qualified_paths, default_field_values)]
use rust_ui::prelude::*;

#[ui(main)]
struct RootView {
    #[state] text:String,
    #[state] list:Vec<String>,

    body:_ = view!{
        
       VStack {
            spacing: Some(10.0),

            ColorView(Color::BLACK).frame(Frame::no_preference().height(100.0))
            HStack {
                TextField(bind!(text))
                    .frame(Frame::no_preference().width(300.0))
                Button("Add to list") || {
                    list.get_mut().push(text.get().clone());
                    text.get_mut().clear();
                }
            }


            // ScrollView {
            //     y:Some(ScrollBehavior::Scroll),

                VStack {
                    spacing: Some(3.0),
                    for item in list.get().iter() {
                        Text(item)
                            .margin(Margin::all(13.0))
                            .background {
                                ColorView(Color::oklcha(1.0, 0.5, 20.0, 1.0))
                            }
                        
                    }
                }
            // }


            

        }
    }
}
