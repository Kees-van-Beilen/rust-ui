// SPDX-FileCopyrightText: 2026  The rust-ui maintainers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use rust_ui::prelude::*;

fn main() {
    rust_ui::native::launch_application_with_view(Text {
        content: "Hello world".to_string(),
    });
}
