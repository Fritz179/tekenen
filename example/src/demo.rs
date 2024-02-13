use tekenen::platform::{Event, Platform, IntervalDecision};

pub mod image;
pub mod text;
pub mod canvas;

pub trait Demo {
    fn update(&mut self, event: &Event) -> IntervalDecision;
    fn draw(&mut self, window: &mut Platform);
}

