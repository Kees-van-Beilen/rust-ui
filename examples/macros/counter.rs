#![feature(more_qualified_paths, default_field_values)]

// SPDX-FileCopyrightText: 2026  The rust-ui maintainers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use rust_ui::prelude::*;

#[ui(main)]

struct RootView {
    #[state] counter: i32 = 0,
    body:_ = view!{
        HStack {
            spacing:Some(10.0),
            Spacer()
            Button("Click me!") || {
                *counter.get_mut() += 1;
            }
            Text(format!("clicked {counter} time(s)"))
            Spacer()
        }
    }
}
