use tekenen::{colors, Draw, Tekenen};
use tekenen::platform::{PlatformTrait, IntervalDecision, Event, KeyDownEvent};

pub struct CanvasDemo {
    tek: Tekenen,
    tick: i32,
}

impl CanvasDemo {
    pub fn new() -> Self {
        Self {
            tek: Tekenen::new(800, 600),
            tick: 0,
        }
    }
}

impl super::Demo for CanvasDemo {
    fn update(&mut self, event: &Event) -> tekenen::platform::IntervalDecision {
        match event {
            Event::Quit => {
                return IntervalDecision::Stop
            },
            Event::KeyDown(KeyDownEvent { char: Some(char), .. }) => {
                println!("{char}")
            },
            _ => { }
        };

        IntervalDecision::Repeat
    }

    fn draw(&mut self, window: &mut tekenen::platform::Platform) {
        let tekenen = &mut self.tek;
        tekenen.set_translation(0, 0);
        tekenen.set_scale(1.0);


        self.tick += 1;

        tekenen.background(colors::GRAY);

        tekenen.circle(0, 0, 10, colors::WHITE);
        tekenen.rect(10, 10, 40, 40, colors::RED);


        tekenen.set_translation(50, 100);
        tekenen.circle(0, 0, 10, colors::WHITE);
        tekenen.rect(0, 0, 40, 40, colors::BLUE);
        tekenen.rect(50, 50, 40, 40, colors::BLUE);


        tekenen.set_translation(50, 200);
        tekenen.set_scale(2.0);
        tekenen.circle(0, 0, 10, colors::WHITE);
        tekenen.rect(0, 0, 40, 40, colors::BLUE);
        tekenen.rect(50, 50, 40, 40, colors::BLUE);


        window.display_pixels(tekenen.get_pixels());
    }
}