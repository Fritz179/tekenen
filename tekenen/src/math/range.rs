use std::{iter::Sum, ops::{Add, AddAssign}};

/// Garanteed to have a valid range (min <= max)
#[derive(Debug, Default, Clone)]
pub struct Range<T: std::cmp::PartialOrd + Copy = i32> {
    pub min: T,
    pub max: T,
}

impl Range {
    pub fn new(min: i32, max: i32) -> Self {
        assert!(min <= max);

        Self {
            min,
            max
        }
    }

    pub fn new_definite(value: i32) -> Self {
        Self {
            min: value,
            max: value
        }
    }

    pub fn new_min_priority(min: i32, max: i32) -> Self {
        if max < min {
            Self {
                min,
                max: min
            }
        } else {
            Self {
                min,
                max
            }
        }
    }

    pub fn constrain(&self, value: i32) -> i32 {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }
}

impl<T: std::cmp::PartialOrd + Copy + Add<Output = T>> Add<T> for Range<T> {
    type Output = Self;

    fn add(self, value: T) -> Self {
        Self {
            min: self.min + value,
            max: self.max + value
        }
    }
}

impl<T: std::cmp::PartialOrd + Copy + Add<Output = T>> Add for Range<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            min: self.min + rhs.min,
            max: self.max + rhs.max
        }
    }
}

impl<T: std::cmp::PartialOrd + Copy + Default + Add<Output = T>> Sum for Range<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, x| acc + x)
    }
}

impl Range<i32> {
    pub fn get_ratio(&self, point: i32) -> f32 {
        (point - self.min) as f32 / (self.max - self.min) as f32
    }

    pub fn ratio(&self, ratio: f32) -> i32 {
        self.min + ((self.max - self.min) as f32 * ratio) as i32
    }
}

#[derive(Debug, Default, Clone)]
pub struct IndefRange<T: std::cmp::PartialOrd + Copy = i32> {
    min: Option<T>,
    max: Option<T>,
}

impl<T: std::cmp::PartialOrd + Copy> IndefRange<T> {
    pub fn new(min: T, max: T) -> Self {
        assert!(min <= max);

        Self {
            min: Some(min), 
            max: Some(max)
        }
    }

    pub fn new_definite(value: T) -> Self {
        Self {
            min: Some(value),
            max: Some(value)
        }
    }

    pub fn new_option(min: Option<T>, max: Option<T>) -> Self {
        if let Some(min) = min {
            if let Some(max) = max {
                assert!(min <= max);
            }
        }

        Self {
            min,
            max
        }
    }

    pub fn new_min_priority(min: Option<T>, max: Option<T>) -> Self {
        if let Some(min) = min {
            if let Some(max) = max {
                if max < min {
                    return Self {
                        min: Some(min),
                        max: Some(min)
                    }
                }
            }
        }

        Self {
            min, 
            max, 
        }
    }

    pub fn new_max_priority(min: Option<T>, max: Option<T>) -> Self {
        if let Some(min) = min {
            if let Some(max) = max {
                if max < min {
                    return Self {
                        min: Some(max),
                        max: Some(max)
                    }
                }
            }
        }
        
        Self {
            min,
            max,
        }
    }

    pub fn get_min(&self) -> Option<T> {
        self.min
    }

    pub fn get_max(&self) -> Option<T> {
        self.max
    }

    pub fn constrain(&self, value: T) -> T {
        if let Some(min) = self.min {
            if value < min {
                return min
            }
        }

        if let Some(max) = self.max {
            if value > max {
                return max
            }
        }

        value
    }

    pub fn is_constrained(&self) -> bool {
        self.min.is_some() || self.max.is_some()
    }

    pub fn is_constrained_both(&self) -> bool {
        self.min.is_some() && self.max.is_some()
    }

    pub fn identify(&self) -> Option<T> {
        let min = self.min?;
        let max = self.max?;

        if min == max {
            Some(min)
        } else {
            None
        }
    }

    /// Result fits in both
    /// Result is stricter or equeal
    pub fn and_max(&mut self, other: Option<T>) {
        if let Some(other) = other {
            if let Some(this) = self.max {
                if other < this {
                    self.max = Some(other)
                }
            } else {

                // other restriction is smaller
                self.max = Some(other)
            }
        }
    }

    /// Result may fit it only one
    /// Result is looser or equal
    pub fn or_max(&mut self, other: Option<T>) {
        if let Some(other) = other {
            if let Some(this) = self.max {
                if other > this {
                    self.max = Some(other)
                }
            }
        } else {

            // other restriction is bigger or equeal
            self.max = None
        }
    }

    /// Result fits in both
    /// Result is stricter or equeal
    pub fn and_min(&mut self, other: Option<T>) {
        if let Some(other) = other {
            if let Some(this) = self.min {
                if other > this {
                    self.min = Some(other)
                }
            } else {

                // other restriction is bigger
                self.min = Some(other)
            }
        }
    }

    /// Result may fit it only one
    /// Result is looser or equal
    pub fn or_min(&mut self, other: Option<T>) {
        if let Some(other) = other {
            if let Some(this) = self.min {
                if other < this {
                    self.min = Some(other)
                }
            }
        } else {

            // other restriction is smaller or equeal
            self.min = None
        }
    }
}

impl IndefRange<i32> {
    pub fn get_ratio(&self, point: i32) -> Option<f32> {
        let min = self.min?;
        let max = self.max?;

        Some((point - min) as f32 / (max - min) as f32)
    }

    pub fn ratio(&self, ratio: f32) -> Option<i32> {
        let min = self.min?;
        let max = self.max?;

        Some(min + ((max - min) as f32 * ratio) as i32)
    }
}

// Add T to each component
impl<T: std::cmp::PartialOrd + Copy + Add<Output = T>> Add<T> for IndefRange<T> {
    type Output = Self;

    fn add(self, rhs: T )-> Self {
        let min = self.min.map(|t| t + rhs);
        let max = self.max.map(|t| t + rhs);

        Self {
            min,
            max
        }
    }
}

impl<T: std::cmp::PartialOrd + Copy + Add<Output = T>> Add for IndefRange<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let min = self.min.map_or_else(|| rhs.min, |lhs| Some(rhs.min.map_or_else(|| lhs, |rhs| lhs + rhs)));
        let max = self.max.map_or_else(|| rhs.max, |lhs| Some(rhs.max.map_or_else(|| lhs, |rhs| lhs + rhs)));

        Self {
            min,
            max
        }
    }
}

impl<T: PartialOrd + Copy + Add<Output = T>> AddAssign<T> for IndefRange<T> {
    fn add_assign(&mut self, rhs: T) {
        if let Some(min) = self.min {
            self.min = Some(min + rhs);
        }

        if let Some(max) = self.max {
            self.max = Some(max + rhs);
        }
    }
}