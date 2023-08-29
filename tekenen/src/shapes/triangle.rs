use crate::math::Vec2;

use super::{Point, Rect, Circle, Shape, Intersect};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Triangle {
    pub p1: Vec2,
    pub p2: Vec2,
    pub p3: Vec2,
}

impl Intersect for Triangle {
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
        other.encloses_triangle(self)
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
