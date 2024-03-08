use tekenen::shapes::rect::Rect;
use tekenen::ui::elements::{BlockFormattingContext, Div, DomElement, P};

use tekenen::{colors, Draw, Tekenen};
use tekenen::platform::{PlatformTrait, IntervalDecision, Event, KeyDownEvent};

pub struct FloatDemo {
    tek: Tekenen,
    div: Box<Div>,
}

impl FloatDemo {
    pub fn new() -> Self {
        // let div = Div::new_vertical_flex(vec![Div::new_horizontal_flex(vec![
        //     Div::new_horizontal_flex(vec![
        //         P::new("hello"),
        //         P::new("world"),
        //     ]),
        //     Div::new_vertical_flex(vec![
        //         P::new("I"),
        //         P::new_fn("Am", |el| { 
        //             el.style.padding.set(50.into());
        //             el.style.margin.set(50.into());
        //             el.style.background_color.set(colors::CYAN);
        //          }),
        //         P::new("A"),
        //         P::new_fn("Column", |el| el.style.background_color.set(colors::RED)),
        //         P::new("!"),
        //     ]),
        // ]),
        // P::new("Am I in the correct place? Also I am a very long text and I probably should wrap at some point")
        // ]);

        let div = Div::new(vec![
            P::new("hello"),
            P::new("world!"),
        ]);

        Self {
            tek: Tekenen::new(800, 600),
            div
        }
    }
}

impl super::Demo for FloatDemo {
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

        // 1. Clear canvas
        let tekenen = &mut self.tek;
        tekenen.set_translation(0, 0);
        tekenen.background(colors::FRITZ_GRAY);

        // 2. Generate Layout Box Tree
        let layout = self.div.get_layout_box();

        // 3. Do Layouting and get Paint Tree
        let context = BlockFormattingContext::new();

        let painter = context.get_painter(&layout, Rect::new(0, 0, 800, 600));


        // 4. Draw the Paint Tree
        painter.paint(tekenen);

        // 5. Display the pixels
        window.display_pixels(tekenen.get_pixels());
    }
}