use tekenen::math::Vec2;
use tekenen::{colors, DrawableSurface, Surface, SurfaceView};
use tekenen::platform::{Platform, PlatformTrait};

pub struct CanvasDemo {
    tek: SurfaceView,
}

impl CanvasDemo {
    pub fn new() -> Self {
        Self {
            tek: SurfaceView::new(800, 600, Surface::new(800, 600).into()),
        }
    }
}

impl super::Demo for CanvasDemo {
    fn draw(&mut self, window: &mut Platform, _tick: i32) {
        let tekenen = &mut self.tek;
        tekenen.set_translation(Vec2::new(0, 0));
        tekenen.set_scale(1.0);

        tekenen.background(colors::FRITZ_GRAY);

        tekenen.fill_color(colors::WHITE);
        tekenen.circle(0, 0, 10);
        tekenen.fill_color(colors::RED);
        tekenen.rect(10, 10, 40, 40);

        tekenen.fill_color(colors::WHITE);
        tekenen.set_translation(Vec2::new(50, 100));
        tekenen.circle(0, 0, 10);
        tekenen.fill_color(colors::BLUE);
        tekenen.rect(0, 0, 40, 40);
        tekenen.rect(50, 50, 40, 40);


        tekenen.set_translation(Vec2::new(50, 200));
        tekenen.set_scale(2.0);
        tekenen.fill_color(colors::WHITE);
        tekenen.circle(0, 0, 10);
        tekenen.fill_color(colors::BLUE);
        tekenen.rect(0, 0, 40, 40);
        tekenen.rect(50, 50, 40, 40);

        window.display_surface(tekenen.get_surface());
    }
}