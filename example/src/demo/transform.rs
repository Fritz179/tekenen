use std::cell::RefCell;
use std::rc::Rc;

use tekenen::platform::{Event, Platform, IntervalDecision, PlatformTrait};
use tekenen::{Tekenen, colors, TransforView, Draw};

pub struct TransfromDemo {
    tek: Rc<RefCell<Tekenen>>,
    tv: TransforView,
    interactable: TransforView,
}

impl TransfromDemo {
    pub fn new() -> Self {
        let tek = Rc::new(RefCell::new(Tekenen::new(800, 600)));

        let tv = TransforView::new(400, 0, 400, 300, Rc::clone(&tek) as Rc<RefCell<dyn Draw>>);
        let interactable = TransforView::new(0, 300, 800, 300, Rc::clone(&tek) as Rc<RefCell<dyn Draw>>);

        Self {
            tv,
            tek,
            interactable
        }
    }
}

impl super::Demo for TransfromDemo {
    fn update(&mut self, event: Event) -> IntervalDecision {
        self.interactable.handle_pan_and_zoom(event);

        IntervalDecision::Repeat
    }

    fn draw(&mut self, window: &mut Platform) {

        // Draw all silver
        self.tek.borrow_mut().background(colors::SILVER);
        let tv = &mut self.tv;
        tv.reset();

        // Top right (GRAY)
        tv.background(colors::GRAY);

        // Simple red Rect
        self.tek.borrow_mut().rect_raw(50, 50, 50, 50, colors::RED);
        tv.rect_raw(50, 50, 50, 50, colors::BLUE);

        // Double the rect
        self.tek.borrow_mut().rect_raw(50, 150, 50 * 2, 50 * 2, colors::RED);
        tv.scale(2.0);
        tv.rect_raw(50 / 2, 150 / 2, 50, 50, colors::BLUE);

        // draw interactable part
        let tv = &mut self.interactable;

        tv.background(colors::WHITE);

        tv.rect_raw(50, 50, 50, 50, colors::GREEN);
        tv.circle_raw(150, 150, 50, colors::BLACK);

        window.display_pixels(self.tek.borrow().get_pixels());
    }
}

// tv has 
//  - screen_size, screen_position
//  - word_position, word_size = zoom * screen_size