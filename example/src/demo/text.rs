use tekenen::ui::{DivElement, ElementBox, TextElement};
use tekenen::{colors, Draw, Tekenen};
use tekenen::platform::{PlatformTrait, IntervalDecision, Event, KeyDownEvent};

pub struct TextDemo {
    tek: Tekenen,
    tick: i32,
    text: TextElement,
    div: DivElement,
}

impl TextDemo {
    pub fn new() -> Self {
        let text = TextElement::new("salve");
        let clone = text.rc_clone();
        let div = DivElement::new_vertical(vec![
            clone.into(),
            TextElement::new("hello").into(),
            TextElement::new("world").into(),
        ]);

        Self {
            tek: Tekenen::new(800, 600),
            tick: 0,
            text,
            div
        }
    }
}

impl super::Demo for TextDemo {
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
        tekenen.background(colors::GRAY);

        let mut text = self.text.get();

        text.text = format!("Tick: {tick}", tick = self.tick);

        self.div.draw(tekenen);

        self.tick += 1;
        window.display_pixels(tekenen.get_pixels());
    }
}