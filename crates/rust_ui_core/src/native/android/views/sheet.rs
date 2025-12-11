// use crate::{layout::{ComputableLayout, RenderObject}, prelude::sheet::SheetModalPresenterView, view::state::PartialBinding};

use std::{cell::Cell, mem};

use android2_android::{
    content::Context,
    view::ViewGroup,
    widget::{RelativeLayout, ScrollView},
};
use jni::{JNIEnv, objects::JString};

use crate::{
    android_println,
    layout::{ComputableLayout, Position, RenderObject, Size},
    native::{ActivityExtension, RenderData, android::{callback::CallbackBlockOnDismiss, class_loader::BottomSheetDialog}, helper::get_env},
    prelude::sheet::SheetModalPresenterView,
    retain,
};

fn get_screen_width<'local>(context: &Context, env: &mut JNIEnv<'local>) -> i32 {
    let context: &Context<'_> = unsafe { mem::transmute(context) };
    let resources = context.get_resources(env);
    let metrics = resources.get_display_metrics(env);
    let width = env
        .get_field(&metrics, "widthPixels", "I")
        .unwrap()
        .i()
        .unwrap();
    return width;
}

impl<Sheet: RenderObject + 'static, View: RenderObject> RenderObject
    for SheetModalPresenterView<View, Sheet>
{
    type Output = View::Output;

    fn render(&self, mut data: crate::native::RenderData) -> Self::Output {

        let  mut new_data = data.clone();
        let env = &mut data.jni;
        let context = data.instance.context();

    
        
        let (sheet_content_generator,identity) = self.sheet.as_ref().unwrap();
        
        let mut bm = data.persistent_storage.borrow_mut();
        let prev_state = *(bm.get_or_init_with(*identity, ||false));

        let (sheet,dialog) = bm.get_or_register_gc(*identity, ||{
            let sheet = sheet_content_generator();
            let bottom_dialog = BottomSheetDialog::new(context, env);
            let dialog = bottom_dialog.dialog();
            let binding = self.binding.clone();
            let d = CallbackBlockOnDismiss::new(env, move |env|{
                binding.update_value(false);
                android_println!("swipped away");
            });
                dialog.set_on_dismiss_listener(d.as_ref(), env);
                            let view = RelativeLayout::new_0(context, env);
            let vg: &ViewGroup = view.as_ref();
            let view_ref: &android2_android::view::View = vg.as_ref();
            bottom_dialog.set_content_view(vg, env);

            let w = get_screen_width(context,env);
            new_data.parent = retain!(&view_ref,env);
            new_data.persistent_storage = Default::default();
            new_data.stack = Default::default();
            let mut rendered_child = sheet.render(new_data);
            let preferred = rendered_child.preferred_size(&Size { width: w as f64, height: std::f64::INFINITY });
            let height = preferred.height.unwrap_or(100.0);
            rendered_child.set_size(Size { width: w as f64, height });
            rendered_child.set_position(Position::default());

            let retained: crate::native::helper::Retained<BottomSheetDialog> = retain!(bottom_dialog,env);
            (
                (sheet,retained.clone()),
                move ||{
                    let env= unsafe { get_env() };
                    retained.dialog().dismiss(env);
                }
            )
        });


        let new_state = *self.binding.get();
        if prev_state!=new_state {
            if new_state {
                dialog.dialog().show(env);
            }else {
                dialog.dialog().dismiss(env);
            }
            bm.insert(*identity, new_state);
        }

        bm.garbage_collection_mark_used(*identity);
        drop(bm);
        self.view.render(data)
        // setContentView

    }
}
