use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::Zero;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Vec2<T = i32> {
    pub x: T,
    pub y: T
}

// Modify
impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn set(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }

    pub fn tuple(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T: Zero> Zero for Vec2<T> {
    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero()
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T: Neg> Neg for Vec2<T> {
    type Output = Vec2<T::Output>;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y
        }
    }
}

// Addition
impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y
    }
}

// Subtraction
impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::<T> {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl<T: SubAssign> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y
    }
}

// Multiplication
impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: (self.x as f32 * rhs) as i32,
            y: (self.y as f32 * rhs) as i32,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x = (self.x as f32 * rhs) as i32; 
        self.y = (self.y as f32 * rhs) as i32; 
    }
}

// Multiplication
impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: (self.x as f32 * rhs) as i32,
            y: (self.y as f32 * rhs) as i32,
        }
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x = (self.x as f32 * rhs) as i32; 
        self.y = (self.y as f32 * rhs) as i32; 
    }
}