//! This crate contains a utility macro to help with the creation of views, who's soul purpose is to layout it child views.
use crate::layout::{ComputableLayout, Position, Size};

/// A child that can freely changed in size/position
pub struct Child<'a> {
    ///index of this child
    pub index: usize,
    ///To number of children
    pub children_len: usize,
    ///reference to the child's layout
    pub layout: &'a mut dyn ComputableLayout,
}

/// A readonly child
pub struct ChildRef<'a> {
    ///index of this child
    pub index: usize,
    ///To number of children
    pub children_len: usize,
    ///reference to the child's layout
    pub layout: &'a dyn ComputableLayout,
}

/// Never appears in user facing code, nor do users of the `virtual_layout!` macro inside of this crate have to worry about this structure
#[doc(hidden)]
#[derive(Debug, Default)]
pub enum PreferredSizeState {
    #[default]
    Uninitialized,
    Initialized(Size<Option<f64>>),
}
/// Never appears in user facing code, nor do users of the `virtual_layout!` macro inside of this crate have to worry about this structure
/// This may lead to a lot of confusion with the `frame` modifier
#[doc(hidden)]
#[derive(Default, Debug)]
pub struct Frame {
    pub position: Position<f64>,
    pub size: Size<f64>,
}

pub trait VirtualLayoutManager<T>: Default {
    ///
    /// Communicate the size this view whishes to take.
    /// This almost directly translates to [`crate::layout::ComputableLayout::preferred_size`].
    /// However before this function is called first [`VirtualLayoutManager::inspect_child`] is called for every child view (sequentially).
    /// This allows you to calculate the preferred size based on the children.
    ///
    fn preferred_size(&self, _view: &T) -> Size<Option<f64>> {
        Size::splat(None)
    }
    ///
    /// This function is called when a parent view calls `set_size` or `set_position` on this view.
    /// It is called for every child in this view sequentially
    ///
    fn set_layout_for_child(&mut self, child: Child, with_frame: &Frame, view: &T);
    ///This method is ran for all children in sequence. Then set_layout_for_child is called in sequence
    /// This allows you to, if necessary, compute layouts which dynamically size based on the children's
    /// preferred dimensions.
    fn inspect_child(&mut self, _child: ChildRef, _with_frame: &Frame, _view: &T) {}
}

// if possible this should not be a macro but an auto implement

pub fn inspect_recurse<Data, Manager: VirtualLayoutManager<Data>>(
    manager: &mut Manager,
    index: &mut usize,
    frame: &Frame,
    data: &Data,
    child: &dyn ComputableLayout,
) {
    if child.v_tables_len() == 0 {
        manager.inspect_child(
            ChildRef {
                index: *index,
                children_len: 0,
                layout: child,
            },
            frame,
            data,
        );
    } else {
        let mut buf = Vec::new();
        child.write_v_tables(&mut buf);
        for child in buf.into_iter() {
            inspect_recurse(manager, index, frame, data, child);
        }
    }
}

pub fn set_layout_recurse<Data, Manager: VirtualLayoutManager<Data>>(
    manager: &mut Manager,
    index: &mut usize,
    frame: &Frame,
    data: &Data,
    child: &mut dyn ComputableLayout,
) {
    if child.v_tables_len() == 0 {
        manager.set_layout_for_child(
            Child {
                index: *index,
                children_len: 0,
                layout: child,
            },
            frame,
            data,
        );
    } else {
        let mut buf = Vec::new();
        child.write_v_tables_mut(&mut buf);
        for child in buf.into_iter() {
            set_layout_recurse(manager, index, frame, data, child);
        }
    }
}

