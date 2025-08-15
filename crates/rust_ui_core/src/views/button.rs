use std::cell::RefCell;

///
/// This is a wrapper around a native button view. Therefore its appearance might differ depending on the situation. Buttons have a string label and callback, which is called whenever the button is pressed.
/// ## Example
/// <table>
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
///         Button("Click me") || {
///             println!("The button has been pressed")
///         }
///     }
/// }
/// ```
/// 
/// </td>
/// <td style="border:none"> <img width="400" src="https://inpolen.nl/profiles/rust-ui/public/example_images/button.png"> </td>
/// </tr>
/// </table>
/// 
#[doc(alias = "ButtonView")]
pub struct Button {
    pub label: String,
    pub callback: RefCell<Box<dyn Fn()>>,
}

pub trait ButtonPartialInit {
    fn init(self)->Button;
}


impl Button {
    ///
    /// Create a new button using a title and callback
    /// 
    pub fn create(title:impl ToString, callback:impl Fn()+'static)->Self{
        Button { label: title.to_string(), callback: RefCell::new(Box::new(callback)) }
    }
    ///
    /// Create a new button with an empty callback
    /// 
    pub fn new(init:impl ToString) ->Self {
        Self::create(init, ||{})
    }


    ///
    /// Use in the `#[ui]` macro to assign the button a callback. Do not call manually.
    /// 
    #[doc(hidden)]
    pub fn with_capture_callback(mut self,callback:impl Fn()+'static,_identity:usize)->Self {
        self.callback = RefCell::new(Box::new(callback));
        self
    }
}