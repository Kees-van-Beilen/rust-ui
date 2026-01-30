use crate::{
    android_println,
    layout::RenderObject,
    views::{TextField, textfield::TextEditor},
};

// pub struct MultilineTextEditor{
//     inner:super::textfield::NativeTextView
// }

impl RenderObject for TextEditor {
    type Output = super::textfield::NativeTextView;

    fn set_identity(mut self, identity: usize) -> Self {
        self.identity = Some(identity);
        self
    }

    fn render(&self, data: crate::native::RenderData) -> Self::Output {
        let mut jni = unsafe { data.jni.unsafe_clone() };

        android_println!(">> id: {:?}", self.identity);
        let out = TextField {
            text_binding: self.text_binding.clone_box(),
            identity: self.identity,
        }
        .render(data);
        //https://developer.android.com/reference/android/text/InputType#TYPE_CLASS_TEXT
        out.0.set_input_type(131072 | 1, &mut jni);
        out
    }
}
