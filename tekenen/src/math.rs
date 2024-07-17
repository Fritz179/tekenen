mod vec2;
pub use vec2::*;

mod mat;
pub use mat::*;

mod range;
pub use range::*;

mod zero;
pub use zero::*;

pub trait Transform {
    fn translate(&mut self, translation: Vec2);
    fn scale(&mut self, scale: f32);
}

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value > min {
        if value < max {
            value
        } else {
            max
        }
    } else {
        min
    }
}