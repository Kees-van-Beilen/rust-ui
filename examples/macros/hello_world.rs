#![feature(more_qualified_paths, default_field_values)]

// SPDX-FileCopyrightText: 2026  The rust-ui maintainers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use rust_ui::prelude::*;

#[ui(main)]
struct RootView {
    body:_ = view!{
        Text("Hello world")
    }
}
