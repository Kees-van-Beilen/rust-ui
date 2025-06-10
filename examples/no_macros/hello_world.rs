use rust_ui::prelude::*;

fn main() {
    rust_ui::native::launch_application_with_view(Text {
        content: "Hello world".to_string(),
    });
}
