//manage system icons
#[derive(Clone)]
pub enum Icon {
    /// Use the system icon set. Please do not use directly instead use the globals
    /// or create your own global. Because on a few identifiers work cross platform.
    /// - iOS/macOS uses SF Symbols
    System(&'static str),
}
