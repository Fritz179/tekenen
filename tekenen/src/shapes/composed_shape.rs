use std::ops::BitAnd;

use super::{Shape, rect::Rect};

pub struct ComposedShape {
    on_mask: Vec<Box<dyn Shape>>,
    off_mask: Vec<Box<dyn Shape>>,
}

impl ComposedShape {

}

// impl Shape for ComposedShape {

// }