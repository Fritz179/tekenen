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

impl Circle {
    fn iter(&self) -> CircleIterator {
        CircleIterator {
            position: self.position.clone(),
            radius: self.radius,
            // TODO: can improve
            curr: self.position.clone() + Vec2::new(-self.radius, -self.radius),
            done: false,
        }
    }
}

impl IntoIterator for Circle {
    type Item = Vec2;
    type IntoIter = CircleIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Shape for Circle {
    fn get_bounding_box(&self) -> Rect {
        todo!()
    }

    fn transform(&mut self, offset: &Vec2, zoom: f32) {
        self.position *= zoom;
        self.position += offset;
        
        self.radius = (self.radius as f32 * zoom) as i32;
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
            let output = self.curr.clone();

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