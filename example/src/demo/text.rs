use std::cell::RefCell;
use std::rc::Rc;

use tekenen::math::Vec2;
use tekenen::ui::elements::{Div, Text, Element};
use tekenen::{colors, Draw, Tekenen};
use tekenen::platform::{PlatformTrait, IntervalDecision, Event, KeyDownEvent};

pub struct TextDemo {
    tek: Tekenen,
    tick: i32,
    text: Rc<RefCell<Text>>,
    div: Rc<RefCell<Div>>,
}

impl TextDemo {
    pub fn new() -> Self {
        let text = Text::new("salve");
        let clone = Rc::clone(&text);
        let div = Div::new_vertical(vec![Div::new_horizontal(vec![
            clone,
            Div::new_horizontal(vec![
                Text::new("hello"),
                Text::new("world"),
            ]),
            Div::new_vertical(vec![
                Text::new("I"),
                Text::new("Am"),
                Text::new("A"),
                Text::new("Column"),
                Text::new("!"),
            ]),
        ]),
        Text::new("Am I in the correct place?")
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


        self.text.borrow_mut().text = format!("Tick: {tick}", tick = self.tick);

        tekenen.set_translation(0, 0);
        self.div.borrow_mut().draw(tekenen, Vec2::new(tekenen.width(), tekenen.height()));

        self.tick += 1;
        window.display_pixels(tekenen.get_pixels());
    }
}