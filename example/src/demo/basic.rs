use tekenen::{Tekenen, colors, ui::*};
use tekenen::platform::{Platform, PlatformTrait, IntervalDecision, Event, KeyDownEvent};

pub struct BasicDemo {
    tek: Tekenen,
    slider: widgets::Slider,
    img8_png: Tekenen, 
    img8_fpia: Tekenen,
    tick: i32,
}

impl BasicDemo {
    pub fn new() -> Self {
        Self {
            tek: Tekenen::new(800, 600),
            slider: widgets::Slider::new(300, 500, 50),
            img8_png: Platform::load_image("8.png").unwrap(),
            img8_fpia: Platform::load_image("8.fpia").unwrap(),
            tick: 0,
        }
    }
}

impl super::Demo for BasicDemo {
    fn update(&mut self, event: Event) -> tekenen::platform::IntervalDecision {
        match event {
            Event::Quit => {
                return IntervalDecision::Stop
            },
            Event::KeyDown(KeyDownEvent { char: Some(char), .. }) => {
                println!("{char}")
            },
            Event::MouseDown { x, y } => {
                self.slider.mouse_down(x, y);
            },
            Event::MouseMove { x, y } => {
                self.slider.mouse_move(x, y);
            },
            Event::MouseUp { x, y } => {
                self.slider.mouse_up(x, y);
            },
            _ => { }
        };

        IntervalDecision::Repeat
    }

    fn draw(&mut self, window: &mut tekenen::platform::Platform) {
        let tekenen = &mut self.tek;

        tekenen.background(colors::GRAY);

        tekenen.rect(50, 100, 100, 150, colors::BLACK);
        tekenen.circle(150, 100, 50, colors::RED);

        tekenen.line(50, 100, 150, 250, colors::WHITE);

        tekenen.line(300, 300, 350, 350, colors::WHITE);
        tekenen.line(350, 300, 400, 300, colors::WHITE);
        tekenen.line(400, 300, 450, 250, colors::WHITE);

        tekenen.line(100, 400, 100, 350, colors::WHITE);
        tekenen.line(150, 400, 150, 550, colors::WHITE);

        tekenen.draw_text(&format!("Hello there, tick: {}", self.tick), 200, 200);

        tekenen.draw_image(600, 200, &self.img8_png);
        tekenen.draw_scaled_image(600, 25, &self.img8_fpia, 5);

        tekenen.ui(Container::horiziontal(vec![
            Container::new(|_b, _tekenen| {
                // tekenen.rect(b.x, b.y, b.w, b.h, colors::RED)
            }),
            Container::vertical(vec![
                Container::new(|_b, _tekenen| {
                    // tekenen.rect(b.x, b.y, b.w, b.h, colors::WHITE)
                }),
                Container::new(|b, tekenen| {
                    tekenen.rect(b.x, b.y, b.w, b.h, colors::WHITE)
                }),
            ])
        ]));

        self.tick += 1;

        // Draw slider
        self.slider.display(tekenen);
        tekenen.draw_text(&format!("Value: {}", self.slider.value), 300, 75);

        window.display_pixels(tekenen.get_pixels());
    }
}