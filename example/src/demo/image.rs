use tekenen::{Tekenen, colors, Draw, Font};
use tekenen::platform::{Platform, PlatformTrait, IntervalDecision, Event, KeyDownEvent};


pub struct ImageDemo {
    tek: Tekenen,
    // slider: slider::Slider,
    img8_png: Tekenen, 
    img8_fpia: Tekenen,
    tick: i32,
}

impl ImageDemo {
    pub fn new() -> Self {
        Self {
            tek: Tekenen::new(800, 600),
            // slider: slider::Slider::new(300, 500, 50),
            img8_png: Platform::parse_image(include_bytes!("../../src/img/8.png")).unwrap(),
            img8_fpia: Platform::parse_image(include_bytes!("../../src/img/8.fpia")).unwrap(),
            tick: 0,
        }
    }
}

impl super::Demo for ImageDemo {
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

        tekenen.background(colors::FRITZ_GRAY);

        tekenen.rect(50, 100, 100, 150, colors::BLACK);
        tekenen.circle(150, 100, 50, colors::RED);

        tekenen.line(50, 100, 150, 250, colors::WHITE);

        tekenen.line(300, 300, 350, 350, colors::WHITE);
        tekenen.line(350, 300, 400, 300, colors::WHITE);
        tekenen.line(400, 300, 450, 250, colors::WHITE);

        tekenen.line(100, 400, 100, 350, colors::WHITE);
        tekenen.line(150, 400, 150, 550, colors::WHITE);

        tekenen.text(&format!("Hello there, tick: {}", self.tick), 200, 200, Font::new(16, colors::WHITE));

        tekenen.draw_image(600, 200, &self.img8_png);
        tekenen.draw_scaled_image(600, 25, &self.img8_fpia, 5);

        self.tick += 1;

        window.display_pixels(tekenen.get_pixels());
    }
}