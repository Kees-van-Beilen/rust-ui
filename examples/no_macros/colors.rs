use rust_ui::prelude::*;

fn main() {
    rust_ui::native::launch_application_with_view(VStack {
        spacing: 0.0,
        children: (
            ColorView(Color::oklch(0.68, 0.2, 29.65)),
            HStack {
                spacing: 10.0,
                children: (
                    ColorView(Color::oklch(0.85, 0.21, 148.24)),
                    ColorView(Color::oklch(0.47, 0.22, 263.65)),
                ),
            },
        ),
    });
}
