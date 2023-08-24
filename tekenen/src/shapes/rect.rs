use crate::math::Vec2;

use super::{Point, Circle, Triangle, Shape, Intersect};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rect {
    pub position: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h:i32) -> Rect {
        Rect {
            position: Vec2 { x, y },
            size: Vec2 {x: w, y: h}
        }
    }
}

impl Shape for Rect {
    fn get_bounding_box(&self) -> Rect {
        self.clone()
    }
}

impl Intersect for Rect {
    fn intersect(&self, other: &dyn Intersect) -> bool {
        other.intersect_rect(self)
    }

    fn intersect_point(&self, other: &Point) -> bool {
        other.position.x >= self.position.x &&
        other.position.x <= self.position.x + self.size.x &&
        other.position.y >= self.position.y &&
        other.position.y <= self.position.y + self.size.y
    }

    fn intersect_rect(&self, other: &Rect) -> bool {
        !(
            self.position.x > other.position.x + other.size.x ||
            self.position.x + self.size.x < other.position.x  ||
            self.position.y > other.position.y + other.size.y ||
            self.position.y + self.size.y < other.position.y
        )
    }

    fn intersect_circle(&self, other: &Circle) -> bool {
        other.intersect_rect(self)
    }

    fn intersect_triangle(&self, other: &Triangle) -> bool {
        other.intersect_rect(self)
    }

    fn encloses(&self, other: &dyn Intersect) -> bool {
        todo!()
    }

    fn encloses_point(&self, other: &Point) -> bool {
        self.intersect_point(other)
    }

    fn encloses_rect(&self, other: &Rect) -> bool {
        other.position.x >= self.position.x &&
        other.position.x + other.size.x <= self.position.x + self.size.x &&
        other.position.y >= self.position.y &&
        other.position.y + other.size.y <= self.position.y + self.size.y
    }

    fn encloses_circle(&self, other: &Circle) -> bool {
        other.position.x - other.radius >= self.position.x &&
        other.position.x + other.radius <= self.position.x + self.size.x &&
        other.position.y - other.radius >= self.position.y &&
        other.position.y + other.radius <= self.position.y + self.size.y
    }

    fn encloses_triangle(&self, other: &Triangle) -> bool {
        self.encloses_point(&other.p1.clone().into()) &&
        self.encloses_point(&other.p2.clone().into()) &&
        self.encloses_point(&other.p3.clone().into())
    }
}
