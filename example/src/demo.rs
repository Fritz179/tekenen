use tekenen::platform::{Event, Platform, IntervalDecision};

pub mod basic;
pub mod text;
pub mod transform;
pub mod idea;
pub mod div;

pub trait Demo {
    fn update(&mut self, event: &Event) -> IntervalDecision;
    fn draw(&mut self, window: &mut Platform);
}

