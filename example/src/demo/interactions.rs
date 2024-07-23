use std::rc::Rc;

use tekenen::{colors, fui::{button::Button, div::Div, slider::Slider, text::Text, FUI}, platform::{Event, IntervalDecision, KeyDownEvent, Platform, PlatformTrait}, printer::Printer, DrawableSurface, Surface, SurfaceView};

use super::Demo;

pub struct InteractionsDemo {
    tekenen: SurfaceView,
    fui: FUI,
    text: Rc<Text>,
}

impl InteractionsDemo {
    pub fn new() -> Self {
        let text = Text::new("Hello, world!");

        // Button
        let mut clicks: i32 = 0;
        let button_text = Text::new("Clicks: 0");
        let button_text_clone = button_text.clone();
        let button = Button::new(move || {
            clicks += 1;
            button_text_clone.set_text(format!("Clicks: {}", clicks));
        });

        // Slider
        let slider_text = Text::new("Value: 50%");
        let slider_text_clone = slider_text.clone();
        let slider = Slider::new(move |value| {
            slider_text_clone.set_text(format!("Value: {}%", (value * 100.0) as i32));
        });

        Self {
            tekenen: SurfaceView::new(800, 600, Surface::new(800, 600).into()),
            fui: FUI::new(Div::new(vec![
                text.clone(), 
                Text::new("Second line?"),
                button,
                button_text,
                slider,
                slider_text
            ])),
            text
        }
    }
}

impl Demo for InteractionsDemo {
    fn update(&mut self, event: Event) -> IntervalDecision {
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