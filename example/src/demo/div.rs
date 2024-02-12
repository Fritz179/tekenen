use tekenen::ui::Element;
use tekenen::Tekenen;
use tekenen::platform::{Platform, PlatformTrait, IntervalDecision, Event, KeyDownEvent};

pub struct DivDemo {
    tek: Tekenen,
    tick: i32,
}

impl DivDemo {
    pub fn new() -> Self {
        Self {
            tek: Tekenen::new(800, 600),
            tick: 0,
        }
    }
}

impl super::Demo for DivDemo {
    fn update(&mut self, event: &Event) -> tekenen::platform::IntervalDecision {
        match event {
            Event::Quit => {
                return IntervalDecision::Stop
            },
            Event::KeyDown(KeyDownEvent { char: Some(char), .. }) => {
                println!("{char}")
            },
            // Event::MouseDown { x, y } => {
            //     self.slider.mouse_down(x, y);
            // },
            // Event::MouseMove { x, y } => {
            //     self.slider.mouse_move(x, y);
            // },
            // Event::MouseUp { x, y } => {
            //     self.slider.mouse_up(x, y);
            // },
            _ => { }
        };

        IntervalDecision::Repeat
    }

    fn draw(&mut self, window: &mut tekenen::platform::Platform) {
        let tekenen = &mut self.tek;


        self.tick += 1;


        window.display_pixels(tekenen.get_pixels());
    }
}