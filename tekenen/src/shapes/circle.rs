use crate::math::Vec2;

use super::{Point, Rect, Triangle, Shape, Intersect};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Circle {
    pub position: Vec2,
    pub radius: i32,
}

impl Circle {
    pub fn new(x: i32, y: i32, radius: i32) -> Self {
        Self {
            position: Vec2::new(x, y),
            radius
        }
    }

    pub fn vec(pos: Vec2, radius: i32) -> Self {
        Self {
            position: pos,
            radius
        }
    }
}

impl Intersect for Circle {
    fn intersect(&self, other: &dyn Intersect) -> bool {
        other.intersect_circle(self)
    }

    fn intersect_point(&self, other: &Point) -> bool {
        let dx = self.position.x - other.position.x;
        let dy = self.position.x - other.position.x;

        dx * dx + dy * dy <= self.radius * self.radius
    }

    fn intersect_rect(&self, other: &Rect) -> bool {
        todo!()
    }

    fn intersect_circle(&self, other: &Circle) -> bool {
        let dx = self.position.x - other.position.x;
        let dy = self.position.x - other.position.x;
        let r = self.radius + other.radius;

        dx * dx + dy * dy <= r * r
    }

    fn intersect_triangle(&self, other: &Triangle) -> bool {
        other.intersect_circle(self)
    }

    fn encloses(&self, other: &dyn Intersect) -> bool {
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
