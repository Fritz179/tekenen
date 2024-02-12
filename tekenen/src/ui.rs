use std::{cell::RefCell, rc::Rc};

use crate::{TransforView, Draw, platform::Event};

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
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            margin: Sides::default(),
            border: Sides::default(),
            width: Unit::Pixels(width),
            height: Unit::Pixels(height)
        }
    }
}

pub trait Element {
    // React to event
    fn event(&mut self, event: Event);

    // TODO: Called with exact time difference
    // fn fixed_update(&mut self)

    // Called once before draw
    fn update(&mut self);

    // TODO: Should be draw(&self)
    fn draw(&mut self);
}

pub trait Contain {
    // React to event
    fn event(&mut self, event: Event);

    // TODO: Called with exact time difference
    // fn fixed_update(&mut self)

    // Called once before draw
    fn update(&mut self);

    // Also used to get tv size, pixels?
    fn get_tv(&mut self) -> &mut dyn Draw;

    // Used for propagating events
    fn get_children(&mut self) -> &[&mut dyn Element];
}

// Element vs Container
// Container has get_children and progagates events, also target of Draw
// EndContainer children [Element] need get_tv to know the Container size
// Who has margin & stuff, only Containers?

// Elements gets draw(tv: &mut tv)
// Container has on self

// Are divs elements? 

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

pub struct UIBuilder {
    tv: Rc<RefCell<TransforView>>,
    children: Vec<Box<dyn Element>>
}

pub trait UIBuilderTrait {
    fn build();
}

impl UIBuilder {
    pub fn new(tv: Rc<RefCell<TransforView>>) -> Self {
        Self {
            tv,
            children: Vec::new()
        }
    }

    pub fn build(self) -> Box<Container> {
        Container::new_vertical(self.children)
    }
}