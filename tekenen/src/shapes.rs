pub mod point;
pub mod rect;
pub mod circle;
pub mod triangle;

use point::Point;
use rect::Rect;
use circle::Circle;
use triangle::Triangle;

use crate::math::Vec2;

pub type Positon = Point;
pub type Size = Vec2;

pub trait Shape: IntoIterator<Item = Vec2> {
    fn transform(&mut self, offset: &Vec2, zoom: f32);
    fn get_bounding_box(&self) -> Rect;
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