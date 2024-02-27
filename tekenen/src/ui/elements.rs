pub mod div;
pub use div::Div;

pub mod slider;
pub use slider::Slider;

pub mod text;
pub use text::Text;

use crate::{math::{IndefRange, Range, Vec2}, platform::Event, Tekenen};

use self::div::Direction;

use super::BoundingBox;

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
