#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn add(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}


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