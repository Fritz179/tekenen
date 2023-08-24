#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32
}

// impl std::cmp::Eq for Vec2 {
//     fn assert_receiver_is_total_eq(&self) {
        
//     }
// }

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