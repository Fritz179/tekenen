use std::cell::RefCell;
use std::rc::Rc;

use tekenen::platform::{Event, Platform, IntervalDecision, PlatformTrait};
use tekenen::{Tekenen, colors, Draw};
use tekenen::ui::{Text, Div};

pub struct TextDemo {
    tek: Rc<RefCell<Tekenen>>,
    ui: Box<Div>,
    ticker: Box<Text>
}

impl TextDemo {
    pub fn new() -> Self {
        let tek = Rc::new(RefCell::new(Tekenen::new(800, 600)));

        let ticker = Text::new("Tick: ");
        let ui = Div::new_vertical(vec![Text::new("Hello"), Text::new("There"), ticker]);

        Self {
            tek,
            ui,
            ticker
        }
    }
}

impl super::Demo for TextDemo {
    fn update(&mut self, _event: &Event) -> IntervalDecision {
        IntervalDecision::Repeat
    }

    fn draw(&mut self, window: &mut Platform) {
        let mut tek = self.tek.borrow_mut();

        tek.background(colors::GRAY);


        // tek.ui(&mut Container::vertical(vec![
        //     Text::new("This is a Section!"),
        //     Container::horizontal(vec![
        //         Text::new("Line 1:"),
        //         Text::new("<x>")
        //     ]),//.justify(justify::space_beetwen),
        //     Container::horizontal(vec![
        //             Text::new("Line 1:"),
        //             Text::new("<y>")
        //         ])
        //         ,//.justify(justify::space_beetwen),
        //         // .border(border::new()::bottom(unit::pixels::new(20))),
        //     Text::new("This is anorher Section!"),
        //     Container::horizontal(vec![
        //         Text::new("A:"),
        //         Text::new("<z>")
        //     ]),//.justify(justify::space_beetwen),
        //     Container::horizontal(vec![
        //             Text::new("A very very very very very very long Line:"),
        //             Text::new("<w>")
        //         ])
        //         ,//.justify(justify::space_beetwen)
        // ]));

        self.ui.draw();

        window.display_pixels(tek.get_pixels());
    }
}