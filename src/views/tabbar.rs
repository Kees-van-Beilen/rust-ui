use crate::{
    icon::Icon,
    layout::{ComputableLayout, RenderObject},
    native::RenderData,
};

/// A tabbar is very os depended
/// On mobile devices it will look like a bar at the bottom
/// Whilst on Macos and iPadOS it may take the form of a sidebar
pub struct TabBar<T: TabGroup> {
    pub active_tab: usize,
    pub children: T,
}
pub trait TabGroup {
    fn for_each_render(
        &self,
        f: impl FnMut(RenderedTabData) -> RenderData,
    ) -> Vec<Box<dyn ComputableLayout>>;
}
impl<A: RenderObject + 'static> TabGroup for (Tab<A>,) {
    fn for_each_render(
        &self,
        mut f: impl FnMut(RenderedTabData) -> RenderData,
    ) -> Vec<Box<dyn ComputableLayout>> {
        vec![Box::new(self.0.content.render(f((&self.0).into())))]
    }
}
impl<A: RenderObject + 'static, B: RenderObject + 'static> TabGroup for (Tab<A>, Tab<B>) {
    fn for_each_render(
        &self,
        mut f: impl FnMut(RenderedTabData) -> RenderData,
    ) -> Vec<Box<dyn ComputableLayout>> {
        vec![
            Box::new(self.0.content.render(f((&self.0).into()))),
            Box::new(self.1.content.render(f((&self.1).into()))),
        ]
    }
}

pub struct Tab<T: RenderObject> {
    pub title: String,
    pub icon: Option<Icon>,
    pub content: T,
}
impl<'a, T: RenderObject> From<&'a Tab<T>> for RenderedTabData<'a> {
    fn from(value: &'a Tab<T>) -> Self {
        RenderedTabData {
            title: &value.title,
            icon: value.icon.as_ref(),
        }
    }
}

pub struct RenderedTabData<'a> {
    pub title: &'a str,
    pub icon: Option<&'a Icon>,
}
