#![warn(missing_docs)]
#![doc(html_logo_url = "https://inpolen.nl/profiles/rust-ui/public/assets/ui-icon-dark.svg")]
//! <p align="center">
//! <img src="https://inpolen.nl/profiles/rust-ui/public/assets/logo-dark.svg">
//! </p>
//!
//! Rust-ui is an open-source cross-platform ui-framework built in Rust, with a focus on creating mobile user interfaces and enhancing developer productivity.
//!
//! ## Example
//! This is a simple "Hello world" app:
//! ```
//! #![feature(more_qualified_paths,default_field_values)]
//! use rust_ui::prelude::*;
//!
//! #[ui(main)]
//! struct RootView {
//!    body:_ = view!{
//!       Text("Hello world")
//!    }
//! }
//! ```
//! Even in this simple hello world app, a lot is actually going on.
//!
//!
//! ## Getting started
//! In this section we'll walk you through everything you have to do to add `rust-ui` to your project. We assume you already have `rustup` installed.
//! ### Using nightly rust
//! Whilst it is not required to use nightly rust, it is highly recommended as it allows you to use the powerful Rust-ui macro system.
//! Install the nightly toolchain, if you haven't done so already.
//! ```sh
//! rustup toolchain install nightly
//! ```
//! You may make the nightly toolchain your default toolchain using `rustup default nightly` or by adding a [`rust-toolchain.toml`](https://rust-lang.github.io/rustup/overrides.html) file to your project.
//! The nightly features rust-ui depends on are bound to be added in Rust in the coming year, as both features are complete but lack proper syn/rust-analyzer support.
//! ### Adding `rust-ui`
//! Rust-ui is in a very volatile state, therefore is unavailable on crate.io, to add rust-ui use:
//! ```sh
//! cargo add --git https://github.com/Kees-van-Beilen/rust-ui.git
//! ```
//! ### Hello world
//! Now copy the hello world example including the `#![feature(more_qualified_paths,default_field_values)]` part at top.
//! If you are on macOS you may now run `cargo +nightly run` to build and run your project.
//!
//!
pub use rust_ui_core::{PartialInitialisable, layout, modifiers, native, view, views};
/// use rust_ui::prelude::*; to import everything you need to create an rust-ui application
pub mod prelude {
    pub use rust_ui_core::prelude::*;
    pub use rust_ui_macro::*;
}
