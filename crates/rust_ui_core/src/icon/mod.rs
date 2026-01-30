#![warn(missing_docs)]
//!Text/image embeddable icons, and bindings to the native icon set
//manage system icons
#[derive(Clone)]
/// Icon resources can be embedded in text or used as standalone images
pub enum Icon {
    /// Use the system icon set. Please do not use directly instead use the globals
    /// or create your own global. Because on a few identifiers work cross platform.
    /// - iOS/macOS uses SF Symbols
    System(&'static str),
}
