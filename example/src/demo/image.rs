use tekenen::{colors, DrawableSurface, Surface, SurfaceView};
use tekenen::platform::{Platform, PlatformTrait};


pub struct ImageDemo {
    tek: SurfaceView,
    img8_png: Surface, 
    img8_fpia: Surface,
}

impl ImageDemo {
    pub fn new() -> Self {
        Self {
            tek: SurfaceView::new(800, 600, Surface::new(800, 600).into()),
            img8_png: Platform::parse_image(include_bytes!("../../src/img/8.png")).unwrap(),
            img8_fpia: Platform::parse_image(include_bytes!("../../src/img/8.fpia")).unwrap(),
        }
    }
}

impl super::Demo for ImageDemo {
    fn draw(&mut self, window: &mut Platform, tick: i32) {
        let tekenen = &mut self.tek;

        tekenen.background(colors::FRITZ_GRAY);
        tekenen.reset_transformation();

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

        tekenen.text(&format!("Hello there, tick: {}", tick), 200, 200, 16);

        tekenen.draw_image(600, 200, &self.img8_png);
        tekenen.draw_image_scaled(600, 25, 4.0, &self.img8_fpia);

        window.display_surface(tekenen.get_surface());
    }
}