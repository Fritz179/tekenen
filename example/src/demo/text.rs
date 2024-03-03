use std::cell::{Ref, RefCell};

use std::rc::Rc;


use tekenen::shapes::rect::Rect;
use tekenen::ui::elements::{Div, Text, Element};
use tekenen::ui::style::Context;
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
        let div = Div::new_vertical_flex(vec![Div::new_horizontal_flex(vec![
            clone,
            Div::new_horizontal_flex(vec![
                Text::new("hello"),
                Text::new("world"),
            ]),
            Div::new_vertical_flex(vec![
                Text::new("I"),
                Text::new_fn("Am", |el| { 
                    el.style.padding.set(50.into());
                    el.style.margin.set(50.into());
                    el.style.background_color.set(colors::CYAN);
                 }),
                Text::new("A"),
                Text::new_fn("Column", |el| el.style.background_color.set(colors::RED)),
                Text::new("!"),
            ]),
        ]),
        Text::new("Am I in the correct place? Also I am a very long text and I probably should wrap at some point")
        ]);

        // let div = Div::new_vertical_flex(vec![
        //     Div::new_horizontal_flex(vec![
        //         clone,
        //         Div::new_horizontal_flex(vec![
        //             Text::new("hello"),
        //             Text::new("world"),
        //         ]),
        //     ])
        // ]);

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
        println!();
        println!("Drawing");

        // Clear canvas
        let tekenen = &mut self.tek;
        tekenen.set_translation(0, 0);
        tekenen.background(colors::FRITZ_GRAY);

        // Update tick counter
        self.text.borrow_mut().text = format!("Tick: {tick}", tick = self.tick);
        self.tick += 1;

        // Get the Paint Tree
        let context = Context {
            containing_block: Rect::new(0, 0, 800, 600)
        };
        let div: Ref<'_, dyn Element> = self.div.borrow();
        let painter = div.get_style().get_painter(Rc::clone(&self.div) as Rc<RefCell<dyn Element>>, Rect::new(0, 0, 800, 600), context);

        // Draw the Paint Tree
        painter.paint(tekenen);

        // Display the pixels
        window.display_pixels(tekenen.get_pixels());
    }
}