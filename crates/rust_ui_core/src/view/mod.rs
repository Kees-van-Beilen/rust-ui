#![warn(missing_docs)]

// SPDX-FileCopyrightText: 2026  The rust-ui maintainers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A collection of a datastructures and methods needed to manage and create views internals.
pub mod collection;
pub mod dyn_render;
pub mod mutable;
pub mod persistent_storage;
pub mod resources;
pub mod state;
pub mod task;
pub mod virtual_layout;