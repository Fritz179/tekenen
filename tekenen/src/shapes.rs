pub mod point;
use std::ops::BitAnd;

use point::Point;

pub mod rect;
use rect::Rect;

pub mod circle;
use circle::Circle;

pub mod triangle;
use triangle::Triangle;

mod composed_shape;
pub use self::composed_shape::ComposedShape;

use crate::math::Vec2;

pub type Positon = Point;
pub type Size = Vec2;

pub trait Shape: Intersect + BitShaping {
    fn transform(&mut self, offset: Vec2, zoom: f32);
    fn get_bounding_box(&self) -> Rect;
    fn dyn_clone(&self) -> Box<dyn Shape>;
    fn iter(&self) -> Box<dyn Iterator<Item = Vec2>>;
}

pub trait Intersect {
    fn intersect(&self, other: &dyn Intersect) -> bool;
    fn intersect_point(&self, other: &Point) -> bool;
    fn intersect_rect(&self, other: &Rect) -> bool;
    fn intersect_circle(&self, other: &Circle) -> bool;
    fn intersect_triangle(&self, other: &Triangle) -> bool;

    fn encloses(&self, other: &dyn Intersect) -> bool;
    fn encloses_point(&self, other: &Point) -> bool;
    fn encloses_rect(&self, other: &Rect) -> bool;
    fn encloses_circle(&self, other: &Circle) -> bool;
    fn encloses_triangle(&self, other: &Triangle) -> bool;
}

pub trait BitShaping {
    fn join(&self, other: &dyn BitShaping) -> ComposedShape { todo!() }
    fn join_point(&self, other: &Point) -> ComposedShape { todo!() }
    fn join_rect(&self, other: &Rect) -> ComposedShape { todo!() }
    fn join_circle(&self, other: &Circle) -> ComposedShape { todo!() }
    fn join_triangle(&self, other: &Triangle) -> ComposedShape { todo!() }
}

impl BitAnd for &dyn BitShaping {
    type Output = ComposedShape;

    fn bitand(self, rhs: &dyn BitShaping) -> Self::Output {
        rhs.join(self)
    }
}