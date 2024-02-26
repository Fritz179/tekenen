use std::{cell::RefMut, iter::Sum, ops::{Add, AddAssign, Bound, Sub}};

use crate::{math::{IndefRange, Range, Vec2}, platform::Event, Tekenen};

pub mod div;
pub use div::DivElement;

pub mod slider;
pub use slider::Slider;

pub mod text;
pub use text::TextElement;

use self::div::Direction;

// https://www.w3.org/TR/UI-sizing-3/#preferred-size-properties
#[derive(Debug, Default)]
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

// TODO: Do it better
// size is (min-content, max-content)
#[derive(Debug)]
pub enum SpaceContraint {
    Ratio(f32, (i32, i32)),
    Area(i32, (i32, i32)),
    Fixed(Range, Range, (i32, i32)),
    Combined(Vec<SpaceContraint>, Direction, (i32, i32)),
    Percent(i32, i32, (i32, i32)),
    RangeConstrain(IndefRange, IndefRange, Box<SpaceContraint>, (i32, i32)),
    MinPercentConstrain(i32, i32, Box<SpaceContraint>, (i32, i32)),
    MaxPercentConstrain(i32, i32, Box<SpaceContraint>, (i32, i32)),
}

impl SpaceContraint {
    fn new_ratio(ratio: f32, size: (i32, i32)) -> Self {
        Self::Ratio(ratio, size)
    }

    fn new_area(area: i32, size: (i32, i32)) -> Self {
        Self::Area(area, size)
    }

    fn new_range(width: Range, height: Range, size: (i32, i32)) -> Self {
        Self::Fixed(width, height, size)
    }

    fn new_fixed(width: i32, height: i32, size: (i32, i32)) -> Self {
        Self::Fixed(Range::new_definite(width), Range::new_definite(height), size)
    }

    fn new_combined(children: Vec<SpaceContraint>, direction: Direction, size: (i32, i32)) -> Self {
        Self::Combined(children, direction, size)
    }

    fn new_percent(width: i32, height: i32, size: (i32, i32)) -> Self {
        Self::Percent(width, height, size)
    }

    fn new_constrain(width: IndefRange, height: IndefRange, child: SpaceContraint, size: (i32, i32)) -> Self {
        Self::RangeConstrain(width, height, Box::new(child), size)
    }

    fn new_min_percent_constrain(width: i32, height: i32, child: SpaceContraint, size: (i32, i32)) -> Self {
        Self::MinPercentConstrain(width, height, Box::new(child), size)
    }

    fn new_max_percent_constrain(width: i32, height: i32, child: SpaceContraint, size: (i32, i32)) -> Self {
        Self::MaxPercentConstrain(width, height, Box::new(child), size)
    }
}

impl SpaceContraint {
    fn get_height(&self, width: i32, parent: Vec2) -> i32 {
        match self {
            Self::Ratio(ratio, _size) => (width as f32 / ratio) as i32,
            Self::Area(area, _size) => area / width,
            Self::Fixed(height, _, _size) => height.constrain(parent.y),
            Self::Combined(children, Direction::Column, _size) => 
                children.iter().map(|c| c.get_height(width, parent)).sum(),
            Self::Combined(children, Direction::Row, _size) => 
                children.iter().map(|c| c.get_height(width, parent)).max().unwrap(),
            Self::Percent(_, percent, _size) => parent.x * percent / 100,
            Self::RangeConstrain(_range, range, child, _size) => 
                range.constrain(child.get_height(width, parent)),
            Self::MinPercentConstrain(_percent, percent, child, _size) => 
                child.get_height(width, parent).max(parent.y * percent / 100),
            Self::MaxPercentConstrain(_percent, percent, child, _size) =>
                child.get_height(width, parent).min(parent.y * percent / 100),
        }
    }

    fn get_width(&self, height: i32, parent: Vec2) -> i32 {
        match self {
            Self::Ratio(ratio, _size) => (height as f32 * ratio) as i32,
            Self::Area(area, _size) => area / height,
            Self::Fixed(_, width, _size) => width.constrain(parent.x),
            Self::Combined(children, Direction::Column, _size) => 
                children.iter().map(|c| c.get_width(height, parent)).max().unwrap(),
            Self::Combined(children, Direction::Row, _size) => 
                children.iter().map(|c| c.get_width(height, parent)).sum(),
            Self::Percent(percent, _, _size) => parent.x * percent / 100,
            Self::RangeConstrain(range, _range, child, _size) => 
                range.constrain(child.get_width(height, parent)),
            Self::MinPercentConstrain(percent, _percent, child, _size) =>
                child.get_width(height, parent).max(parent.x * percent / 100),
            Self::MaxPercentConstrain(percent, _percent, child, _size) =>
                child.get_width(height, parent).min(parent.x * percent / 100),
        }
    }

    fn get_min_content(&self) -> i32 {
        match self {
            Self::Ratio(_, size) => size.0,
            Self::Area(_, size) => size.0,
            Self::Fixed(_, _, size) => size.0,
            Self::Combined(children, _, size) => 
                children.iter().map(|c| c.get_min_content()).sum(),
            Self::Percent(_, percent, size) => size.0 * percent / 100,
            Self::RangeConstrain(_, _, child, size) => 
                child.get_min_content().max(size.0),
            Self::MinPercentConstrain(_, percent, child, size) =>
                child.get_min_content().max(size.0 * percent / 100),
            Self::MaxPercentConstrain(_, percent, child, size) =>
                child.get_min_content().min(size.0 * percent / 100),
        }
    }

    fn get_max_content(&self) -> i32 {
        match self {
            Self::Ratio(_, size) => size.1,
            Self::Area(_, size) => size.1,
            Self::Fixed(_, _, size) => size.1,
            Self::Combined(children, _, size) => 
                children.iter().map(|c| c.get_max_content()).sum(),
            Self::Percent(_, percent, size) => size.1 * percent / 100,
            Self::RangeConstrain(_, _, child, size) => 
                child.get_max_content().min(size.1),
            Self::MinPercentConstrain(_, percent, child, size) =>
                child.get_max_content().min(size.1 * percent / 100),
            Self::MaxPercentConstrain(_, percent, child, size) =>
                child.get_max_content().max(size.1 * percent / 100),
        }
    }
}

pub trait ElementBox {
    type InnerElement: Element;

    fn get(&self) -> RefMut<'_, Self::InnerElement>;

    // React to event
    fn event(&mut self, event: Event) {
        self.get().event(event)
    }

    // TODO: Called with exact time difference
    // fn fixed_update(&mut self)

    // Called once before draw
    fn update(&mut self) {
        self.get().update()
    }

    fn draw(&mut self, target: &mut Tekenen, available_space: Vec2) -> Vec2 {
        self.get().draw(target, available_space)
    }
}

pub trait Element: std::fmt::Debug {
    // React to event
    fn event(&mut self, event: Event);

    // Called once before layout and draw
    fn update(&mut self);

    // Do layout calclulation and return the space constraints

    // get Constraint or width / height ?
        // Constraints can be changed / frozen
        // width height is more specific
    fn get_layout(&self) -> SpaceContraint;

    // Draw onto target withing given space
    fn draw(&self, target: &mut Tekenen, space: Vec2) -> Vec2;

    fn get_bb(&self) -> &BoundingBox {
        todo!()
    }
}