use android2_android::{view::ViewGroup, widget::{FrameLayout, RelativeLayout, ScrollView}};

use crate::{layout::ComputableLayout, native::{ActivityExtension, android::views::{delegate_destroy, delegate_set_position, delegate_set_size}, helper::Retained}, retain, view::persistent_storage::PersistentStorageRef, views::{Axis, ScrollBehavior}};

pub struct NativeScrollView<Child: crate::layout::ComputableLayout> {
    view: Retained<ScrollView<'static>>,
    content:Retained<ViewGroup<'static>>,
    axis: Axis,
    child: Child,
}

pub struct ScrollViewStorage {
    storage: PersistentStorageRef,
    view: Retained<ScrollView<'static>>,
    content:Retained<ViewGroup<'static>>,
}

impl <T:crate::layout::RenderObject> crate::layout::RenderObject for crate::views::ScrollView<T> {
    type Output = NativeScrollView<T::Output>;

    fn set_identity(mut self, identity: usize) -> Self {
        self.identity = identity;
        self
    }

    fn render(&self, mut data: crate::native::RenderData) -> Self::Output {
        let identity = self.identity;
        let mut bm = data.persistent_storage.borrow_mut();

        let view = bm.get_or_register_gc(identity, ||{
            let env = &mut data.jni;
            let context = data.instance.context();
            let view = ScrollView::new_0(context, env);
            let fl: &FrameLayout = view.as_ref();
            let vg: &ViewGroup = fl.as_ref();
            data.parent.add_view_0(vg.as_ref(), env);

            let content = RelativeLayout::new_0(context, env);
            let content_vg:&ViewGroup = content.as_ref();
            vg.add_view_0(content_vg.as_ref(), env);

            let retained: Retained<ScrollView> = retain!(view,env);
            let retained_content: Retained<ViewGroup> = retain!(content_vg,env);
            (ScrollViewStorage {
                storage: Default::default(),
                view: retained.clone(),
                content:retained_content.clone()
            },move ||{
                let fl: &FrameLayout = retained.as_ref();
                let vg: &ViewGroup = fl.as_ref();
                delegate_destroy(vg);
            })
        });
        let storage = view.storage.clone();
        let content_view = view.content.clone();
        data.parent = content_view.clone();
        let view = view.view.clone();
        bm.garbage_collection_mark_used(identity);
        drop(bm);
        data.persistent_storage = storage.clone();
        let child = self.child.render(data);

        NativeScrollView {
            view:view,
            content:content_view,
            axis:self.axis,
            child
        }



    }

    
}

impl <T:ComputableLayout> ComputableLayout for NativeScrollView<T> {
    fn set_size(&mut self, to: crate::prelude::Size<f64>) {
        // todo!()
        {
            let fl: &FrameLayout = self.view.as_ref();
            let vg: &ViewGroup = fl.as_ref();
            delegate_set_size(vg, to);
        }
        let mut child_size = to;

        if self.axis.x == ScrollBehavior::Scroll || self.axis.y == ScrollBehavior::Scroll {
            let preferred_size = self.child.preferred_size(&to);
            match (self.axis.x, preferred_size.width) {
                (ScrollBehavior::Scroll, Some(width)) if width > to.width => {
                    child_size.width = width
                }
                _ => {}
            }

            match (self.axis.y, preferred_size.height) {
                (ScrollBehavior::Scroll, Some(height)) if height > to.height => {
                    child_size.height = height
                }
                _ => {}
            }
        }

        // let mut frame = self.content_view.frame();
        delegate_set_size(&*self.content, child_size);
        // println!("RUST_UI SIZE: {:?} {:?}",frame.size,to);

        // self.content_view.setFrame(frame);
        // self.ns_view.setContentSize(frame.size);
        
        self.child.set_size(child_size);

    }

    fn set_position(&mut self, to: crate::prelude::Position<f64>) {
        let fl: &FrameLayout = self.view.as_ref();
        let vg: &ViewGroup = fl.as_ref();
        delegate_set_position(vg, to);
    }

    fn destroy(&mut self) {
        self.child.destroy();
    }
}