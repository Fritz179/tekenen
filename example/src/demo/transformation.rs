use tekenen::{colors, math::Vec2, platform::{Platform, PlatformTrait}, shapes::rect::Rect, DrawableSurface, Surface, SurfaceView};

use super::Demo;

pub struct TransformationDemo {
    screen: SurfaceView
}

impl TransformationDemo {
    pub fn new() -> Self {
        Self {
            screen: SurfaceView::new(800, 600, Surface::new(800, 600).into())
        }
    }
}

const SPACE: i32 = 150;

impl Demo for TransformationDemo {
    fn draw(&mut self, window: &mut Platform, _tick: i32) {
        let ctx = &self.screen;

        ctx.reset_transformation();
        ctx.reset_clip();
        ctx.background(colors::FRITZ_GRAY);
        ctx.fill_color(colors::WHITE);

        // Draw the grid
        for i in 0..5 {
            ctx.line(SPACE * i, 0, SPACE * i, 600);
            ctx.line(0, SPACE * i, 600, SPACE * i);
        }

        for x in 0..4 {
            for y in 0..4 {
                ctx.clip(Rect::new(x * SPACE, y * SPACE, SPACE, SPACE));
                ctx.reset_transformation();
                ctx.fill_color(colors::WHITE);

                // Top left cell
                if x == 0 && y == 0 {
                    ctx.line(0, 0, SPACE, SPACE);
                    ctx.text("FIRST", 50, 20, 16);
                    ctx.text("SECOND", 20, SPACE - 20, 16);
                    continue;
                }

                // Info cells
                if x == 0 || y == 0 {
                    let i = if x == 0 { y } else { x };

                    match i {
                        1 => { ctx.text("NONE", 20, SPACE / 2 - 8, 8); },
                        2 => { ctx.text("SCALE 2", 20, SPACE / 2 - 8, 8); },
                        3 => { ctx.text("TRAN 20", 20, SPACE / 2 - 8, 8); },
                        _ => unreachable!()
                    }

                    continue;
                }

                // Draw the origin
                ctx.clip(Rect::new(x * SPACE + 10, y * SPACE + 10, SPACE - 10, SPACE - 10));
                ctx.fill_color(colors::GREEN);
                ctx.line(-10, 0, 90, 0);
                ctx.line(0, -10, 0, 90);

                // Apply transformations
                for i in 0..=1 {
                    let i = if i == 0 { x } else { y };

                    match i {
                        1 => { },
                        2 => { ctx.scale(2.0); },
                        3 => { ctx.translate(Vec2::new(20, 0)); },
                        _ => unreachable!()
                    }
                }

                // Draw the shape
                ctx.fill_color(colors::RED);
                ctx.circle(0, 0, 5);
                ctx.rect(10, 10, 10, 10);
            }
        }

        window.display_surface(self.screen.get_surface());
    }
}