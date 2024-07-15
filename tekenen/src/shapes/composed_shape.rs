use crate::math::Vec2;

use super::{Shape, BitShaping, Intersect, point::Point, rect::Rect, circle::Circle, triangle::Triangle};

#[derive(Debug, Clone, Copy)]
pub enum BitOperand {
    And,
    Or,
    Xor,
    Not
}

#[derive(Debug)]
pub struct ComposedShape {
    shape_a: Box<dyn Shape>,
    shape_b: Box<dyn Shape>,
    operation: BitOperand,
}

impl ComposedShape {
    pub fn and(a: Box<dyn Shape>, b: Box<dyn Shape>) -> Self {
        Self {
            shape_a: a,
            shape_b: b,
            operation: BitOperand::And
        }
    }
}

impl Intersect for ComposedShape {
    fn intersect_upcast(&self) -> &dyn Intersect {
        self
    }

    fn intersect(&self, other: &dyn Intersect) -> bool {
        other.intersect(self)
    }

    fn intersect_point(&self, other: &Point) -> bool {
        todo!()
    }

    fn intersect_rect(&self, other: &Rect) -> bool {
        todo!()
    }

    fn intersect_circle(&self, other: &Circle) -> bool {
        todo!()
    }

    fn intersect_triangle(&self, other: &Triangle) -> bool {
        todo!()
    }

    fn encloses(&self, other: &dyn Intersect) -> bool {
        todo!()
    }

    fn is_enclosed_by(&self, other: &dyn Intersect) -> bool {
        todo!()
    }

    fn encloses_point(&self, other: &Point) -> bool {
        todo!()
    }

    fn encloses_rect(&self, other: &Rect) -> bool {
        todo!()
    }

    fn encloses_circle(&self, other: &Circle) -> bool {
        todo!()
    }

    fn encloses_triangle(&self, other: &Triangle) -> bool {
        todo!()
    }
}

use std::cmp::{min, max};

impl Shape for ComposedShape {
    fn draw_yourself(&self, target: &crate::tekenen::SurfaceDrawer) {
        todo!()
    }

    fn dyn_clone(&self) -> Box<dyn Shape> {
        todo!()
    }

    fn get_bounding_box(&self) -> Rect {
        let a = self.shape_a.get_bounding_box();
        let b = self.shape_a.get_bounding_box();

        let Rect { position: Vec2 {x: ax, y: ay}, size: Vec2 {x: aw, y: ah} } = a;
        let Rect { position: Vec2 {x: bx, y: by}, size: Vec2 {x: bw, y: bh} } = b;

        let x1 = min(ax, bx);
        let y1 = min(ay, by);

        let x2 = max(ax + aw, bx + bw);
        let y2 = max(ay + ah, by + bh);

        Rect::new(x1, y1, x2 - x1, y2 - y1)
    }

    fn iter(&self) -> Box<dyn Iterator<Item = Vec2>> {
        Box::new(ComposedIterator {
            shape_a: self.shape_a.dyn_clone(),
            shape_b: self.shape_b.dyn_clone(),
            operation: self.operation,
            rect_iter: self.get_bounding_box().iter()
        })
    }

    fn scale(&mut self, zoom: f32) {
        todo!()
    }

    fn tranlsate(&mut self, offset: Vec2) {
        todo!()
    }
}

impl BitShaping for ComposedShape {
    fn bit_dyn_clone(&self) -> Box<dyn Shape> {
        todo!()
    }

    fn join_and(&self, other: &dyn BitShaping) -> ComposedShape {
        todo!()
    }
}

struct ComposedIterator {
    shape_a: Box<dyn Shape>,
    shape_b: Box<dyn Shape>,
    operation: BitOperand,
    rect_iter: Box<dyn Iterator<Item = Vec2>>
}

impl Iterator for ComposedIterator {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let candidate = self.rect_iter.next()?;

            let bit_a = self.shape_a.intersect_point(&candidate.into());
            let bit_b = self.shape_b.intersect_point(&candidate.into());

            if match self.operation {
                BitOperand::And => bit_a & bit_b,
                BitOperand::Or  => bit_a | bit_b,
                BitOperand::Xor => bit_a ^ bit_b,
                BitOperand::Not =>       ! bit_a,
            } {
                return Some(candidate)
            }
        }
    }
}