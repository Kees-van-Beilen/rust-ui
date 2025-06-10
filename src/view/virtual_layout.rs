use crate::layout::{ComputableLayout, Position, Size};

// pub trait VirtualLayout {
//     type Children: ComputableLayout;
//     fn children(&self)->&Self::Children;
//     //create a custom layout organizer
// }

pub struct Child<'a> {
    ///index of this child
    pub index: usize,
    ///To number of children
    pub children_len: usize,
    ///reference to the child's layout
    pub layout: &'a mut dyn ComputableLayout,
}
pub struct ChildRef<'a> {
    ///index of this child
    pub index: usize,
    ///To number of children
    pub children_len: usize,
    ///reference to the child's layout
    pub layout: &'a dyn ComputableLayout,
}
#[derive(Debug, Default)]
pub enum PreferredSizeState {
    #[default]
    Uninitialized,
    None,
    Some(Size<f64>),
}
#[derive(Default, Debug)]
pub struct Frame {
    pub position: Position<f64>,
    pub size: Size<f64>,
}
pub trait VirtualLayoutManager<T>: Default {
    fn preferred_size(&self, _view: &T) -> Option<Size<f64>> {
        None
    }
    fn set_layout_for_child(&mut self, child: Child, with_frame: &Frame, view: &T);
    ///This method is ran for all children in sequence. Then set_layout_for_child is called in sequence
    /// This allows you to, if necessary, compute layouts which dynamically size based on the children's
    /// preferred dimensions.
    fn inspect_child(&mut self, _child: ChildRef, _with_frame: &Frame, _view: &T) {}
}

#[macro_export]
macro_rules! virtual_layout {
    ($name:ident ($data:ident) => $rendered:ident ($layout:ident) {$($field:ident:$type:ty),+}) => {
        pub struct $name<T: crate::view::collection::ViewCollection> {
            $(pub $field : $type),+,
            pub children: T,
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
        impl<T: crate::view::collection::ViewCollection> crate::layout::RenderObject for $name<T> {
            type Output = $rendered<T::RenderOutput>;

            fn render(&self, data: crate::native::RenderData) -> Self::Output {
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
                self.children.with_v_tables(|tables|{
                    let children_len:usize = tables.iter().map(|e|{let a = e.v_tables_len();if a ==0 {1}else{a}}).sum();
                    let mut index: usize = 0;
                    let mut manager:$layout = Default::default();
                    for table in tables.iter() {
                        let v_tables = table.v_tables();
                        if v_tables.len() == 0 {
                            let c = crate::view::virtual_layout::ChildRef{
                                index,
                                children_len,
                                layout: *table,
                            };
                            manager.inspect_child(c, &self.frame,&self.data);
                            index+=1;
                        }else{
                            //we only allow one level of recursion
                            for child in v_tables.iter() {
                                let c = crate::view::virtual_layout::ChildRef{
                                    index,
                                    children_len,
                                    layout: *child,
                                };
                                manager.inspect_child(c, &self.frame,&self.data);
                                index+=1;
                            }
                        }
                    }
                    for table in tables {
                        let v_tables = table.v_tables_mut();
                        if v_tables.len() == 0 {
                            let c = crate::view::virtual_layout::Child{
                                index,
                                children_len,
                                layout: *table,
                            };
                            manager.set_layout_for_child(c, &self.frame,&self.data);
                            index+=1;
                        }else{
                            //we only allow one level of recursion
                            for child in v_tables.iter_mut() {
                                let c = crate::view::virtual_layout::Child{
                                    index,
                                    children_len,
                                    layout: *child,
                                };
                                manager.set_layout_for_child(c, &self.frame,&self.data);
                                index+=1;
                            }
                        }
                    }
                    self.preferred_size = match manager.preferred_size(&self.data) {
                        Some(a) => crate::view::virtual_layout::PreferredSizeState::Some(a),
                        None => crate::view::virtual_layout::PreferredSizeState::None,
                    }

                });
            }
        }
        impl<T: crate::view::collection::LayoutCollection> crate::layout::ComputableLayout for $rendered<T> {
            fn set_size(&mut self, to: crate::layout::Size<f64>) {
                self.frame.size = to;
                self.set_child_layout();
            }

            fn preferred_size(&self,_:&Size<f64>)->Option<Size<f64>> {
                // println!("get pref size {:?}",self.preferred_size );
                match self.preferred_size {
                    crate::view::virtual_layout::PreferredSizeState::Some(a) => Some(a),
                    crate::view::virtual_layout::PreferredSizeState::None => None,
                    crate::view::virtual_layout::PreferredSizeState::Uninitialized => {
                        //perform layout calculation for first draw
                        let mut preferred_size = None;
                        self.children.with_v_tables_ref(|tables|{
                            let children_len:usize = tables.iter().map(|e|{let a = e.v_tables_len();if a ==0 {1}else{a}}).sum();
                            let mut index: usize = 0;
                            let mut manager:$layout = Default::default();
                            for table in tables.iter() {
                                let v_tables = table.v_tables();
                                if v_tables.len() == 0 {
                                    let c = crate::view::virtual_layout::ChildRef{
                                        index,
                                        children_len,
                                        layout: *table,
                                    };
                                    manager.inspect_child(c, &self.frame,&self.data);
                                    index+=1;
                                }else{
                                    //we only allow one level of recursion
                                    for child in v_tables.iter() {
                                        let c = crate::view::virtual_layout::ChildRef{
                                            index,
                                            children_len,
                                            layout: *child,
                                        };
                                        manager.inspect_child(c, &self.frame,&self.data);
                                        index+=1;
                                    }
                                }
                            }
                            preferred_size = manager.preferred_size(&self.data);
                            // dbg!(manager);
                        });
                        preferred_size
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
