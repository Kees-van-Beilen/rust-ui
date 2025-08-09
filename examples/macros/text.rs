#![feature(more_qualified_paths,default_field_values)]
use rust_ui::prelude::*;



#[ui(main)]
struct RootView {

    body:_ = view!{
        VStack {
            HStack {
                Text("A title here").title()
                Spacer()
            }
            HStack {
                Text("All these children are small")
                    .background {
                        HStack {
                            ColorView(bevy_color::palettes::basic::RED.into())
                            ColorView(bevy_color::palettes::basic::BLUE.into())
                        }
                    }
                Text("Except").with_font_size(18.0)
                ColorView(Color::WHITE).frame(Frame::new(26.0, 26.0))
                Text("That one")
                Spacer()

            }
        }
    }
}