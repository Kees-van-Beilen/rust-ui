//! Wrapper around a native tab bar

use crate::{
    icon::Icon,
    layout::{ComputableLayout, RenderObject},
    native::RenderData,
};

/// A tabbar is very os depended
/// On mobile devices it will look like a bar at the bottom
/// Whilst on Macos and iPadOS it may take the form of a sidebar
/// 
/// This view must be the root view.
/// This view doesn't work on android
pub struct TabBar<T: TabGroup> {
    /// The currently active tab index
    pub active_tab: usize,
    /// A list of Tabs
    pub children: T,
}

/// A [`ViewCollection`](`crate::view::collection::ViewCollection`) containing only [`Tab`] views
pub trait TabGroup {
    /// Render the tabs in a tab group
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

/// A Tab view
pub struct Tab<T: RenderObject> {
    /// the title of the tab
    pub title: String,
    /// The icon of the tab
    pub icon: Option<Icon>,
    /// The content of the tab
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

/// Data about the tab
pub struct RenderedTabData<'a> {
    /// Title of the tab
    pub title: &'a str,
    /// Icon of the tab
    pub icon: Option<&'a Icon>,
}
