pub mod basic;

pub trait Demo {
    fn update(&mut self, event: tekenen::platform::Event) -> tekenen::platform::IntervalDecision;
    fn draw(&mut self, window: &mut tekenen::platform::Platform);
}