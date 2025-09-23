#![feature(more_qualified_paths, default_field_values)]
use rust_ui::prelude::*;

const RED: Color = Color::oklch(0.6, 0.19, 33.88);
const BLUE: Color = Color::oklch(0.6, 0.16, 258.35);
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
                            ColorView(RED)
                            ColorView(BLUE)
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