///
/// automatically implement the boiler plate
///
/// ## Usage example
/// ```
/// //here you write the data required to properly compute the layout
/// #[derive(Default, Debug)]
/// pub struct VStackLayout {
///     current_width: f64,
///     allocated_width: f64,
///     unallocated_units: usize,
///     prefer_height: Option<f64>,
///     prefer_width: Option<f64>,
///     child_count:usize,
/// }
/// // this macro call automatically creates the structs `VStack`, `VStackData`, `VStackPartialInit` and `RenderedVStack`
/// virtual_layout!(VStack (VStackData,VStackPartialInit) => RenderedVStack (VStackLayout) {
///     //here are the fields that user of your view will have to input
///     spacing:f64
/// });
///
/// ```
#[macro_export]
macro_rules! virtual_layout {
    ($name:ident ($data:ident, $partial:ident) => $rendered:ident ($layout:ident) {$($field:ident:$type:ty),+}) => {
        pub struct $name<T: crate::view::collection::ViewCollection> {
            $(pub $field : $type),+,
            pub children: T,
        }
        impl<T: crate::view::collection::ViewCollection> Default for $partial<T>{
            fn default()->Self {
                Self {
                     $($field : None),+,
                     children:None
                }
            }
        }
        pub struct $partial<T: crate::view::collection::ViewCollection> {
             $(pub $field : Option<$type>),+,
             pub children: Option<T>
        }
        pub struct $data {
            $(pub $field : $type),+,
        }
        pub struct $rendered<T: crate::view::collection::LayoutCollection> {
            data:$data,
            frame:crate::view::virtual_layout::Frame,
            children: T,
            preferred_size:crate::view::virtual_layout::PreferredSizeState
        }
        impl<T: crate::view::collection::ViewCollection> rust_ui::PartialInitialisable for $name<T> {
            type PartialInit = $partial<T>;
        }
        impl<T: crate::view::collection::ViewCollection> $name<T> {
            pub fn new(init: $partial<T>)->Self{
                Self {
                    $($field : init.$field.unwrap_or_default()),+,
                    children: init.children.unwrap()
                }
            }
        }
        impl<T: crate::view::collection::ViewCollection> crate::layout::RenderObject for $name<T> {
            type Output = $rendered<T::RenderOutput>;

            fn render(&self, data: crate::native::RenderData) -> Self::Output {
                // crate::android_println!("(virtual)trace/render {}",std::any::type_name::<$name<T>>());
                $rendered {
                    data: $data {
                        $($field : self.$field),+,
                    },
                    frame: Default::default(),
                    children: self.children.render(data),
                    preferred_size: crate::view::virtual_layout::PreferredSizeState::Uninitialized
                }
            }
        }
        impl<T: crate::view::collection::LayoutCollection> $rendered<T> {

            fn set_child_layout(&mut self){
                // while let Some(table) =
                let mut buf = Vec::new();
                self.children.write_v_tables_mut(&mut buf);

                let mut manager:$layout = Default::default();
                let mut index = 0usize;

                for child in buf.iter() {
                    inspect_recurse(&mut manager, &mut index, &self.frame, &self.data, *child);
                }
                index = 0;
                for child in buf.iter_mut() {
                    set_layout_recurse(&mut manager, &mut index, &self.frame, &self.data, *child);
                }
                self.preferred_size = crate::view::virtual_layout::PreferredSizeState::Initialized(manager.preferred_size(&self.data));
            }
        }
        impl<T: crate::view::collection::LayoutCollection> crate::layout::ComputableLayout for $rendered<T> {
            fn set_size(&mut self, to: crate::layout::Size<f64>) {
                self.frame.size = to;
                self.set_child_layout();
            }

            fn preferred_size(&self,_:&Size<f64>)->Size<Option<f64>> {
                // println!("get pref size {:?}",self.preferred_size );
                match self.preferred_size {
                    crate::view::virtual_layout::PreferredSizeState::Initialized(a) => a,
                    crate::view::virtual_layout::PreferredSizeState::Uninitialized => {
                        let mut index: usize = 0;
                        let mut manager:$layout = Default::default();
                        let mut buf = Vec::new();
                        self.children.write_v_tables(&mut buf);
                        for child in buf.iter() {
                            inspect_recurse(&mut manager, &mut index, &self.frame, &self.data, *child);
                        }
                        manager.preferred_size(&self.data)
                    }
                }
            }

            fn set_position(&mut self, to: crate::layout::Position<f64>) {
                self.frame.position = to;
                self.set_child_layout();
            }

            fn destroy(&mut self) {
                self.children.with_v_tables(|tables| {
                    for child in tables {
                        child.destroy();
                    }
                });
            }
        }

    };
}
