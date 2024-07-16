use std::ops::Add;

pub mod point;
use point::Point;

pub mod line;
use line::Line;

pub mod rect;
use rect::Rect;

pub mod circle;
use circle::Circle;

pub mod triangle;
use triangle::Triangle;

// mod composed_shape;
// pub use self::composed_shape::ComposedShape;

use crate::{math::Vec2, tekenen::SurfaceDrawer};

pub trait Shape: Intersect + /* BitShaping + */ std::fmt::Debug {
    fn tranlsate(&mut self, offset: Vec2);
    fn scale(&mut self, zoom: f32);

    fn draw_yourself(&self, target: &SurfaceDrawer);

    fn get_bounding_box(&self) -> Rect;
    fn dyn_clone(&self) -> Box<dyn Shape>;
    fn iter(&self) -> Box<dyn Iterator<Item = Vec2>>;
}

pub trait Intersect {
    fn intersect_upcast(&self) -> &dyn Intersect;

    fn intersect(&self, other: &dyn Intersect) -> bool;
    fn intersect_point(&self, other: &Point) -> bool;
    fn intersect_rect(&self, other: &Rect) -> bool;
    fn intersect_circle(&self, other: &Circle) -> bool;
    fn intersect_triangle(&self, other: &Triangle) -> bool;

    fn encloses(&self, other: &dyn Intersect) -> bool;
    fn is_enclosed_by(&self, other: &dyn Intersect) -> bool;
    fn encloses_point(&self, other: &Point) -> bool;
    fn encloses_rect(&self, other: &Rect) -> bool;
    fn encloses_circle(&self, other: &Circle) -> bool;
    fn encloses_triangle(&self, other: &Triangle) -> bool;
}

// pub trait BitShaping {
//     fn bit_dyn_clone(&self) -> Box<dyn Shape>;
    
//     fn join_and(&self, other: &dyn BitShaping) -> ComposedShape {
//         ComposedShape::and(self.bit_dyn_clone(), other.bit_dyn_clone())
//     }
//     // fn join_point(&self, other: &Point) -> ComposedShape { todo!() }
//     // fn join_rect(&self, other: &Rect) -> ComposedShape { todo!() }
//     // fn join_circle(&self, other: &Circle) -> ComposedShape { todo!() }
//     // fn join_triangle(&self, other: &Triangle) -> ComposedShape { todo!() }
// }

// can &, |, ^, !
// can get bounding boxes and test each pixel

// impl std::ops::BitAnd for &dyn BitShaping {
//     type Output = ComposedShape;

//     fn bitand(self, rhs: &dyn BitShaping) -> Self::Output {
//         rhs.join_and(self)
//     }
// }

/// A helper to store all the directions
#[derive(Debug, Clone)]
pub struct Sides<T = i32> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl<T: Default> Default for Sides<T> {
    fn default() -> Self {
        Self {
            top: T::default(),
            right: T::default(),
            bottom: T::default(),
            left: T::default()
        }
    }
} 

impl<T: Copy > Sides<T> {
    pub fn new(value: T) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value
        }
    }
}

impl Sides {
    pub fn get_total_height(&self) -> i32 {
        self.top + self.bottom
    }

    pub fn get_total_width(&self) -> i32 {
        self.left + self.right
    }
}

impl<T: Add<Output = T>> Add for Sides<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            top: self.top + rhs.top,
            right: self.right + rhs.right,
            bottom: self.bottom + rhs.bottom,
            left: self.left + rhs.left
        }
    }
}