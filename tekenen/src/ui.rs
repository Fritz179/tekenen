use std::{cell::RefCell, rc::Rc};

use crate::{TransforView, Draw};

use super::Tekenen;

pub mod container;
pub use container::Container;

pub mod slider;
pub use slider::Slider;

pub mod text;
pub use text::Text;

#[derive(Debug, Default)]
pub enum Unit {
    #[default]
    Auto,
    Pixels(i32),
    Percent(f32),
}

impl Unit {
    fn pixels(&self) -> i32 {
        match self {
            Unit::Auto => panic!("Auto no pixels"),
            Unit::Percent(_) => panic!("Percent no pixels"),
            Unit::Pixels(pixels) => *pixels
        }
    }
}

#[derive(Debug, Default)]
pub struct Sides {
    up: Unit,
    right: Unit,
    down: Unit,
    left: Unit,
}

#[derive(Debug, Default)]
pub struct BoundingBox {
    margin: Sides,
    border: Sides,
    width: Unit,
    height: Unit
}

impl BoundingBox {
    fn new(width: i32, height: i32) -> Self {
        Self {
            margin: Sides::default(),
            border: Sides::default(),
            width: Unit::Pixels(width),
            height: Unit::Pixels(height)
        }
    }
}

pub trait UIBox {
    fn draw(&mut self, view: &mut dyn Draw);
    fn get_box(&mut self, max: BoundingBox) -> &BoundingBox;
    // fn get_children(&mut self) -> &[Box<dyn UIBox>];
}

// 1) Get size by passing down max allowed space for 100%
// 2) Draw according to calculated size, invalidate all if needed
// 3) React to key/mouse?

struct TempTV<'a> {
    target: &'a mut dyn Draw
}

impl<'a> TempTV<'a> {
    fn new(target: &'a mut dyn Draw, x: i32, y: i32, w: i32, h: i32) -> TransforView<TempTV<'a>> {
        TransforView::new(x, y, w, h, Rc::new(RefCell::new(Self {target})))
    }
}

impl<'a> Draw for TempTV<'a> {
    fn background(&mut self, color: crate::Pixel) {
        self.target.background(color)
    }

    fn shape(&mut self, shape: &dyn crate::shapes::Shape, color: crate::Pixel) {
        self.target.shape(shape, color)
    }

    fn draw_text(&mut self, text: &str, x: i32, y: i32) -> (i32, i32) {
        self.target.draw_text(text, x, y)
    }

    fn get_size(&self) -> crate::math::Vec2 {
        self.target.get_size()
    }
}

impl Tekenen {
    pub fn ui(&mut self, container: &mut Box<Container>) {
        let view = container.get_box(BoundingBox::new(self.width() as i32, self.height() as i32));

        let mut tv = TempTV::new(self, 0, 0, view.width.pixels(), view.height.pixels());

        container.draw(&mut tv)
    }
}