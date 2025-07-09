#[cfg(any(target_os = "ios", target_os = "macos"))]
mod apple_shared;
#[cfg(target_os = "ios")]
mod ios;
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "ios")]
pub use ios::native::*;
#[cfg(target_os = "macos")]
pub use macos::native::*;
