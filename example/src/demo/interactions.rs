use std::rc::Rc;

use tekenen::{colors, fui::{div::Div, text::Text, FUI}, platform::{Event, IntervalDecision, KeyDownEvent, Platform, PlatformTrait}, printer::Printer, DrawableSurface, Surface, SurfaceView};

use super::Demo;

pub struct InteractionsDemo {
    tekenen: SurfaceView,
    fui: FUI,
    text: Rc<Text>,
}

impl InteractionsDemo {
    pub fn new() -> Self {
        let text = Text::new("Hello, world!");

        Self {
            tekenen: SurfaceView::new(800, 600, Surface::new(800, 600).into()),
            fui: FUI::new(Div::new(vec![text.clone(), Text::new("Second line?")])),
            text
        }
    }
}

impl Demo for InteractionsDemo {
    fn update(&mut self, event: &Event) -> IntervalDecision {
        self.fui.event(event);

        if let Event::KeyDown(KeyDownEvent { char: Some(key), .. }) = event {
            match key {
                'd' => println!("{:?}", Printer::new(&self.fui)),
                's' => println!("{}", Printer::new(&self.fui)),
                _ => { }
            }
        }

        IntervalDecision::Repeat
    }

    fn draw(&mut self, window: &mut Platform, tick: i32) {
        let ctx = &self.tekenen;

        self.text.set_text(format!("Hello, world! {}", tick));

        ctx.background(colors::FRITZ_GRAY);
        self.fui.render(ctx);

        window.display_surface(ctx.get_surface());
    }
}