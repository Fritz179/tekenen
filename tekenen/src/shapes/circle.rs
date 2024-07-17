use crate::{math::{Transform, Vec2}, DrawableSurface};

use super::{Point, Rect, Triangle, Shape, Intersect};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    fn intersect_upcast(&self) -> &dyn Intersect {
        self
    }

    fn intersect(&self, other: &dyn Intersect) -> bool {
        other.intersect_circle(self)
    }

    fn intersect_point(&self, other: &Point) -> bool {
        let dx = self.position.x - other.position.x;
        let dy = self.position.y - other.position.y;

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

    fn is_enclosed_by(&self, other: &dyn Intersect) -> bool {
        other.encloses_circle(self)
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

// impl BitShaping for Circle {
//     fn bit_dyn_clone(&self) -> Box<dyn Shape> {
//         Box::new(*self)
//     }
// }

impl Transform for Circle {
    fn translate(&mut self, offset: Vec2) {
        self.position += offset
    }

    fn scale(&mut self, zoom: f32) {
        self.position *= zoom;
        self.radius = (self.radius as f32 * zoom) as i32;
    }

}

impl Shape for Circle {
    fn draw_yourself(&self, target: &crate::tekenen::SurfaceDrawer) {
        target.circle(self.position.x, self.position.y, self.radius);
    }

    fn get_bounding_box(&self) -> Rect {
        let Vec2 {x, y} = self.position;
        let r = self.radius;

        Rect::new(x - r, y - r, r + r, r + r)
    }

    fn dyn_clone(&self) -> Box<dyn Shape> {
        Box::new(*self)
    }

    fn iter(&self) -> Box<dyn Iterator<Item = Vec2>> {
        Box::new(CircleIterator {
            position: self.position,
            radius: self.radius,
            // TODO: can improve
            curr: self.position + Vec2::new(-self.radius, -self.radius),
            done: false,
        })
    }
}

pub struct CircleIterator {
    position: Vec2,
    radius: i32,
    curr: Vec2,
    done: bool,
}

impl Iterator for CircleIterator {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let output = self.curr;

            if self.curr.x == self.position.x + self.radius {
                if self.curr.y == self.position.y + self.radius {
                    if self.done {
                        return None
                    } else {
                        self.done = true;
                    }
                } else {
                    self.curr.x = self.position.x - self.radius;
                    self.curr.y += 1;
                }
            } else {
                self.curr.x += 1;
            }

            let dx = self.curr.x - self.position.x;
            let dy = self.curr.y - self.position.y;
            let r = self.radius;

            if dx * dx + dy * dy < r * r {
                return Some(output)
            }
        }
    }
}