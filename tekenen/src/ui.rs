use std::cell::RefMut;

use crate::{platform::Event, Tekenen};

pub mod div;
pub use div::DivElement;

pub mod slider;
pub use slider::Slider;

pub mod text;
pub use text::TextElement;

#[derive(Debug, Default)]
pub enum Value {
    #[default]
    Auto,
    Pixels(i32),
    Percent(f32),
}

impl Value {
    fn pixels(&self) -> i32 {
        match self {
            Value::Auto => panic!("Auto no pixels"),
            Value::Percent(_) => panic!("Percent no pixels"),
            Value::Pixels(pixels) => *pixels
        }
    }
}

#[derive(Debug, Default)]
pub struct Sides {
    up: Value,
    right: Value,
    down: Value,
    left: Value,
}

#[derive(Debug, Default)]
pub struct BoundingBox {
    margin: Sides,
    border: Sides,
    width: Value,
    height: Value
}

impl BoundingBox {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            margin: Sides::default(),
            border: Sides::default(),
            width: Value::Pixels(width),
            height: Value::Pixels(height)
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

    fn draw(&mut self, target: &mut Tekenen) {
        self.get().draw(target)
    }
}

pub trait Element {
    // React to event
    fn event(&mut self, event: Event);

    // Called once before draw
    fn update(&mut self);

    fn draw(&mut self, target: &mut Tekenen);
}