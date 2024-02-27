use std::{cell::RefCell, rc::Rc};

use crate::{math::{IndefRange, Range, Vec2}, platform::Event, shapes::rect::Rect, Tekenen};

use self::elements::{div::Direction, Element, SpaceContraint};

pub mod elements;

// https://www.w3.org/TR/UI-sizing-3/#preferred-size-properties
#[derive(Debug, Default, Clone, Copy)]
pub enum UIValue {
    #[default]
    Auto,
    Pixels(i32),
    Percent(f32),
}

impl UIValue {
    fn new(value: i32) -> Self {
        Self::Pixels(value)
    }

    fn is_definite(&self) -> bool {
        match self {
            UIValue::Auto => false,
            UIValue::Percent(_) => true,
            UIValue::Pixels(_) => true,
        }
    }

    fn get_pixels(&self, percent100: i32) -> Option<i32> {
        match self {
            UIValue::Auto => None,
            UIValue::Percent(percent) => Some((percent100 as f32 * percent) as i32),
            UIValue::Pixels(pixels) => Some(*pixels)
        }
    }

    fn get_pixels_auto(&self, auto: i32, percent100: i32) -> i32 {
        match self {
            UIValue::Auto => auto,
            UIValue::Percent(percent) => (percent100 as f32 * percent) as i32,
            UIValue::Pixels(pixels) => *pixels
        }
    }

    fn get_constraint(&self) -> Option<SpaceContraint> {
        match self {
            UIValue::Auto => None,
            UIValue::Percent(percent) => todo!(),
            // UIValue::Percent(percent) => Some(SpaceContraint::Percent(*percent as i32 * 100)),
            UIValue::Pixels(pixels) => todo!()
            // UIValue::Pixels(pixels) => Some(SpaceContraint::new_fixed(*pixels, *pixels))
        }
    }

    fn add_contrain(&self, contraint: SpaceContraint, direction: Direction) -> SpaceContraint {
        match self {
            UIValue::Auto => contraint,
            UIValue::Percent(percent) => todo!(),
            // UIValue::Percent(percent) => SpaceContraint::new(Range::new_definite(0), contraint),
            UIValue::Pixels(pixels) => contraint
        }
    }
}

#[derive(Debug, Default)]
struct UISize {
    value: UIValue,
    min: UIValue,
    max: UIValue
}

impl UISize {
    // https://www.w3.org/TR/UI-sizing-3/#terms
    fn new_pixels(value: i32) -> Self {
        Self {
            value: UIValue::Pixels(value),
            ..Default::default()
        }
    }

    fn is_constrained(&self) -> bool {
        self.min.is_definite() || self.max.is_definite()
    }

    fn get_constraint(&self, percent100: i32) -> IndefRange {
        IndefRange::new_option(self.min.get_pixels(percent100), self.max.get_pixels(percent100))
    }

    // https://UI-tricks.com/almanac/properties/m/max-width/
    fn constrain(&self, value: i32, percent100: i32) -> i32 {
        self.get_constraint(percent100).constrain(value)
    }
}

#[derive(Debug, Default)]
pub struct UISide {
    margin: UIValue,
    border: UIValue,
    padding: UIValue,
}

impl UISide {
    pub fn margin(&self, percent100: i32) -> i32 {
        self.margin.get_pixels_auto(0, percent100)
    }

    pub fn border(&self, percent100: i32) -> i32 {
        self.border.get_pixels_auto(0, percent100)
    }

    pub fn padding(&self, percent100: i32) -> i32 {
        self.padding.get_pixels_auto(0, percent100)
    }

    pub fn total_size(&self, percent100: i32) -> i32 {
        self.margin(percent100) + self.border(percent100) + self.padding(percent100)
    }
}


#[derive(Debug, Default)]
pub struct BoundingBox {
    up: UISide,
    right: UISide,
    down: UISide,
    left: UISide,
    width: UISize,
    height: UISize
}

impl BoundingBox {
    pub fn set_margin(&mut self, margin: UIValue) {
        // self.up = margin;

    }
}

impl BoundingBox {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            up: UISide::default(),
            right: UISide::default(),
            down: UISide::default(),
            left: UISide::default(),
            width: UISize::new_pixels(width),
            height: UISize::new_pixels(height)
        }
    }
}

struct Layout {
    /// clickable area ()
    clickable: Rect,

    /// 
    position: Rect,

    element: Rc<RefCell<dyn Element>>,
}


// struct UIBorder {
//     width: UIBorderCell,
//     style: ,
//     color: u32,
// }

// struct BoxStyle {
//     margin: UIEdge<UILenPercentAutoCell>,
//     padding: UIEdge<UIPosLenPercentCell>,
//     border: UIBorder
// }