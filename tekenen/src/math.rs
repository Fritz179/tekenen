mod vec2;
pub use vec2::*;

mod mat;
pub use mat::*;

mod range;
pub use range::*;

pub fn constrain(val: i32, min: i32, max: i32) -> i32 {
    if val > min {
        if val < max {
            val
        } else {
            max
        }
    } else {
        min
    }
}