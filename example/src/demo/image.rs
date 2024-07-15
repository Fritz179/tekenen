use tekenen::{colors, DrawableSurface, Surface, SurfaceView};
use tekenen::platform::{Platform, PlatformTrait, IntervalDecision, Event, KeyDownEvent};


pub struct ImageDemo {
    tek: SurfaceView,
    // slider: slider::Slider,
    img8_png: Surface, 
    img8_fpia: Surface,
    tick: i32,
}

impl ImageDemo {
    pub fn new() -> Self {
        Self {
            tek: SurfaceView::new(800, 600, Surface::new(800, 600).into()),
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

        tekenen.fill_color(colors::BLACK);
        tekenen.rect(50, 100, 100, 150);
        tekenen.fill_color(colors::RED);
        tekenen.circle(150, 100, 50);

        tekenen.fill_color(colors::WHITE);
        tekenen.line(50, 100, 150, 250);

        tekenen.line(300, 300, 350, 350);
        tekenen.line(350, 300, 400, 300);
        tekenen.line(400, 300, 450, 250);

        tekenen.line(100, 400, 100, 350);
        tekenen.line(150, 400, 150, 550);

        tekenen.text(&format!("Hello there, tick: {}", self.tick), 200, 200, 16);

        // tekenen.draw_image(600, 200, &self.img8_png);
        // tekenen.draw_scaled_image(600, 25, &self.img8_fpia, 5);

        self.tick += 1;

        window.display_surface(tekenen.get_surface());
    }
}