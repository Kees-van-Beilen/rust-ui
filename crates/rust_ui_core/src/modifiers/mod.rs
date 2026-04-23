#![warn(missing_docs)]

// SPDX-FileCopyrightText: 2026  The rust-ui maintainers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Modifiers are views/virtual views that wrap around an actual view.
//! Modifiers may change resource, introduce new views, or change layout.

pub mod background;
pub mod boxed;
pub mod font;
pub mod frame;
pub mod margin;
pub mod on_appear;
pub mod on_tap;
pub mod sheet;

pub use background::BackgroundModifier;
pub use boxed::BoxedModifier;
pub use font::FontResourceModifier;
pub use frame::{Frame, FrameModifier};
pub use margin::{Margin, MarginModifier};
pub use on_appear::OnAppearModifier;
pub use on_tap::OnTapModifier;
pub use sheet::SheetModifier;
