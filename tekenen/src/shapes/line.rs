use crate::{math::Vec2, DrawableSurface};

use super::{BitShaping, Intersect, Shape};

#[derive(Debug)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
}

impl Line {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self { start: Vec2::new(x1, y1), end: Vec2::new(x2, y2) }
    }

    pub fn new_vec(p1: Vec2, p2: Vec2) -> Self {
        Self { start: p1, end: p2 }
    }
}

impl BitShaping for Line {
    fn bit_dyn_clone(&self) -> Box<dyn Shape> {
        todo!()
    }
}

impl Intersect for Line {
    fn intersect_upcast(&self) -> &dyn Intersect {
        todo!()
    }

    fn intersect(&self, other: &dyn Intersect) -> bool {
        todo!()
    }

    fn intersect_point(&self, other: &super::point::Point) -> bool {
        todo!()
    }

    fn intersect_rect(&self, other: &super::rect::Rect) -> bool {
        todo!()
    }

    fn intersect_circle(&self, other: &super::circle::Circle) -> bool {
        todo!()
    }

    fn intersect_triangle(&self, other: &super::triangle::Triangle) -> bool {
        todo!()
    }

    fn encloses(&self, other: &dyn Intersect) -> bool {
        todo!()
    }

    fn is_enclosed_by(&self, other: &dyn Intersect) -> bool {
        todo!()
    }

    fn encloses_point(&self, other: &super::point::Point) -> bool {
        todo!()
    }

    fn encloses_rect(&self, other: &super::rect::Rect) -> bool {
        todo!()
    }

    fn encloses_circle(&self, other: &super::circle::Circle) -> bool {
        todo!()
    }

    fn encloses_triangle(&self, other: &super::triangle::Triangle) -> bool {
        todo!()
    }
}

impl Shape for Line {
    fn draw_yourself(&self, target: &crate::tekenen::SurfaceDrawer) {
        target.line(self.start.x, self.start.y, self.end.x, self.end.y);
    }

    fn dyn_clone(&self) -> Box<dyn Shape> {
        todo!()
    }

    fn get_bounding_box(&self) -> super::rect::Rect {
        todo!()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = Vec2>> {
        todo!()
    }

    fn scale(&mut self, zoom: f32) {
        todo!()
    }

    fn tranlsate(&mut self, offset: Vec2) {
        todo!()
    }
}