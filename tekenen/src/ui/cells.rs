// Percentage (to what?), Absolute, Auto
#[derive(Debug, Clone)]
enum UICell {
    Auto,
    Pixels(i32),
    Percent(f32),
    BorderColor(u32),
}

// Length, Percentage, Auto
type UILenPercentAutoCell = UICell;

impl UILenPercentAutoCell {
    fn new_pixels(value: i32) -> Self {
        Self::Pixels(value)
    }

    fn new_percent(value: f32) -> Self {
        Self::Percent(value)
    }

    fn new_auto() -> Self {
        Self::Auto
    }

    fn get_percent(&self, percent100: i32) -> i32 {
        match self {
            Self::Pixels(pixels) => *pixels,
            Self::Percent(percent) => (percent100 as f32 * percent) as i32,
            _ => panic!("Invalid value: {self:?}")
        }
    }

    fn get_percent_auto(&self, percent100: i32, auto: i32) -> i32 {
        match self {
            Self::Pixels(pixels) => *pixels,
            Self::Percent(percent) => (percent100 as f32 * percent) as i32,
            Self::Auto => auto,
            _ => panic!("Invalid value: {self:?}")
        }
    }
}

// Positive Len, Percentage
type UIPosLenPercentCell = UICell;

impl UIPosLenPercentCell {
    fn new_pixels(value: i32) -> Self {
        assert!(value >= 0, "Value must be positive");

        Self::Pixels(value)
    }

    fn new_percent(value: f32) -> Self {
        assert!(value >= 0.0 && value <= 1.0, "Value must be between 0 and 1");

        Self::Percent(value)
    }

    fn get_percent(&self, percent100: i32) -> i32 {
        match self {
            Self::Pixels(pixels) => *pixels,
            Self::Percent(percent) => (percent100 as f32 * percent) as i32,
            _ => panic!("Invalid value: {self:?}")
        }
    }

}

type UIBorderCell = UICell;

struct UIEdge<T> {
    top: T,
    right: T,
    bottom: T,
    left: T,
}