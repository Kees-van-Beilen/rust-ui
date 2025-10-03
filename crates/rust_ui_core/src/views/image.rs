/// An image handle points to an image. Images may be located on disk, bundled into the application or ar part of a global system image collection.
/// You should not construct [`crate::native::NativeImageHandle`] yourself.
pub enum ImageHandle {
    Native(crate::native::NativeImageHandle),
}

/// Image scaling modes dictate how images are scaled in image views.
///
/// ![image demonstrating the difference between the image scaling modes](https://inpolen.nl/profiles/rust-ui/public/example_images/fit-fill-example.png)
///
/// CC0 Picture of _White and Grey Kitten Smelling White Daisy Flower by Alex Bargain_
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum ImageScalingMode {
    /// The picture is scaled down to fit the frame such that entire picture is visible
    #[default]
    Fit,
    /// The picture is scaled up such that it covers the entire frame
    Fill,
}

///
/// This is a wrapper around a native image view.
///
/// ## Example
/// <table style="width:100%">
/// <tr>
/// <td style="border:none">
///
///
/// ```rust
/// use rust_ui::prelude::*;
///
/// #[ui(main)]
/// struct RootView {
///     body:_ = view!{
///        ImageView("assets/demo/cat.png")
///            .fill()
///     }
/// }
/// ```
///
/// </td>
/// <td style="border:none;width:400px"> <img width="400" src="https://inpolen.nl/profiles/rust-ui/public/example_images/image_fill.png"> </td>
/// </tr>
/// </table>
///
pub struct ImageView {
    pub(crate) image_handle: ImageHandle,
    pub(crate) scaling_mode: ImageScalingMode,
}

impl ImageView {
    /// Construct an image view using the specified path.
    pub fn new(path: impl ToString) -> Self {
        ImageView {
            image_handle: ImageHandle::Native(crate::native::NativeImageHandle::from_path(path)),
            scaling_mode: ImageScalingMode::Fit,
        }
    }

    /// Specify the image scaling mode
    pub fn set_scaling_mode(mut self, mode: ImageScalingMode) -> Self {
        self.scaling_mode = mode;
        self
    }

    ///Set the image scaling mode to [`ImageScalingMode::Fit`]. This is already the default image scaling mode.
    /// <table style="width:100%">
    /// <tr>
    /// <td style="border:none">
    ///
    ///
    /// ```rust
    /// use rust_ui::prelude::*;
    ///
    /// #[ui(main)]
    /// struct RootView {
    ///     body:_ = view!{
    ///        ImageView("assets/demo/cat.png")
    ///            .fit()
    ///     }
    /// }
    /// ```
    ///
    /// </td>
    /// <td style="border:none;width:400px"> <img width="400" src="https://inpolen.nl/profiles/rust-ui/public/example_images/image_fit.png"> </td>
    /// </tr>
    /// </table>
    pub fn fit(mut self) -> Self {
        self.scaling_mode = ImageScalingMode::Fit;
        self
    }

    ///Set the image scaling mode to [`ImageScalingMode::Fill`].
    /// <table style="width:100%">
    /// <tr>
    /// <td style="border:none">
    ///
    ///
    /// ```rust
    /// use rust_ui::prelude::*;
    ///
    /// #[ui(main)]
    /// struct RootView {
    ///     body:_ = view!{
    ///        ImageView("assets/demo/cat.png")
    ///            .fill()
    ///     }
    /// }
    /// ```
    ///
    /// </td>
    /// <td style="border:none;width:400px"> <img width="400" src="https://inpolen.nl/profiles/rust-ui/public/example_images/image_fill.png"> </td>
    /// </tr>
    /// </table>
    pub fn fill(mut self) -> Self {
        self.scaling_mode = ImageScalingMode::Fill;
        self
    }
}
