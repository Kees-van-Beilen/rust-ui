use crate::{
    self as rust_ui,
    view::virtual_layout::{inspect_recurse, set_layout_recurse},
};
use std::ops::Add;

use crate::{
    layout::{Position, Size},
    view::virtual_layout::VirtualLayoutManager,
    virtual_layout,
};

virtual_layout!(HStack (HStackData,HStackPartialInit) => RenderedHStack (HStackLayout) {
    spacing:f64
});

virtual_layout!(VStack (VStackData,VStackPartialInit) => RenderedVStack (VStackLayout) {
    spacing:f64
});

#[derive(Default, Debug)]
pub struct HStackLayout {
    current_width: f64,
    allocated_width: f64,
    unallocated_units: usize,
    prefer_height: Option<f64>,
    prefer_width: Option<f64>,
    child_count: usize,
}

#[derive(Default, Debug)]
pub struct VStackLayout {
    current_height: f64,
    allocated_height: f64,
    unallocated_units: usize,
    prefer_width: Option<f64>,
    prefer_height: Option<f64>,
    child_count: usize,
}

impl VirtualLayoutManager<HStackData> for HStackLayout {
    fn preferred_size(&self, _: &HStackData) -> Size<Option<f64>> {
        Size {
            width: self.prefer_width,
            height: self.prefer_height,
        }
    }
    fn set_layout_for_child(
        &mut self,
        child: crate::view::virtual_layout::Child,
        with_frame: &crate::view::virtual_layout::Frame,
        data: &HStackData,
    ) {
        let portion = match child.layout.preferred_size(&with_frame.size).width {
            Some(width) => width,
            None if self.unallocated_units == 0 => with_frame.size.width,
            None => {
                (with_frame.size.width
                    - self.allocated_width
                    - data.spacing * (self.child_count as f64 - 1.0).max(0.0))
                    / self.unallocated_units as f64
            }
        };

        child.layout.set_size(Size {
            width: portion,
            height: with_frame.size.height,
        });
        child.layout.set_position(Position {
            x: self.current_width + with_frame.position.x,
            y: with_frame.position.y,
        });

        self.current_width += portion + data.spacing;
    }
    fn inspect_child(
        &mut self,
        child: crate::view::virtual_layout::ChildRef,
        with_frame: &crate::view::virtual_layout::Frame,
        _: &HStackData,
    ) {
        let size = child.layout.preferred_size(&with_frame.size);
        // let portion = with_frame.size.width / child.children_len as f64;
        if let Some(width) = size.width {
            self.allocated_width += width;
        } else {
            self.unallocated_units += 1;
        }
        if let Some(height) = size.height {
            self.prefer_height = Some(self.prefer_height.unwrap_or_default().max(height));
        }
        if let Some(width) = size.width {
            self.prefer_width = Some(self.prefer_width.unwrap_or_default().add(width));
        }
        self.child_count += 1;
    }
}

impl VirtualLayoutManager<VStackData> for VStackLayout {
    fn preferred_size(&self, _view: &VStackData) -> Size<Option<f64>> {
        Size {
            width: self.prefer_width,
            height: self.prefer_height,
        }
    }
    fn set_layout_for_child(
        &mut self,
        child: crate::view::virtual_layout::Child,
        with_frame: &crate::view::virtual_layout::Frame,
        data: &VStackData,
    ) {
        let portion = match child.layout.preferred_size(&with_frame.size).height {
            Some(height) => height,
            None if self.unallocated_units == 0 => with_frame.size.height,
            None => {
                (with_frame.size.height
                    - self.allocated_height
                    - data.spacing * (self.child_count as f64 - 1.0).max(0.0))
                    / self.unallocated_units as f64
            }
        };

        child.layout.set_size(Size {
            width: with_frame.size.width,
            height: portion,
        });
        child.layout.set_position(Position {
            x: with_frame.position.x,
            y: self.current_height + with_frame.position.y,
        });
        self.current_height += portion + data.spacing;
    }
    fn inspect_child(
        &mut self,
        child: crate::view::virtual_layout::ChildRef,
        with_frame: &crate::view::virtual_layout::Frame,
        _: &VStackData,
    ) {
        // let portion = with_frame.size.height / child.children_len as f64;
        // println!("do portioned layout {}",child.index);
        let size: Size<Option<f64>> = child.layout.preferred_size(&with_frame.size);
        if let Some(height) = size.height {
            self.allocated_height += height;
        } else {
            self.unallocated_units += 1;
        }
        if let Some(width) = size.width {
            self.prefer_width = Some(self.prefer_width.unwrap_or_default().max(width));
        }
        if let Some(height) = size.height {
            self.prefer_height = Some(self.prefer_height.unwrap_or_default().add(height));
        }
        self.child_count += 1;
    }
}
