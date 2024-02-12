use std::cell::RefCell;
use std::rc::Rc;

use tekenen::platform::{Event, Platform, IntervalDecision, PlatformTrait};
use tekenen::{Tekenen, colors, Draw, TransforView};
use tekenen::ui::{Container, Text, Element, UIBuilder};

pub struct TextDemo {
    tek: Rc<RefCell<Tekenen>>,
    ui: Box<Container>
}

impl TextDemo {
    pub fn new() -> Self {
        let tek = Rc::new(RefCell::new(Tekenen::new(800, 600)));

        // let ui = UIBuilder::new(Rc::new(RefCell::new(TransforView::new(0, 0, 800, 600, tek.clone()))))
        //     .vertical_container()
        //         .text("Section 1")
        //         .container()
        //             .text("Line 1")
        //             .text("<X>")
        //         .build()
        //     .build()
        // .build();

        let ui = UIBuilder::new(Rc::new(RefCell::new(TransforView::new(0, 0, 800, 600, tek.clone()))))
        .text("Section 1")
        .build();

        Self {
            tek,
            ui
        }
    }
}

impl super::Demo for TextDemo {
    fn update(&mut self, event: &Event) -> IntervalDecision {
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