use tekenen::shapes::rect::Rect;
use tekenen::{colors, DrawableSurface, Surface, SurfaceView};
use tekenen::platform::{Event, IntervalDecision, Platform, PlatformTrait};


pub struct GraphDemo {
    graph: SurfaceView,
    original: SurfaceView,
}

impl GraphDemo {
    pub fn new() -> Self {
        let original = SurfaceView::new(800, 600, Surface::new(800, 600).into());
        let graph = original.clone();
        graph.clip(Rect::new(100, 0, 700, 500));

        Self {
            graph,
            original
        }
    }
}

fn function(x: i32) -> i32 {
    400 + (f32::sin(x as f32 / 10.0) * 100.0) as i32
}

impl super::Demo for GraphDemo {
    fn update(&mut self, event: &Event) -> IntervalDecision {
        if self.graph.handle_pan_and_zoom(event) {
            return IntervalDecision::Repeat
        }

        IntervalDecision::Repeat
    }

    fn draw(&mut self, window: &mut Platform) {
        self.graph.background(colors::FRITZ_GRAY);

        // Fraw function
        let mut last_y = function(0);

        self.graph.fill_color(colors::WHITE);
        self.graph.circle(0, last_y, 10);

        for x in 1..self.graph.width() {
            let curr = function(x);

            self.graph.line(x - 1, last_y, x, curr);
            last_y = curr;
        }

        self.graph.circle(self.graph.width(), last_y, 10);

        // Rects for reference
        self.graph.fill_color(colors::RED);
        self.graph.rect(0, 0, 10, 10);

        self.graph.fill_color(colors::GREEN);
        self.graph.rect(100, 100, 10, 10);

        // Draw over borders
        self.original.fill_color(colors::BLACK);
        self.original.rect(0, 0, 100, self.original.height());
        self.original.rect(100, self.original.height() - 100, self.original.width() - 100, 100);

        // let rect = self.graph.get_world_screen();
        // println!("World: {}", rect);

        // Diplay result
        window.display_surface(self.graph.get_surface());
    }
}