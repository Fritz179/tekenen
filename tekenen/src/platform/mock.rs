use super::PlatformTrait;

pub struct MockPlatform {}

impl PlatformTrait for MockPlatform {
    fn new(width: u32, height: u32) -> Result<Self, super::PlatformError> where Self: Sized {
        unreachable!()
    }

    fn display_surface(&mut self, pixels: std::cell::Ref<crate::tekenen::Surface>) {
        unreachable!()
    }

    fn get_remaining_time() -> std::time::Duration {
        unreachable!()
    }

    fn log(value: u32) {
        unreachable!()
    }

    fn read_events(&mut self) -> Option<super::Event> {
        unreachable!()
    }

    fn set_interval(callback: impl FnMut() -> super::IntervalDecision + 'static, fps: u32) {
        unreachable!()
    }
}