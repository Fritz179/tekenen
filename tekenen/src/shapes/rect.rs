use std::{fmt::Display, ops::{Add, AddAssign, Sub}};

use crate::{math::Vec2, DrawableSurface};

use super::{BitShaping, Circle, Intersect, Point, Shape, Sides, Triangle};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rect {
    pub position: Vec2,
    pub size: Vec2,
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect: [x: {}, y: {}, w: {}, h: {}]", self.position.x, self.position.y, self.size.x, self.size.y)
    }
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h:i32) -> Self {
        Self {
            position: Vec2 { x, y },
            size: Vec2 {x: w, y: h}
        }
    }

    pub fn new_vec(pos: Vec2, size: Vec2) -> Self {
        Self {
            position: pos,
            size
        }
    }
}

impl Shape for Rect {
    fn draw_yourself(&self, target: &crate::tekenen::SurfaceDrawer) {
        target.rect(self.position.x, self.position.y, self.size.x, self.size.y);
    }

    fn tranlsate(&mut self, offset: Vec2) {
        self.position += offset
    }

    fn scale(&mut self, zoom: f32) {
        self.position *= zoom;
        self.size *= zoom;
    }

    fn get_bounding_box(&self) -> Rect {
        *self
    }

    fn dyn_clone(&self) -> Box<dyn Shape> {
        Box::new(*self)
    }

    fn iter(&self) -> Box<dyn Iterator<Item = Vec2>> {
        Box::new(RectIter {
            start: self.position,
            end: self.position + self.size,
            curr: self.position,
            done: false
        })
    }
}

impl Intersect for Rect {
    fn intersect_upcast(&self) -> &dyn Intersect {
        self
    }

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
        other.is_enclosed_by(self)
    }

    fn is_enclosed_by(&self, other: &dyn Intersect) -> bool {
        other.encloses_rect(self)
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
        self.encloses_point(&other.p1.into()) &&
        self.encloses_point(&other.p2.into()) &&
        self.encloses_point(&other.p3.into())
    }
}

impl BitShaping for Rect {
    fn bit_dyn_clone(&self) -> Box<dyn Shape> {
        Box::new(*self)
    }
}

pub struct RectIter {
    start: Vec2,
    end: Vec2,
    curr: Vec2,
    done: bool
}

impl Iterator for RectIter {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.curr;

        if self.curr.x == self.end.x {
            if self.curr.y == self.end.y {
                if self.done {
                    return None
                } else {
                    self.done = true;
                }
            } else {
                self.curr.x = self.start.x;
                self.curr.y += 1;
            }
        } else {
            self.curr.x += 1;
        }

        Some(output)
    }
}

impl Add<Sides> for Rect {
    type Output = Self;

    fn add(self, rhs: Sides) -> Self::Output {
        Self {
            position: Vec2 {
                x: self.position.x - rhs.left,
                y: self.position.y - rhs.top
            },
            size: Vec2 {
                x: self.size.x + rhs.left + rhs.right,
                y: self.size.y + rhs.top + rhs.bottom
            }
        }
    }
}

impl AddAssign<Sides> for Rect {
    fn add_assign(&mut self, rhs: Sides) {
        self.position.x -= rhs.top;
        self.position.y -= rhs.left;
        self.size.x += rhs.top + rhs.bottom;
        self.size.y += rhs.left + rhs.right;
    }
}

impl Sub<Sides> for Rect {
    type Output = Self;

    fn sub(self, rhs: Sides) -> Self::Output {
        Self {
            position: Vec2 {
                x: self.position.x + rhs.left,
                y: self.position.y + rhs.top
            },
            size: Vec2 {
                x: self.size.x - rhs.left - rhs.right,
                y: self.size.y - rhs.top - rhs.bottom
            }
        }
    }
}