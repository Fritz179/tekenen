use tekenen::math::Vec2;
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
        let div = DivElement::new_vertical(vec![DivElement::new_horizontal(vec![
            clone.into(),
            DivElement::new_horizontal(vec![
                TextElement::new("hello").into(),
                TextElement::new("world").into(),
            ]).into(),
            DivElement::new_vertical(vec![
                TextElement::new("I").into(),
                TextElement::new("Am").into(),
                TextElement::new("A").into(),
                TextElement::new("Column").into(),
                TextElement::new("!").into(),
            ]).into(),
        ]).into(),
        TextElement::new("Am I in the correct place?").into()
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


        self.text.get().text = format!("Tick: {tick}", tick = self.tick);

        tekenen.set_translation(0, 0);
        self.div.draw(tekenen, Vec2::new(tekenen.width(), tekenen.height()));

        self.tick += 1;
        window.display_pixels(tekenen.get_pixels());
    }
}