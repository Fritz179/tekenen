use crate::math::Vec2;

use super::{Rect, Circle, Triangle, Shape, Intersect};

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    pub position: Vec2
}

impl From<Vec2> for Point {
    fn from(position: Vec2) -> Self {
        Self {
            position
        }
    }
}

impl Shape for Point {
    fn get_bounding_box(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, 0, 0)
    }
}

impl Intersect for Point {
    fn intersect(&self, other: &dyn Intersect) -> bool {
        other.intersect_point(self)
    }

    fn intersect_point(&self, other: &Point) -> bool {
        self.position == other.position
    }

    fn intersect_rect(&self, other: &Rect) -> bool {
        other.intersect_point(self)
    }

    fn intersect_circle(&self, other: &Circle) -> bool {
        other.intersect_point(self)
    }

    fn intersect_triangle(&self, other: &Triangle) -> bool {
        other.intersect_point(self)
    }

    fn encloses(&self, other: &dyn Intersect) -> bool {
        other.encloses_point(self)
    }

    fn encloses_point(&self, other: &Point) -> bool {
        self == other
    }

    // If rect has zero size?
    fn encloses_rect(&self, other: &Rect) -> bool {
        false
    }

    fn encloses_circle(&self, other: &Circle) -> bool {
        false
    }

    fn encloses_triangle(&self, other: &Triangle) -> bool {
        false
    }
}

