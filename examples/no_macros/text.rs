use rust_ui::{
    modifiers::{
        font::FontResourceModifier,
        frame::{Frame, FrameModifier},
    },
    prelude::*,
};

fn main() {
    rust_ui::native::launch_application_with_view(VStack {
        spacing: 0.0,
        children: (
            HStack {
                spacing: 0.0,
                children: (
                    Text {
                        content: "A title here".to_string(),
                    }
                    .title(),
                    Spacer,
                ),
            },
            HStack {
                spacing: 0.0,
                children: (
                    Text {
                        content: "All these children are small".to_string(),
                    },
                    Text {
                        content: "Except".to_string(),
                    }
                    .with_font_size(18.0),
                    ColorView(Color::WHITE).frame(Frame::new(26.0, 26.0)),
                    Text {
                        content: "That one".to_string(),
                    },
                    Spacer,
                ),
            }
            .with_font_size(10.0),
        ),
    });
}
